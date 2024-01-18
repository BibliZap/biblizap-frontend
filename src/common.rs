use thiserror::Error;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Request(#[from] gloo_net::Error),
    #[error(transparent)]
    Csv(#[from] csv::Error),
    #[error(transparent)]
    Xlsx(#[from] rust_xlsxwriter::XlsxError),
    #[error("Csv into_inner error")]
    CsvIntoInner(String),
    #[error("JsValue error")]
    JsValueString(String),
    #[error(transparent)]
    TryFromInt(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("HtmlElement dyn_ref error")]
    HtmlElementDynRef,
    #[error(transparent)]
    NodeRefMissingValue(#[from] NodeRefMissingValue)
}

#[derive(Error, Debug)]
pub enum NodeRefMissingValue {
    #[error("Id list is missing")]
    IdList,
    #[error("Output max size is missing")]
    OutputMaxSize,
    #[error("Depth is missing")]
    Depth,
    #[error("SearchFor is missing")]
    SearchFor
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JsValueString(value.as_string().unwrap_or_default())
    }
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

