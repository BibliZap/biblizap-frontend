use web_sys::{HtmlElement, Blob};
use wasm_bindgen::prelude::*;


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

    let uint8arr = js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(bytes) }.into()); 
    let array = js_sys::Array::new();
    array.push(&uint8arr.buffer());

    let blob = Blob::new_with_u8_array_sequence_and_options(
                &array,
                web_sys::BlobPropertyBag::new().type_("text/csv"),
            ).unwrap();
    let download_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

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