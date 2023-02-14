use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub kind: ErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    InvalidRootNode,
    MissingNamespaceAttribute,
    MissingNamespace
}

impl Error {
    #[must_use]
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    #[doc(hidden)]
    pub fn __description(&self) -> &str {
        match self.kind {
            ErrorKind::InvalidRootNode => "The root node should be named 'schema'",
            ErrorKind::MissingNamespaceAttribute => "Could not find an xmlns attribute",
            ErrorKind::MissingNamespace => "Could not find an XML namespace",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.__description().fmt(f)
    }
}

macro_rules! raise {
    ($kind:expr) => {
        return Err(crate::error::Error{
            kind: $kind
        })
    };
}
