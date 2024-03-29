//! Context feeds & tracks the rfc-legal state changes

use crate::IMAP4rev2State;
use crate::IMAP4rev2StateIllegalSwitch;

/// IMAP4rev2 Protocol Context
#[derive(Debug, Default, PartialEq)]
pub struct IMAP4rev2Context {
    pub(crate) rfc_state: IMAP4rev2State,
}

impl<'a> IMAP4rev2Context {
    /// New starting from RFC default state [`IMAP4rev2State::NotAuthenticated`]
    pub fn new() -> Self {
        IMAP4rev2Context::default()
    }
    /// New starting from [`IMAP4rev2State::Authenticated`] state
    #[cfg(feature = "imap4rev2_state_preauthenticated")]
    pub fn from_authenticated() -> Self {
        IMAP4rev2Context {
            rfc_state: IMAP4rev2State::Authenticated,
        }
    }
    /// Try state switch into [`IMAP4rev2State::Authenticated`]
    /// Note: Selected and Idle states are originally Authenticated states
    pub fn try_into_authenticated(
        self: &mut Self,
    ) -> Result<&mut Self, IMAP4rev2StateIllegalSwitch> {
        match self.rfc_state {
            IMAP4rev2State::Authenticated => {
                Err(IMAP4rev2StateIllegalSwitch::AuthenticatedFromAuthenticated)
            }
            IMAP4rev2State::Logout => Err(IMAP4rev2StateIllegalSwitch::AuthenticatedFromLogout),
            IMAP4rev2State::NotAuthenticated | IMAP4rev2State::Selected | IMAP4rev2State::Idle => {
                self.rfc_state = IMAP4rev2State::Authenticated;
                Ok(self)
            }
        }
    }
    /// New starting from [`IMAP4rev2State::Selected`] state
    #[cfg(feature = "imap4rev2_state_preselected")]
    pub fn from_selected() -> Self {
        IMAP4rev2Context {
            rfc_state: IMAP4rev2State::Selected,
        }
    }
    /// Try state switch into [`IMAP4rev2State::Selected`]
    pub fn try_into_selected(self: &mut Self) -> Result<&mut Self, IMAP4rev2StateIllegalSwitch> {
        match self.rfc_state {
            IMAP4rev2State::Selected => Err(IMAP4rev2StateIllegalSwitch::SelectedFromSelected),
            IMAP4rev2State::Logout => Err(IMAP4rev2StateIllegalSwitch::SelectedFromLogout),
            IMAP4rev2State::Idle => Err(IMAP4rev2StateIllegalSwitch::SelectedFromIdle),
            IMAP4rev2State::NotAuthenticated => {
                Err(IMAP4rev2StateIllegalSwitch::SelectedFromNotAuthenticated)
            }
            IMAP4rev2State::Authenticated => {
                self.rfc_state = IMAP4rev2State::Selected;
                Ok(self)
            }
        }
    }
    /// New starting from [`IMAP4rev2State::Logout`] state
    #[cfg(feature = "imap4rev2_state_prelogout")]
    pub fn from_logout() -> Self {
        IMAP4rev2Context {
            rfc_state: IMAP4rev2State::Logout,
        }
    }
    /// Try state switch into [`IMAP4rev2State::Logout`]
    pub fn try_into_logout(self: &mut Self) -> Result<&mut Self, IMAP4rev2StateIllegalSwitch> {
        match self.rfc_state {
            IMAP4rev2State::Logout => Err(IMAP4rev2StateIllegalSwitch::LogoutFromLogout),
            IMAP4rev2State::Idle => Err(IMAP4rev2StateIllegalSwitch::LogoutFromIdle),
            IMAP4rev2State::NotAuthenticated
            | IMAP4rev2State::Authenticated
            | IMAP4rev2State::Selected => {
                self.rfc_state = IMAP4rev2State::Logout;
                Ok(self)
            }
        }
    }
    /// New starting from [`IMAP4rev2State::Idle`] state    
    #[cfg(feature = "imap4rev2_state_preidle")]
    pub fn from_idle() -> Self {
        IMAP4rev2Context {
            rfc_state: IMAP4rev2State::Idle,
        }
    }
    /// Try state switch into [`IMAP4rev2State::Idle`]
    pub fn try_into_idle(self: &mut Self) -> Result<&mut Self, IMAP4rev2StateIllegalSwitch> {
        match self.rfc_state {
            IMAP4rev2State::Logout => Err(IMAP4rev2StateIllegalSwitch::IdleFromLogout),
            IMAP4rev2State::Idle => Err(IMAP4rev2StateIllegalSwitch::IdleFromIdle),
            IMAP4rev2State::NotAuthenticated => {
                Err(IMAP4rev2StateIllegalSwitch::IdleFromNotAuthenticated)
            }
            IMAP4rev2State::Authenticated | IMAP4rev2State::Selected => {
                self.rfc_state = IMAP4rev2State::Idle;
                Ok(self)
            }
        }
    }
    /// Try to extract next Response within the current Context
    pub fn try_next_response(
        self: &mut Self,
        input: &'a [u8],
    ) -> Result<crate::request_response::Response<'a>, crate::request_response::ScanResponseError<'a>>
    {
        crate::request_response::Response::scan_with_context(self, input)
    }
    /// Try to extract next Request within the current Context
    pub fn try_next_request(
        self: &mut Self,
        input: &'a [u8],
    ) -> Result<crate::request_response::Request<'a>, crate::request_response::ScanRequestError<'a>>
    {
        crate::request_response::Request::scan_with_context(self, input)
    }
}

