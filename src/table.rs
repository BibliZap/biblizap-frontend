use std::ops::Deref;
use std::rc::Rc;

use web_sys::{Blob, HtmlElement};
use yew::prelude::*;
use serde::{Deserialize, Serialize};

use gloo_utils::document;

use wasm_bindgen::prelude::*;

use crate::common;
#[wasm_bindgen(module = "/js/datatable.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn datatable_create(table: &str);
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Article {
    pub first_author: Option<String>,
    pub year_published: Option<i32>,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub doi: Option<String>,
    pub citations: Option<i32>,
    pub score: Option<i32>
}

#[derive(Clone, PartialEq)]
pub enum TableStatus {
    NotRequested,
    Requested,
    RequestError(String),
    Available(Rc<Vec<Article>>)
}

#[derive(Clone, PartialEq, Properties)]
pub struct TableContainerProps {
    pub table_status: UseStateHandle<TableStatus>,
    pub update_selected: Callback<(String, bool)>
}
#[function_component(TableContainer)]
pub fn table_container(props: &TableContainerProps) -> Html  {
    let content = match props.table_status.deref() {
        TableStatus::NotRequested => { html! { } }
        TableStatus::Available(articles) => { html! {<Table articles={articles} update_selected={props.update_selected.clone()}/>} }
        TableStatus::Requested => { html! {<Spinner/>} }
        TableStatus::RequestError(msg) =>  { html! {<Error msg={msg.to_owned()}/>} }
    };

    content
}

#[function_component(Spinner)]
pub fn spinner() -> Html {
    html! {
        <div class="container-fluid">
            <div class="d-flex justify-content-center">
                <div class="spinner-border" role="status" style="width: 5rem; height: 5rem; margin-bottom: 50px;">
                    <span class="visually-hidden">{"Loading..."}</span>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TableProps {
    articles: Rc<Vec<Article>>,
    update_selected: Callback<(String, bool)>
}

fn to_csv(articles: &[Article]) -> Result<Vec<u8>, common::Error> {
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

fn download_bytes_as_file(bytes: &[u8], filename: &str) -> Result<(), common::Error> {
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

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let articles = props.articles.to_owned();
    datatable_create("#example");

    
    let on_download_click = {
        let articles = articles.clone();
        Callback::from(move |_: MouseEvent| {
            let bytes = to_csv(articles.as_ref()).unwrap();

            match download_bytes_as_file(&bytes, "out.csv") {
                Ok(_) => (),
                Err(error) => {gloo_console::log!(format!("{error}"));}
            }
        })
        
    };
    
    html! {
        <div class="container-fluid">
            <table id="example" class="table" style="width:100%">
                <thead>
                    <tr>
                        <th></th>
                        <th>{"Doi"}</th>
                        <th>{"Title"}</th>
                        <th>{"First author"}</th>
                        <th>{"Abstract"}</th>
                        <th>{"Year published"}</th>
                        <th>{"Citations"}</th>
                        <th>{"Score"}</th>
                    </tr>
                </thead>
                <tbody>
                    { articles.deref().iter().map(|article| html!{<Row article={article.clone()} update_selected={props.update_selected.clone()}/>} ).collect::<Html>() }
                </tbody>
            </table>
            <DownloadButton onclick={on_download_click}/>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct RowProps {
    article: Article,
    update_selected: Callback<(String, bool)>
}
#[function_component(Row)]
pub fn row(props: &RowProps) -> Html {
    fn doi_link(doi: Option<String>) -> Option<String> {
        let doi = doi?;
        Some(format!("https://doi.org/{}", doi))
    }

    let onchange = {
        let update_selected = props.update_selected.clone();
        let doi = props.article.doi.clone();
        Callback::from(move |event: Event| {
            let update_selected = update_selected.clone();
            let doi = doi.clone();
            let checked = event.target_unchecked_into::<web_sys::HtmlInputElement>().checked();
            if let Some(doi) = doi { update_selected.emit((doi, checked)) }
        })
    };

    html! {
        <tr>
            <td><input type={"checkbox"} class={"row-checkbox"} onchange={onchange}/></td>
            <td><a href={doi_link(props.article.doi.clone())}>{props.article.doi.clone().unwrap_or_default()}</a></td>
            <td>{props.article.title.clone().unwrap_or_default()}</td>
            <td>{props.article.first_author.clone().unwrap_or_default()}</td>
            <td>{props.article.summary.clone().unwrap_or_default()}</td>
            <td>{props.article.year_published.unwrap_or_default()}</td>
            <td>{props.article.citations.unwrap_or_default()}</td>
            <td>{props.article.score.unwrap_or_default()}</td>
        </tr>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    onclick: Callback<MouseEvent>
}

#[function_component(DownloadButton)]
pub fn download_button(props: &ButtonProps) -> Html {
    html! {
        <div>
            <button class="btn btn-primary mb-10" onclick={props.onclick.clone()}>{"Download articles"}</button>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ErrorProps {
    msg: AttrValue
}

#[function_component(Error)]
pub fn error(props: &ErrorProps) -> Html {
    html! {
        <div class="container-fluid">
            <div class="alert alert-danger" role="alert">
                {props.msg.to_owned()}
            </div>
        </div>
    }
}