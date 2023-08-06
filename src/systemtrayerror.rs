//! Error management for XML-Proc
use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub struct SystemTrayError {
    pub message: String,
}

impl SystemTrayError {
    pub fn _new(msg: &str) -> SystemTrayError {
        SystemTrayError {
            message: msg.to_string(),
        }
    }
}

impl Error for SystemTrayError {}

impl Display for SystemTrayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod testSystemTrayError {
    use super::*;

    #[test]
    fn functionNew_should_returnNewSystemtrayError() {
        let error = SystemTrayError::_new("An error occured");

        assert_eq!(error.message, "An error occured")
    }

    #[test]
    fn traitDerive_should_returnWelloutput() {
        let error = SystemTrayError::_new("An error occured");

        assert_eq!(
            format!("{:?}", error),
            "SystemTrayError { message: \"An error occured\" }"
        )
    }

    #[test]
    fn traitDisplay_should_returnWelloutput() {
        let error = SystemTrayError::_new("An error occured");

        assert_eq!(format!("{}", error), "An error occured")
    }
}
