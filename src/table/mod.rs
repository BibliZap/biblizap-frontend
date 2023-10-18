use std::{cell::RefCell, ops::DerefMut};
use std::ops::Deref;
use std::rc::Rc;

use yew::prelude::*;

pub mod article;
pub use article::Article;

mod footer;
use footer::TableFooter;

mod download;
use download::*;

#[derive(Clone, PartialEq)]
pub enum TableStatus {
    NotRequested,
    Requested,
    RequestError(String),
    Available(Rc<RefCell<Vec<Article>>>)
}

#[derive(Clone, PartialEq, Properties)]
pub struct TableContainerProps {
    pub table_status: UseStateHandle<TableStatus>,
}
#[function_component(TableContainer)]
pub fn table_container(props: &TableContainerProps) -> Html  {
    let content = match props.table_status.deref() {
        TableStatus::NotRequested => { html! { } }
        TableStatus::Available(articles) => { html! {<Table articles={articles}/>} }
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
    articles: Rc<RefCell<Vec<Article>>>,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    
    let selected_articles = use_mut_ref(Vec::<String>::new);
    let selected_articles = use_state(|| selected_articles);

    let update_selected = {
        let selected_articles = selected_articles.clone();
        Callback::from(move |element : (String, bool)| {
            let rc = selected_articles.deref().to_owned();
            if element.1 {
                rc.deref().borrow_mut().push(element.0); 
            } else {
                rc.deref().borrow_mut().retain(|x| *x != element.0)
            }
            selected_articles.set(rc);
        })
    };

    let articles = props.articles.to_owned();
    let global_search = use_state(|| "".to_string());

    let r = regex::Regex::new(&global_search).unwrap_or(regex::Regex::new("").unwrap());
    
    let articles_to_display = articles
        .deref()
        .borrow()
        .iter()
        .filter(|a| a.matches(&r))
        .cloned()
        .collect::<Vec<_>>();

    let on_download_click = {
        let articles = articles.clone();
        Callback::from(move |_: MouseEvent| {
            let bytes = to_csv(articles.deref().borrow().deref()).unwrap();

            match download_bytes_as_file(&bytes, "out.csv") {
                Ok(_) => (),
                Err(error) => {gloo_console::log!(format!("{error}"));}
            }
        })
    };
    
    let article_per_page = use_state(|| 10i32);
    let table_current_page = use_state(|| 0i32);

    let first_article = (table_current_page.deref() * article_per_page.deref()).clamp(0, articles_to_display.len() as i32) as usize;
    let last_article = (first_article as i32 + article_per_page.deref()).clamp(0, articles_to_display.len() as i32) as usize;
    let articles_slice = &articles_to_display[first_article..last_article];

    let dummy_state = use_state(|| ());
    let update_table = {
        Callback::from(move |_: ()| {
            dummy_state.set(());
        })
    };
    
    html! {
        <div class="container-fluid">
            <hr/>
            <TableGlobalSearch filter={global_search.clone()}/>
            <p>{global_search.clone().to_string()}</p>
            <table class="table table-hover table-bordered">
                <thead>
                    <tr>
                        <th></th>
                        <HeaderCell articles={articles.clone()} name="Doi" update_table={update_table.clone()}/>
                        <HeaderCell articles={articles.clone()} name="Title" update_table={update_table.clone()}/>
                        <HeaderCell articles={articles.clone()} name="First author" update_table={update_table.clone()}/>
                        <HeaderCell articles={articles.clone()} name="Abstract" update_table={update_table.clone()}/>
                        <HeaderCell articles={articles.clone()} name="Year published" update_table={update_table.clone()}/>
                        <HeaderCell articles={articles.clone()} name="Citations" update_table={update_table.clone()}/>
                        <HeaderCell articles={articles.clone()} name="Score" update_table={update_table.clone()}/>
                    </tr>
                </thead>
                <tbody>
                    { articles_slice.iter().map(|article| html!{<Row article={article.clone()} update_selected={update_selected.clone()}/>} ).collect::<Html>() }
                </tbody>
            </table>
            <TableFooter article_total_number={articles_to_display.len()} article_per_page={article_per_page} table_current_page={table_current_page}/>
            <DownloadButton onclick={on_download_click}/>
        </div>
    }
}


#[derive(Clone, PartialEq, Properties)]
pub struct HeaderCellProps {
    name: AttrValue,
    articles: Rc<RefCell<Vec<Article>>>,
    update_table: Callback<()>
}

#[function_component(HeaderCell)]
fn header_cell(props: &HeaderCellProps) -> Html {
    let sort = {
        let articles = props.articles.clone();
        let name = props.name.clone();
        let update_table = props.update_table.clone();
        Callback::from(move |_: MouseEvent| {
            let mut ref_vec = articles.deref().borrow_mut();
            ref_vec.deref_mut().sort_by_key(|a| std::cmp::Reverse(a.get(&name).unwrap_or_default()));
            update_table.emit(());
        })
    };
    html! {
        <th><button class="btn col-12 text-start" onclick={sort}><strong>{&props.name}</strong><i class="bi bi-sort-down float-end"></i></button></th>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TableGlobalSearchProps {
    filter: UseStateHandle<String>,
}

#[function_component(TableGlobalSearch)]
fn table_global_search(props: &TableGlobalSearchProps) -> Html {
    let input_node_ref = use_node_ref();
    let oninput = {
        let filter = props.filter.clone();
        let input_node_ref = input_node_ref.clone();
        Callback::from(move |_: InputEvent| {
            let value = input_node_ref.cast::<web_sys::HtmlInputElement>().unwrap().value();
            filter.set(value);
        })
    };

    html! {
        <div class="row justify-content-end">
            <div class="mb-3 form-check col" style="max-width: 20%">
                <label class="form-label">{"Search all fields"}</label>
                <input type="text" class="form-control" oninput={oninput} ref={input_node_ref}/>
            </div>
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
            <button class="btn btn-outline-secondary btn-lg mb-10" onclick={props.onclick.clone()}><i class="bi bi-download me-2"></i>{"Download articles"}</button>
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