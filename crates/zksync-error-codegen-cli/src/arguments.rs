use structopt::StructOpt;
use std::error::Error;

use clap::Parser;

///
/// Parse a single key-value pair
///
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Debug, Parser)]
#[structopt(
    name = "Generator of the error handling code in ZKsync components.",
    global_settings = &[structopt::clap::AppSettings::ArgRequiredElseHelp],
)]
pub struct Arguments {
    /// Link to the master JSON file.
    #[structopt(long = "root-definitions")]
    pub root: String,

    /// Links to additional JSON file.
    #[structopt(long = "additional-definitions")]
    pub additional_definition_files: Vec<String>,

    /// Selected backend.
    #[structopt(long = "backend", possible_values=&["rust", "markdown-mdbook"])]
    pub backend: zksync_error_codegen::arguments::Backend,

    /// Be verbose and produce debug output.
    #[structopt(long = "verbose", short = "v")]
    pub verbose: bool,

    /// Output directory for the generated files.
    #[structopt(long = "output", default_value = "zksync-error")]
    pub output_directory: String,

    /// Provide a backend-specific argument. Should be repeated for every backend argument.
    #[structopt(long = "backend-arg", short = "a", parse(try_from_str = parse_key_val),)]
    pub backend_args: Vec<(String,String)>,
}
