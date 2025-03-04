use clap::Parser;
use clap_derive::Parser;
use genai::chat::{ChatMessage, ChatRequest};
use genai::resolver::AuthData;
use genai::Client;
use std::borrow::Cow;
use std::fs::{read_to_string, write};
use std::path::PathBuf;

/// A command line interface for modif[ai]-resume allowing users with a LaTeX
/// resume to easily modify their resumes based on job descriptions and get a
/// resume that's honest and tailored for each job.
#[derive(Parser)]
pub struct Cli {
    /// The path to a text file containing the resume. This needs to be a LaTeX
    /// file since the tool currently only understand how to work with LaTeX
    /// resumes.
    ///
    /// If the resume if not written in LaTeX then some unexpected behavior can
    /// take place.
    ///
    /// The tool currently operates with the assumption that the resume is made
    /// up of just a single file or that the primary contents of the resume are
    /// in a single file.
    #[clap(short, long)]
    input: PathBuf,

    /// The path of the output file that will be created as a result of updating
    /// the resume.
    #[clap(short, long)]
    output: PathBuf,

    /// The Gemini API key.
    #[clap(short, long)]
    api_key: String,

    /// The url of the job description.
    ///
    /// The CLI will attempt to get the job description from the specified URL.
    /// If it fails, then an error will be thrown.
    #[clap(short, long)]
    job_description_url: String,

    /// The path to a file to use as the template for the prompt to the AI to
    /// update the template.
    #[clap(short, long)]
    template: Option<PathBuf>,

    /// The model to use in updating the resume.
    #[clap(short, long, default_value = "gemini-2.0-pro-exp-02-05")]
    model: String,
}

impl Cli {
    /// Resolves update the resume template from the CLI arguments if it can
    /// be resolved.
    pub fn resolve_template(&self) -> Result<Cow<'static, str>, Error> {
        match self.template {
            Some(ref template) => read_to_string(template.as_path())
                .map_err(Error::FailedToReadTemplate)
                .map(Cow::Owned),
            None => Ok(Cow::Borrowed(include_str!(
                "../assets/update-resume-template.md"
            ))),
        }
    }

    pub fn resume(&self) -> Result<String, Error> {
        read_to_string(self.input.as_path()).map_err(Error::FailedToReadTemplate)
    }

    pub fn gemini_client(&self) -> Client {
        let key = self.api_key.clone();
        Client::builder()
            .with_auth_resolver_fn(move |_| Ok(Some(AuthData::from_single(key))))
            .build()
    }

    pub fn model(&self) -> String {
        self.model.clone()
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let client = cli.gemini_client();
    let model = cli.model();

    // Getting the content of the job description page.
    let job_description_html = reqwest::get(&cli.job_description_url)
        .await
        .map_err(Error::FailedToFetchJobDescriptionPage)?
        .text()
        .await
        .map_err(Error::FailedToFetchJobDescriptionPage)?;

    // Getting the filled template as the prompt.
    let prompt = {
        let resume = cli.resume()?;
        let prompt = cli
            .resolve_template()?
            .replace("{{job_description_html}}!", &job_description_html)
            .replace("{{resume}}!", &resume);
        ChatRequest::new(vec![
            ChatMessage::system(
                "Very strictly follow the rules provided by the user in their request",
            ),
            ChatMessage::user(prompt),
        ])
    };

    let response = client
        .exec_chat(model.as_str(), prompt, None)
        .await
        .map_err(Error::GeminiError)?
        .content_text_into_string()
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed(""));
    let response_len = response.len();
    let cleaned_up_response = response_len
        .checked_sub(3)
        .and_then(|end| response.get(9..end))
        .unwrap_or("");

    write(cli.output, cleaned_up_response).map_err(Error::FailedToWriteOutput)?;

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    FailedToReadTemplate(std::io::Error),
    FailedToReadResume(std::io::Error),
    FailedToWriteOutput(std::io::Error),
    FailedToFetchJobDescriptionPage(reqwest::Error),
    GeminiError(genai::Error),
}
