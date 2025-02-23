use std::fmt::Display;

use super::{Error, Root};

impl Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string =
            serde_json::to_string_pretty(&self).expect("Serializing Root should never fail");
        f.write_str(&string)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string =
            serde_json::to_string_pretty(&self).expect("Serializing Error should never fail");
        f.write_str(&string)
    }
}
