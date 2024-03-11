//! IMAP Context

pub enum ContextError {}

#[derive(Debug, Default)]
pub struct IMAP4rev2Context {
    pub rfc_state: crate::state::IMAP4rev2State,
}

impl IMAP4rev2Context {
    pub fn new() -> Self {
        IMAP4rev2Context::default()
    }
    pub fn try_next_response(self: &mut Self, input: &str) -> Result<(), ContextError> {
        crate::request_response::Response::scan_with_context(self, input);
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::IMAP4rev2Context;
    use crate::state::IMAP4rev2State;

    #[test]
    fn defaults() {
        let s = IMAP4rev2Context::new();
        assert_eq!(s.rfc_state, IMAP4rev2State::NotAuthenticated);
    }
}
