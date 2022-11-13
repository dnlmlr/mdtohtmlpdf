use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// The input Markdown file
    pub input: String,

    /// Converter HTML output
    #[arg(short = 'o', long)]
    pub html_out: Option<String>,

    /// Converted PDF output
    #[arg(short = 'p', long)]
    pub pdf_out: Option<String>,
}