#[cfg(test)]
mod test {

    use crate::IMAP4rev2State;
    use crate::{IMAP4rev2Context, IMAP4rev2StateIllegalSwitch};

    #[test]
    fn rfc9051_defaults() {
        let ctx = IMAP4rev2Context::new();
        assert_eq!(ctx.rfc_state, IMAP4rev2State::NotAuthenticated);
    }
    #[test]
    #[skip] // todo!()
    fn rfc9051_s8_capability_not_authenticated() {
        let mut ctx = IMAP4rev2Context::new();
        let res = ctx.try_next_response("* OK [CAPABILITY STARTTLS AUTH=SCRAM-SHA-256 LOGINDISABLED IMAP4rev2] IMAP4rev2 Service Ready");
    }
    #[test]
    #[skip] // todo!()
    fn rfc9051_s8_capability_authenticated() {
        let mut ctx = IMAP4rev2Context::new();
        let res = ctx.try_next_response("* OK [CAPABILITY STARTTLS AUTH=SCRAM-SHA-256 LOGINDISABLED IMAP4rev2] IMAP4rev2 Service Ready");
    }

    //-------------------------------------------
    // IMAP4Rev2 State switch into Authenticated
    //-------------------------------------------

    #[test]
    fn rfc9051_states_legal_try_into_authenticated_from_not_authenticated() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_authenticated().unwrap();
        assert_eq!(ctx.rfc_state, IMAP4rev2State::Authenticated);
    }

    #[test]
    fn rfc9051_states_illegal_try_into_authenticated_from_authenticated() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_authenticated().unwrap();
        assert_eq!(
            ctx.try_into_authenticated(),
            Err(IMAP4rev2StateIllegalSwitch::AuthenticatedFromAuthenticated)
        );
    }

    #[test]
    fn rfc9051_states_legal_try_into_authenticated_from_selected_substate() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_authenticated().unwrap();
        ctx.try_into_selected().unwrap();
        ctx.try_into_authenticated().unwrap();
        assert_eq!(ctx.rfc_state, IMAP4rev2State::Authenticated);
    }

    #[test]
    fn rfc9051_states_illegal_try_into_authenticated_from_logout() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_authenticated().unwrap();
        ctx.try_into_logout().unwrap();
        assert_eq!(
            ctx.try_into_authenticated(),
            Err(IMAP4rev2StateIllegalSwitch::AuthenticatedFromLogout)
        );
    }

    #[test]
    fn rfc9051_states_legal_try_into_authenticated_from_idle_substate() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_authenticated().unwrap();
        ctx.try_into_idle().unwrap();
        ctx.try_into_authenticated().unwrap();
        assert_eq!(ctx.rfc_state, IMAP4rev2State::Authenticated);
    }

    //-------------------------------------------
    // IMAP4Rev2 State switch into Selected
    //-------------------------------------------

    #[test]
    fn rfc9051_states_illegal_try_into_selected_from_not_authenticated() {
        let mut ctx = IMAP4rev2Context::new();
        assert_eq!(
            ctx.try_into_selected(),
            Err(IMAP4rev2StateIllegalSwitch::SelectedFromNotAuthenticated)
        );
    }

    #[test]
    fn rfc9051_states_legal_try_into_selected_substate_from_authenticated() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_authenticated().unwrap();
        ctx.try_into_selected().unwrap();
        assert_eq!(ctx.rfc_state, IMAP4rev2State::Selected);
    }

    #[test]
    fn rfc9051_states_illegal_try_into_selected_from_selected() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_authenticated().unwrap();
        ctx.try_into_selected().unwrap();
        assert_eq!(
            ctx.try_into_selected(),
            Err(IMAP4rev2StateIllegalSwitch::SelectedFromSelected)
        );
    }

    #[test]
    fn rfc9051_states_illegal_try_into_selected_from_logout() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_logout().unwrap();
        assert_eq!(
            ctx.try_into_selected(),
            Err(IMAP4rev2StateIllegalSwitch::SelectedFromLogout)
        );
    }

    #[test]
    fn rfc9051_states_illegal_try_into_selected_from_idle() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_authenticated().unwrap();
        ctx.try_into_idle().unwrap();
        assert_eq!(
            ctx.try_into_selected(),
            Err(IMAP4rev2StateIllegalSwitch::SelectedFromIdle)
        );
    }

    //-------------------------------------------
    // IMAP4Rev2 State switch into Logout
    //-------------------------------------------

    #[test]
    fn rfc9051_states_legal_try_into_logout_from_not_authenticated() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_logout().unwrap();
        assert_eq!(ctx.rfc_state, IMAP4rev2State::Logout);
    }

    #[test]
    fn rfc9051_states_legal_try_into_logout_from_authenticated() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_authenticated().unwrap();
        ctx.try_into_logout().unwrap();
        assert_eq!(ctx.rfc_state, IMAP4rev2State::Logout);
    }

    #[test]
    fn rfc9051_states_legal_try_into_logout_from_selected() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_authenticated().unwrap();
        ctx.try_into_selected().unwrap();
        ctx.try_into_logout().unwrap();
        assert_eq!(ctx.rfc_state, IMAP4rev2State::Logout);
    }

    #[test]
    fn rfc9051_states_illegal_try_into_logout_from_logout() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_logout().unwrap();
        assert_eq!(
            ctx.try_into_logout(),
            Err(IMAP4rev2StateIllegalSwitch::LogoutFromLogout)
        );
    }

    #[test]
    fn rfc9051_states_illegal_try_into_logout_from_idle() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_authenticated().unwrap();
        ctx.try_into_idle().unwrap();
        assert_eq!(
            ctx.try_into_logout(),
            Err(IMAP4rev2StateIllegalSwitch::LogoutFromIdle)
        );
    }

    //-------------------------------------------
    // IMAP4Rev2 State switch into Idle
    //-------------------------------------------

    #[test]
    fn rfc9051_states_illegal_try_into_idle_from_not_authenticated() {
        let mut ctx = IMAP4rev2Context::new();
        assert_eq!(
            ctx.try_into_idle(),
            Err(IMAP4rev2StateIllegalSwitch::IdleFromNotAuthenticated)
        );
    }

    #[test]
    fn rfc9051_states_legal_try_into_idle_from_authenticated() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_authenticated().unwrap();
        ctx.try_into_idle().unwrap();
        assert_eq!(ctx.rfc_state, IMAP4rev2State::Idle);
    }

    #[test]
    fn rfc9051_states_illegal_try_into_idle_from_selected() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_authenticated().unwrap();
        ctx.try_into_selected().unwrap();
        ctx.try_into_idle().unwrap();
        assert_eq!(ctx.rfc_state, IMAP4rev2State::Idle);
    }

    #[test]
    fn rfc9051_states_illegal_try_into_idle_from_logout() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_authenticated().unwrap();
        ctx.try_into_logout().unwrap();
        assert_eq!(
            ctx.try_into_idle(),
            Err(IMAP4rev2StateIllegalSwitch::IdleFromLogout)
        );
    }

    #[test]
    fn rfc9051_states_illegal_try_into_idle_from_idle() {
        let mut ctx = IMAP4rev2Context::new();
        ctx.try_into_authenticated().unwrap();
        ctx.try_into_idle().unwrap();
        assert_eq!(
            ctx.try_into_idle(),
            Err(IMAP4rev2StateIllegalSwitch::IdleFromIdle)
        );
    }
    //------------------------------------
    // IMAP4Rev2 Context opt-in Features
    //------------------------------------

    #[test]
    #[cfg(feature = "imap4rev2_state_preauthenticated")]
    fn feature_imap4rev2_state_preauthenticated() {
        assert_eq!(
            IMAP4rev2Context::from_authenticated(),
            IMAP4rev2Context {
                rfc_state: IMAP4rev2State::Authenticated
            }
        );
    }

    #[test]
    #[cfg(feature = "imap4rev2_state_preselected")]
    fn feature_imap4rev2_state_preselected() {
        assert_eq!(
            IMAP4rev2Context::from_selected(),
            IMAP4rev2Context {
                rfc_state: IMAP4rev2State::Selected
            }
        );
    }

    #[test]
    #[cfg(feature = "imap4rev2_state_prelogout")]
    fn feature_imap4rev2_state_prelogout() {
        assert_eq!(
            IMAP4rev2Context::from_logout(),
            IMAP4rev2Context {
                rfc_state: IMAP4rev2State::Logout
            }
        );
    }

    #[test]
    #[cfg(feature = "imap4rev2_state_preidle")]
    fn feature_imap4rev2_state_preidle() {
        assert_eq!(
            IMAP4rev2Context::from_idle(),
            IMAP4rev2Context {
                rfc_state: IMAP4rev2State::Idle
            }
        );
    }

    #[test]
    #[cfg(feature = "imap4rev2_state_preall")]
    fn feature_imap4rev2_state_preall() {
        feature_imap4rev2_state_preidle();
        feature_imap4rev2_state_prelogout();
        feature_imap4rev2_state_preselected();
        feature_imap4rev2_state_preauthenticated();
    }
}
