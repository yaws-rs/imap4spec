#[allow(non_camel_case_types)]
trait IMAP4rev2Request {}

#[allow(non_camel_case_types)]
trait IMAP4rev2Response {}

#[allow(non_camel_case_types)]
pub trait IMAP4rev2Client_Universal {
    fn response_capability(&self) -> ();
    fn response_noop(&self) -> ();
    fn response_logout(&self) -> ();
}

#[allow(non_camel_case_types)]
pub trait IMAP4rev2Client_NotAuthenticated {
    fn response_starttls(&self) -> ();
    fn response_authenticate(&self) -> ();
    fn response_login(&self) -> ();
}

#[allow(non_camel_case_types)]
pub trait IMAP4rev2Client_Authenticated {
    fn response_select(&self) -> ();
    fn response_examine(&self) -> ();
    fn response_create(&self) -> ();
    fn response_delete(&self) -> ();
    fn response_rename(&self) -> ();
    fn response_subscribe(&self) -> ();
    fn response_unsubscribe(&self) -> ();
    fn response_list(&self) -> ();
    fn response_namespace(&self) -> ();
    fn response_status(&self) -> ();
    fn response_append(&self) -> ();
    fn response_idle(&self) -> ();
}

#[allow(non_camel_case_types)]
pub trait IMAP4rev2Client_Idle {
    fn item_idle(&self) -> ();
}
