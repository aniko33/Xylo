pub type Result<T> = std::result::Result<T, Error>;

pub enum ErrorKind {
    AlreadyExists,
    FileAlreadyExists,
    IsNotADirectory,
    InvalidConfig
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        use ErrorKind::*;
        match *self {
            AlreadyExists => "Project already exists",
            IsNotADirectory => "Is not a directory",
            FileAlreadyExists => "File already exists",
            InvalidConfig => "Invalid config"
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.as_str(), f)
    }
}

pub struct Error {
    kind: ErrorKind,
}
impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Self { kind }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind.as_str())
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.kind.as_str()
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.kind, f)
    }
}
