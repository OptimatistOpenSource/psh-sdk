use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about, long_about = None, version)]
pub struct Args {
    #[arg(verbatim_doc_comment)]
    /// Path to your strategy project
    ///   Example: sc -p strategy-template -o strategy
    #[arg(long)]
    #[clap(short = 'p')]
    #[arg(value_name = "PATH")]
    pub project_dir: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Path to your strategy.wasm
    ///   Example: sc -w strategy.wasm -o strategy
    #[arg(long)]
    #[clap(short = 'w')]
    #[arg(value_name = "PATH")]
    pub wasm_path: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Where you want to place your compilation result
    ///   Example: sc -p strategy-template -o strategy
    #[arg(long)]
    #[clap(short = 'o')]
    #[arg(default_value = "strategy")]
    #[arg(value_name = "PATH")]
    pub output_path: Option<String>,
}
