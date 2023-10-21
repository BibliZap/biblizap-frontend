use web_sys::HtmlElement;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::common;
use crate::table::Article;

pub fn to_csv(articles: &[Article]) -> Result<Vec<u8>, common::Error> {
    let mut wtr = csv::Writer::from_writer(Vec::new());

    for article in articles.iter() {
        wtr.serialize(article)?;
    }

    wtr.flush()?;

    match wtr.into_inner() {
        Ok(vec) => Ok(vec),
        Err(error) => Err(common::Error::CsvIntoInner(error.to_string()))
    }
}

pub fn download_bytes_as_file(bytes: &[u8], filename: &str) -> Result<(), common::Error> {
    use gloo_utils::document;
    let blob= gloo_file::Blob::new(bytes);
    let download_url = web_sys::Url::create_object_url_with_blob(&blob.into()).unwrap();

    let a: HtmlElement = document().create_element("a").unwrap().dyn_into().unwrap();
    
    match a.set_attribute("href", &download_url) {
        Ok(_) => Ok(()),
        Err(error) => Err(common::Error::JsValue(error.as_string().unwrap_or_default()))
    }?;
    match a.set_attribute("download", filename) {
        Ok(_) => Ok(()),
        Err(error) => Err(common::Error::JsValue(error.as_string().unwrap_or_default()))
    }?;

    a.click();

    match document().remove_child(&a) {
        Ok(_) => Ok(()),
        Err(error) => Err(common::Error::JsValue(error.as_string().unwrap_or_default()))
    }?;

    Ok(())
}


#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub onclick: Callback<MouseEvent>
}

#[function_component(DownloadButton)]
pub fn download_button(props: &ButtonProps) -> Html {
    html! {
        <div>
            <button class="btn btn-outline-secondary btn-lg mb-10" onclick={props.onclick.clone()}><i class="bi bi-download me-2"></i>{"Download articles"}</button>
        </div>
    }
}