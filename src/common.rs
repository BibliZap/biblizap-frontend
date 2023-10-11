use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Json error")]
    JsonError(#[from] serde_json::Error),
    #[error("Request error")]
    RequestError(#[from] gloo_net::Error),
}

#[derive(PartialEq)]
pub enum CurrentPage {
    BibliZapApp,
    HowItWorks,
    Contact,
    LegalInformation
}