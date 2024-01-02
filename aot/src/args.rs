use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about, long_about = None, version)]
pub struct Args {
    #[arg(verbatim_doc_comment)]
    /// Path to your profiling project
    ///   Example: paot -p profiling-template -o profiling
    #[arg(long)]
    #[clap(short = 'p')]
    #[arg(value_name = "PATH")]
    pub project_dir: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Path to your profiling.wasm
    ///   Example: paot -w profiling.wasm -o profiling
    #[arg(long)]
    #[clap(short = 'w')]
    #[arg(value_name = "PATH")]
    pub wasm_path: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Where you want to place your compilation result
    ///   Example: paot -p profiling-template -o profiling
    #[arg(long)]
    #[clap(short = 'o')]
    #[arg(default_value = "profiling")]
    #[arg(value_name = "PATH")]
    pub output_path: Option<String>,
}
