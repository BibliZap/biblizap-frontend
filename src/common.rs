use thiserror::Error;
use yew::prelude::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Json error")]
    Json(#[from] serde_json::Error),
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Request error")]
    Request(#[from] gloo_net::Error),
    #[error("Csv error")]
    Csv(#[from] csv::Error),
    #[error("Csv into_inner error")]
    CsvIntoInner(String),
    #[error("JsValue error")]
    JsValue(String)
}

#[derive(PartialEq)]
pub enum CurrentPage {
    BibliZapApp,
    HowItWorks,
    Contact,
    LegalInformation
}

#[derive(Clone, PartialEq, Default, Debug, serde::Serialize)]
pub enum SearchFor {
    References,
    Citations,
    #[default]
    Both
}

pub fn get_value(node_ref: &NodeRef) -> Option<String> {
    Some(node_ref.cast::<web_sys::HtmlInputElement>()?.value())
}

