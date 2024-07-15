use clap::Parser;

#[derive(Parser)]
#[command(arg_required_else_help(true))]
pub struct CLI {
    #[clap(flatten)]
    pub op: MainOperations,
    #[clap(short = 't')]
    pub test: Option<Vec<String>>,
    #[clap(long = "print-header")]
    pub print_header: bool,
    #[clap(long = "no-testing")]
    pub no_testing: bool
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
pub struct MainOperations {
    #[clap(short = 'c', conflicts_with="decompile")]
    pub compile: Option<Vec<String>>,
    #[clap(short = 'd', conflicts_with="compile")]
    pub decompile: Option<Vec<String>>,
}
