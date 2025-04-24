pub mod arguments;
pub mod error;

use clap::Parser;

use arguments::Arguments;

use error::ApplicationError;
use zksync_error_codegen::load_and_generate;

fn main_inner(arguments: Arguments) -> Result<(), ApplicationError> {
    Ok(load_and_generate(arguments.try_into()?)?)
}

fn main() {
    let arguments = Arguments::parse();
    if let Err(error) = main_inner(arguments) {
        eprintln!("{error}")
    }
}
