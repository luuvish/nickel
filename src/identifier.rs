//! Define the type of an identifier.
use std::fmt;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Ident(pub String);

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Ident(ident) = self;
        write!(f, "{}", ident)
    }
}
