pub mod backend;
pub mod conversion;

use clap::Error as ClapError;
use clap::Parser;

pub use backend::Backend;

///
/// Generates one of the following:
/// - Crate `zksync-error` that contains the description of all failures observable in ZKsync components
/// - Documentation for these errors.
///
#[derive(Debug, Parser)]
#[command(
    version,
    about,
    long_about = "Generator of the error handling code in ZKsync components."
)]
pub struct Arguments {
    /// Link to the master JSON file.
    #[arg(long = "root-definitions")]
    pub root: String,

    /// Links to additional JSON file.
    #[arg(long = "additional-definitions")]
    pub additional_definition_files: Vec<String>,

    /// Selected backend.
    #[arg(short = 'b',
          long = "backend",
          value_parser = clap::value_parser!(Backend))]
    pub backend: Backend,

    /// Be verbose and produce debug output.
    #[arg(long = "verbose", short = 'v')]
    pub verbose: bool,

    /// Output directory for the generated files.
    #[arg(long = "output", default_value = "zksync-error")]
    pub output_directory: String,

    /// Provide a backend-specific argument. Should be repeated for every backend argument.
    #[arg(
        long = "backend-arg",
        short = 'a',
        num_args = 1,
        value_parser(parse_key_val)
    )]
    pub backend_args: Vec<(String, String)>,
}

///
/// Utility function to parse a single key value pair separated by `=`.
/// More precisely, it should match a regular expression` *(.*) *= *(.*) *`, and
/// the two respective groups are returned as owned strings.
///
fn parse_key_val(s: &str) -> Result<(String, String), ClapError> {
    let pos = s.find('=').ok_or_else(|| {
        ClapError::raw(
            clap::error::ErrorKind::ValueValidation,
            "expected `key=value` format",
        )
    })?;
    let left = s[..pos].trim().to_string();
    let right = s[pos + 1..].trim().to_string();
    Ok((left, right))
}

#[cfg(test)]
mod tests {
    use crate::arguments::parse_key_val;

    #[test]
    fn key_value_good() {
        assert_eq!(parse_key_val("x = y").unwrap(), ("x".into(), "y".into()))
    }
}
