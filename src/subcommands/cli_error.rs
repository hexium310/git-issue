use std::{error, fmt, num};


#[derive(Debug)]
pub enum CliError {
    NotEnoughCommands,
    Parse(num::ParseIntError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CliError::NotEnoughCommands => write!(f, "Not enough commands."),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for CliError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            CliError::NotEnoughCommands => None,
            CliError::Parse(ref err) => Some(err),
        }
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}
