
use std::fmt::{Debug, Error, Formatter};

pub trait ErrorReport {
    fn generate_report(&self) -> String;
}

impl Debug for ErrorReport {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&self.generate_report())
    }
}