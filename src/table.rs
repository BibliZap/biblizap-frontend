use std::ops::Deref;

use yew::prelude::*;
use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;
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
    pub score: Option<i32>
}

#[derive(Clone, PartialEq)]
pub enum TableStatus {
    NotRequested,
    Requested,
    RequestError(String),
    Available(Vec<Article>)
}

#[derive(Clone, PartialEq, Properties)]
pub struct TableContainerProps {
    pub table_status: UseStateHandle<TableStatus>,
    pub update_blacklist: Callback<(String, bool)>
}
#[function_component(TableContainer)]
pub fn table_container(props: &TableContainerProps) -> Html  {
    let content = match props.table_status.deref() {
        TableStatus::NotRequested => { html! { } }
        TableStatus::Available(articles) => { html! {<Table articles={articles.to_owned()} update_blacklist={props.update_blacklist.clone()}/>} }
        TableStatus::Requested => { html! {<Spinner/>} }
        TableStatus::RequestError(msg) =>  { html! {<Error msg={msg.to_owned()}/>} }
    };

    html! {
        <div class="container-fluid" style="margin-top: 50px;">
            {content}
        </div>
    }
}

#[function_component(Spinner)]
pub fn spinner() -> Html {
    html! {
        <div class="d-flex justify-content-center">
            <div class="spinner-border" role="status" style="width: 5rem; height: 5rem;">
                <span class="visually-hidden">{"Loading..."}</span>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TableProps {
    articles: Vec<Article>,
    update_blacklist: Callback<(String, bool)>
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let articles = props.articles.to_owned();
    datatable_create("#example");
    
    html! {
        <div>
            <table id="example" class="table" style="width:100%">
                <thead>
                    <tr>
                        <th>{"Doi"}</th>
                        <th>{"Title"}</th>
                        <th>{"First author"}</th>
                        <th>{"Abstract"}</th>
                        <th>{"Year published"}</th>
                        <th>{"Score"}</th>
                        <th>{"Blacklist"}</th>
                    </tr>
                </thead>
                <tbody>
                    { articles.into_iter().map(|article| html!{<Row article={article} update_blacklist={props.update_blacklist.clone()}/>} ).collect::<Html>() }
                </tbody>
            </table>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct RowProps {
    article: Article,
    update_blacklist: Callback<(String, bool)>
}
#[function_component(Row)]
pub fn row(props: &RowProps) -> Html {
    fn doi_link(doi: Option<String>) -> Option<String> {
        let doi = doi?;
        Some(format!("https://doi.org/{}", doi))
    }

    let onchange = {
        let update_blacklist = props.update_blacklist.clone();
        let doi = props.article.doi.clone();
        Callback::from(move |event: Event| {
            let update_blacklist = update_blacklist.clone();
            let doi = doi.clone();
            let checked = event.target_unchecked_into::<web_sys::HtmlInputElement>().checked();
            if let Some(doi) = doi { update_blacklist.emit((doi, checked)) }
        })
    };

    html! {
        <tr>
            <td><a href={doi_link(props.article.doi.clone())}>{props.article.doi.clone().unwrap_or_default()}</a></td>
            <td>{props.article.title.clone().unwrap_or_default()}</td>
            <td>{props.article.first_author.clone().unwrap_or_default()}</td>
            <td>{props.article.summary.clone().unwrap_or_default()}</td>
            <td>{props.article.year_published.unwrap_or_default()}</td>
            <td>{props.article.score.unwrap_or_default()}</td>
            <td><input type={"checkbox"} class={"row-checkbox"} onchange={onchange}/></td>
        </tr>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ErrorProps {
    msg: AttrValue
}

#[function_component(Error)]
pub fn error(props: &ErrorProps) -> Html {
    html! {
        <div class="alert alert-danger" role="alert">
            {props.msg.to_owned()}
        </div>
    }
}