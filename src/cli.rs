use clap::Parser;

#[derive(Parser)]
#[command(arg_required_else_help(true))]
pub struct CLI {
    #[clap(short = 'c', conflicts_with="decompile")]
    pub compile: Option<Vec<String>>,
    #[clap(short = 'd', conflicts_with="compile")]
    pub decompile: Option<Vec<String>>,
    #[clap(short = 't')]
    pub test: bool,
}
