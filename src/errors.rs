use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct HealthcheckError {
    details: String
}

impl HealthcheckError {
    pub(crate) fn new(msg: &str) -> HealthcheckError {
        HealthcheckError {details: msg.to_string()}
    }
}

impl fmt::Display for HealthcheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for HealthcheckError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn raises_my_error(yes: bool) -> Result<(), HealthcheckError> {
    if yes {
        Err(HealthcheckError::new("borked"))
    } else {
        Ok(())
    }
}