use std::fmt;

/// All currently supported countries
#[derive(Debug, Copy, Clone)]
pub enum Country {
    US,
    CA,
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Country::US => write!(f, "US"),
            Country::CA => write!(f, "CA"),
        }
    }
}
