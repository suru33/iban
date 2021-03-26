use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IBANMetaData {
    pub country: String,
    pub code: String,
    pub sepa: bool,
    pub length: u8,
    pub account_check: bool,
    pub branch: bool,
    pub format: String,
}
