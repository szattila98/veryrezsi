use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorMsg {
    reason: String,
}

impl ErrorMsg {
    pub fn new<S: AsRef<str>>(reason: S) -> Self {
        Self {
            reason: reason.as_ref().into(),
        }
    }
}
