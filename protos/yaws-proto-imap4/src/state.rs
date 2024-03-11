//! Client States per RFC 9051
//! This is different from protocol states e.g. IDLE etc.

/// IMAPrev2 rfc9051 section 3
/// https://datatracker.ietf.org/doc/html/rfc9051#name-state-and-flow-diagram
#[derive(Debug, Default, Eq, PartialEq)]
pub enum IMAP4rev2State {
    /// IMAP4rev2 s.3.1
    #[default]
    NotAuthenticated,
    /// IMAP4rev2 s.3.2
    Authenticated,
    /// IMAP4rev2 s.3.3
    Selected,
    /// IMAP4rev2 s.3.4
    Logout,
    /// IMAP4rev2 s.6.3.13 - extension
    Idle,
}

/// IMAP4rev2 s. 6.1 - Any state by Client
pub mod universal {
    /// Available upon Any state (Universal as worded in rfc)
    #[derive(Debug)]
    pub enum IMAP4rev2Command {
        /// IMAP4rev2 s.6.1.1 - List server capabilities
        /// No Arguments, Returns OK or BAD
        Capability,
        /// IMAP4rev2 s.6.1.2 - No-op No Operation see rfc
        Noop,
        /// IMAP4rev2 s.6.1.3 - Logout
        Logout,
    }
}

/// IMAP4rev2 s. 6.2 - Not authenticated state by Client
pub mod not_authenticated {
    /// Available upon Not Authenticated state
    #[derive(Debug)]
    pub enum IMAP4rev2Command {
        Universal(super::universal::IMAP4rev2Command),
        /// s.6.2.1 - STARTTLS
        /// Not available in CLEARTEXT port
        Starttls,
        /// s.6.2.2 - SASL authentication
        Authenticate,
        /// s.6.2.3 - Cleartext Login Authentication
        /// rfc considers this insecure | use of last resort
        Login,
    }
}

/// IMAP4rev2 s. 6.3 - Authenticated State
pub mod authenticated {
    /// Available upon Authenticated state
    #[derive(Debug)]
    pub enum IMAP4rev2Command {
        Universal(super::universal::IMAP4rev2Command),
        /// s.6.3.1 - an explicit indication supporting particular extensions
        Enable,
        /// s.6.3.2 - selects a mailbox for access
        Select,
        /// s.6.3.3 - selects a mailbox for access but for read-only operation
        Examine,
        /// s.6.3.4 - creates a mailbox with the given name
        Create,
        /// s.6.3.5 - permanently removes the mailbox with the given name
        Delete,
        /// s.6.3.6 - command changes the name of a mailbox
        Rename,
        /// s.6.3.7 - adds the mailbox "active" or "subscribed" status
        Subscribe,
        /// s.6.3.8 - removes the mailbox from "active" or "subscribed" status
        Unsubscribe,
        /// s.6.3.9 - list mailboxes
        List,
        /// s.6.3.10 - see prefix and hierarchy delimiter
        Namespace,
        /// s.6.3.11 - requests the status of the indicated mailbox
        Status,
        /// s.6.3.12 - appens a message to the mailbox
        Append,
        /// s.6.3.13 - ask server to transmit unsolicited realtime updates
        Idle,
    }
}
