use clap::Parser;

#[derive(Parser)]
pub struct CLI {
    #[clap(short = 'c', conflicts_with="decompile")]
    pub compile: Option<Vec<String>>,
    #[clap(short = 'd', conflicts_with="compile")]
    pub decompile: Option<Vec<String>>,
    #[clap(short = 't')]
    pub test: bool,
}
