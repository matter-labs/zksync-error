pub mod arguments;

use clap::Parser;

use arguments::Arguments;

use zksync_error_codegen::error::ProgramError;
use zksync_error_codegen::load_and_generate;

fn main_inner(arguments: Arguments) -> Result<(), ProgramError> {
    load_and_generate(arguments.into())
}

fn main() {
    let arguments = Arguments::parse();
    if let Err(error) = main_inner(arguments) {
        eprintln!("{error}")
    }
}
