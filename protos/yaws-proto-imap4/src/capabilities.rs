//! IMAP4rev2 s.7.2.2 Capability Response

/// IMAP Capabilities are assigned by IANA
/// https://www.iana.org/assignments/imap-capabilities/imap-capabilities.xhtml
#[allow(dead_code)]
pub enum Capabilities {
    Acl,
    AnnotateExperiment1,
    AppendLimit,
    AuthEquals,
    Binary,
    Catenate,
    Children,
    CompressEqualsDeflate,
    CondStore,
    ContextEqualsSearch,
    ContextEqualsSort,
    Convert,
    CreateSpecialUse,
    Enable,
    Esearch,
    Esort,
    Filters,
    I18nLevelEquals1,
    I18nLevelEquals2,
    Id,
    Idle,
    Imap4Rev1,
    Imap4Rev2,
    ImapSieveEqual,
    Language,
    ListExtended,
    ListMyRights,
    ListStatus,
    LiteralPlus,
    LiteralMinus,
    LoginReferrals,
    LoginDisabled,
    MailboxReferrals,
    Metadata,
    MetadataServer,
    Move,
    MultiAppend,
    MultiSearch,
    Namespace,
    Notify,
    ObjectId,
    Partial,
    Preview,
    Qresync,
    Quota,
    QuotaEquals,
    QuotaSet,
    Replace,
    RightsEquals,
    SaslIr,
    SaveDate,
    SearchEqualsFuzzy,
    SearchRes,
    Sort,
    SortEqualsDisplay,
    SpecialUse,
    StartTls,
    StatusEqualsSize,
    Thread,
    UidPLus,
    UnAuthenticate,
    UnSelect,
    UrlPartial,
    UrlAuth,
    UrlAuthEqualsBinary,
    Utf8EqualsAccept,
    Utf8EqualsAll,
    Utf8EqualsAppend,
    Utf8EqualsOnly,
    Utf8EqualsUser,
    Within,
}

use core::str::FromStr;

pub enum CapabilitiesParseError {
}

impl FromStr for Capabilities {
    type Err = CapabilitiesParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ => todo!()
        }
    }
}
