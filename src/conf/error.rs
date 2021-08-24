use super::*;
use std::error::Error;
use self::ConfigError::*;

#[derive(Debug)]
pub enum ConfigError {
	NotFound,
	IoError,
	BadFilePath(PathBuf, &'static str),
	BadEnv(String),
	BadEntry(String, PathBuf),
	BadType(String, &'static str, &'static str, Option<PathBuf>),
	ParseError(String, PathBuf, String, Option<(usize, usize)>),
}

impl Error for ConfigError {
	fn description(&self) -> &str {
		match *self {
			NotFound => "config file was not found",
			IoError => "there was an I/O error while reading the config file",
			BadFilePath(..) => "the config file path is invalid",
			BadEnv(..) => "the environment specified in `ROCKET_ENV` is invalid",
			BadEntry(..) => "an environment specified as `[environment]` is invalid",
			BadType(..) => "a key was specified with a value of the wrong type",
			ParseError(..) => "the config file contains invalid TOML",
		}
	}
}

impl fmt::Display for ConfigError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			NotFound => write!(f, "config file was not found"),
			IoError => write!(f, "I/O error while reading the config file"),
			BadFilePath(ref p, _) => write!(f, "{:?} is not a valid config path", p),
			BadEnv(ref e) => write!(f, "{:?} is not a valid `ROCKET_ENV` val", e),
			BadEntry(ref e, _) => {
				write!(f, "{:?} is not a valid `[environment]` entry", e)
			},
			BadType(ref n, e, a, _) => {
				write!(f, "type mismatch for '{}'. expected {}, found {}", n, e, a)
			},
			ParseError(..) => write!(f, "the config file contains invalid TOML"),
		}
	}
}
