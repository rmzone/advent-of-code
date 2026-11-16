use nom_locate::LocatedSpan;
use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Parsing error in the input")]
    ParseError(nom::Err<nom::error::Error<Box<str>>>),
}

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(err: nom::Err<nom::error::Error<&str>>) -> Self {
        Self::ParseError(err.map_input(|input| input.into()))
    }
}

impl From<nom::Err<nom::error::Error<LocatedSpan<&str>>>> for Error {
    fn from(err: nom::Err<nom::error::Error<LocatedSpan<&str>>>) -> Self {
        Self::ParseError(err.map_input(|input| input.into_fragment().into()))
    }
}
