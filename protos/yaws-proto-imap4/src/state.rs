
/// IMAPrev2 rfc9051 section 3
/// https://datatracker.ietf.org/doc/html/rfc9051#name-state-and-flow-diagram
pub enum IMAP4rev2State {
    /// IMAP4rev2 s.3.1
    #[default]
    NOT_AUTHENTICATED,
    /// IMAP4rev2 s.3.2
    AUTHENTICATED,
    /// IMAP4rev2 s.3.3
    SELECTED,
    /// IMAP4rev2 s.3.4
    LOGOUT
}

/// IMAP4rev2 s. 6.1 - Any state by Client
pub mod universal {
    /// Available upon Any state (Universal as worded in rfc)
    pub enum IMAPrev2Command {
        /// IMAP4rev2 s.6.1.1 - List server capabilities
        /// No Arguments, Returns OK or BAD
        CAPABILITY,
        /// IMAP4rev2 s.6.1.2 - No-op No Operation see rfc
        NOOP,
        /// IMAP4rev2 s.6.1.3 - Logout
        LOGOUT,
    }
}

/// IMAP4rev2 s. 6.2 - Not authenticated state by Client
pub mod not_authenticated {
    /// Available upon Not Authenticated state
    pub enum IMAPrev2Command {
        /// 6.2.1 - STARTTLS
        /// Not available in CLEARTEXT port
        STARTTLS,
        /// 6.2.2 - SASL authentication
        AUTHENTICATE,
        /// 6.2.3 - Cleartext Login Authentication
        /// rfc considers this insecure | use of last resort
        LOGIN,
    }
}

/// IMAP4rev2 s. 6.3 - Authenticated State
pub mod authenticated {
    /// Available upon Authenticated state
    
}
