pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    InvalidRegion(String),
    InvalidPath(std::path::PathBuf),
    ListResponseError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            Error::InvalidRegion(region) => write!(f, "Invalid region: {}", region),
            Error::InvalidPath(path) => write!(f, "Invalid path: {:?}", path),
            Error::ListResponseError(msg) => write!(f, "List response error: {}", msg),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error)
    }
}

impl From<std::path::PathBuf> for Error {
    fn from(path: std::path::PathBuf) -> Self {
        Error::InvalidPath(path)
    }
}
