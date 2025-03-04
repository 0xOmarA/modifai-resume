<div align="center">
    <h1>
        <picture>
            <source media="(prefers-color-scheme: dark)" srcset="./assets/dark.png" width="500">
            <img alt="Text changing depending on mode. Light: 'So light!' Dark: 'So dark!'" src="./assets/light.png" width="500">
        </picture>
    </h1>
    <p>
        <strong>A Rust CLI tool that modifies resumes using AI based on job descriptions to ensure that you're as competitive as you can be for jobs you're applying for.</strong>
    </p>
</div>

## Introduction

> [!CAUTION]
> I'm not responsible in any way for hallucinations in your resume that are the result of the AI. If this does happen, then it's your responsibility to check that. I can't control what the AI does and have given it very strict rules and instructed it to not make up information on the resume or lie.

This is a very simple CLI tool that's used to update $\LaTeX$ based resumes based on a job description to ensure that the resume contains information relevant to the job based on the job description prioritizing the important information at the top.

## Usage

- Obtain a Gemini API key by following the steps here: https://aistudio.google.com/app/apikey.
- Install this CLI tool through:
  ```
  $ cargo install --git https://github.com/0xOmarA/modifai-resume
  ```
- Run the CLI tool on your resume:
  ```bash
  modifai-resume \
      -i main.tex \
      -o main-o.tex \
      -j https://jobs.lever.co/ethereumfoundation/8d9b1823-ba67-4195-a673-e11a85a48d62 \
      -a $GEMINI_API_KEY
  ```

## Help

```
$ modifai-resume --help
A command line interface for modif[ai]-resume allowing users with a LaTeX resume to easily modify their resumes based on job descriptions and get a resume that's honest and tailored for each job

Usage: modifai-resume [OPTIONS] --input <INPUT> --output <OUTPUT> --api-key <API_KEY> --job-description-url <JOB_DESCRIPTION_URL>

Options:
  -i, --input <INPUT>
          The path to a text file containing the resume. This needs to be a LaTeX file since the tool currently only understand how to work with LaTeX resumes.

          If the resume if not written in LaTeX then some unexpected behavior can take place.

          The tool currently operates with the assumption that the resume is made up of just a single file or that the primary contents of the resume are in a single file.

  -o, --output <OUTPUT>
          The path of the output file that will be created as a result of updating the resume

  -a, --api-key <API_KEY>
          The Gemini API key

  -j, --job-description-url <JOB_DESCRIPTION_URL>
          The url of the job description.

          The CLI will attempt to get the job description from the specified URL. If it fails, then an error will be thrown.

  -t, --template <TEMPLATE>
          The path to a file to use as the template for the prompt to the AI to update the template

  -m, --model <MODEL>
          The model to use in updating the resume

          [default: gemini-2.0-pro-exp-02-05]

  -h, --help
          Print help (see a summary with '-h')
```
