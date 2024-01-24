use clap::Parser;

/// Argument Parser
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub struct Args {
    /// Optimizations used in the compiler
    #[arg(long, short)]
    pub optimizations: Option<u8>,

    /// Shows the SquidC version | SquidC |major|.|minor|.|patch|-|details| for |OS| |arch|
    #[arg(long, short = 'V')]
    pub version: bool,

    /// Shows newer versions if detected
    #[arg(long, visible_alias = "cnv")]
    pub check_updates: bool,
}
