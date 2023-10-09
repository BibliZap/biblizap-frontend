use yew::prelude::*;
use thiserror::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

mod table;
use table::*;

mod navbar;
use navbar::*;

mod wall;
use wall::*;


#[derive(Error, Debug)]
pub enum Error {
    #[error("Json error")]
    JsonError(#[from] serde_json::Error),
    #[error("Request error")]
    RequestError(#[from] gloo_net::Error),
}

#[function_component(App)]
fn app() -> Html {
    let table_status = use_state(|| TableStatus::NotRequested);
    let on_receiving_response = { 
        let table_status = table_status.clone();
        Callback::from(move |table: Result<Vec<Article>, Error>| {
            match table {
                Ok(table) => table_status.set(TableStatus::Available(table)),
                Err(error) => table_status.set(TableStatus::RequestError(error.to_string())),
            };
        })
    };
    let on_requesting_table = {
        let table_status = table_status.clone();
        Callback::from(move |_: ()| {
            table_status.set(TableStatus::Requested);
        })
    };

    let blacklist = use_mut_ref(HashMap::<String, bool>::new);
    let blacklist = use_state(|| blacklist);

    let update_blacklist = {
        let blacklist = blacklist.clone();
        Callback::from(move |element : (String, bool)| {
            let rc = blacklist.deref().to_owned();
            rc.borrow_mut().insert(element.0, element.1);
            blacklist.set(rc);
        })
    };
    html! {
        <main>
            <NavBar/>
            <Wall/>
            <SnowballForm {on_requesting_table} {on_receiving_response} {blacklist}/>
            <TableContainer table_status={table_status.clone()} update_blacklist={update_blacklist}/>
        </main>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct FormProps {
    on_requesting_table: Callback<()>,
    on_receiving_response: Callback<Result<Vec<Article>, Error>>,
    blacklist: UseStateHandle<Rc<RefCell<HashMap<String, bool>>>>
}

#[derive(Clone, PartialEq, Properties, Debug, Default, Serialize, Deserialize)]
struct SnowballParameters {
    output_max_size: usize,
    depth: u8,
    input_id_list: Vec<String>,
}

//todo: handle errors properly
async fn get_response(form_content: &SnowballParameters) -> Result<Vec<Article>, Error> {
    let response = gloo_net::http::Request::post("http://127.0.0.1:8080/api")
        .header("Access-Control-Allow-Origin", "*")
        .body(serde_json::to_string(&form_content)?)?
        .send()
        .await?
        .text()
        .await?;

    let value = serde_json::from_str::<serde_json::Value>(&response)?;
    let mut articles = serde_json::from_value::<Vec<Article>>(value)?;

    articles.sort_by_key(|x| -x.score.unwrap());
    
    Ok(articles)
}


#[function_component]
fn SnowballForm(props: &FormProps) -> Html {
    let id_list_node = use_node_ref();
    let depth_node = use_node_ref();
    let output_max_size_node = use_node_ref();
    let blacklist = props.blacklist.clone()
        .as_ref()
        .borrow()
        .clone()
        .into_iter().filter(|x| x.1)
        .map(|x| x.0)
        .collect::<Vec<String>>();

    let blacklist_str = blacklist.join(" ");

    let onsubmit: Callback<SubmitEvent> = {
        let id_list_node = id_list_node.clone();
        let depth_node = depth_node.clone();
        let output_max_size_node = output_max_size_node.clone();
        let on_receiving_response = props.on_receiving_response.clone();
        let on_requesting_table = props.on_requesting_table.clone();
        Callback::from(move |event: SubmitEvent| {
            on_requesting_table.emit(());

            let on_receiving_response = on_receiving_response.clone();
            event.prevent_default();
            let input_id_list = id_list_node.cast::<web_sys::HtmlInputElement>().unwrap().value()
                .split(' ')
                .map(str::to_string)
                .collect::<Vec<String>>();

            let depth = depth_node.cast::<web_sys::HtmlInputElement>().unwrap().value().parse::<u8>().unwrap();

            let output_max_size = output_max_size_node.cast::<web_sys::HtmlInputElement>().unwrap().value().parse::<usize>().unwrap();
            let form_content = SnowballParameters {
                output_max_size,
                depth,
                input_id_list
            };
            wasm_bindgen_futures::spawn_local(async move {
                let response = get_response(&form_content).await;

                format!("{response:#?}");

                on_receiving_response.emit(response);
            });
        })
    };
    html! {
        <form class="container-md" onsubmit={onsubmit}>
            <div class="mb-3 form-check">
                <label for="idInput" class="form-label">{"Enter a PMID, a DOI or a Lens ID"}</label>
                <input type="text" class="form-control" id="idInput" name="idListInput" ref={id_list_node.clone()}/>
                <div id="idInputHelp" class="form-text">{"You can enter multiple references separated by spaces."}</div>
            </div>
            <div class="mb-3 form-check">
                <div class="row">
                <div class="col">
                    <label class="form-check-label" for="depthSelect">{"Select depth"}</label>
                    <select class="form-select" aria-label="Default select example" id="depthSelect" value="2" ref={depth_node.clone()}>
                        <option value="1">{"1"}</option>
                        <option value="2" selected=true>{"2"}</option>
                        <option value="3">{"3"}</option>
                    </select>
                    <div id="depthSelectHelp" class="form-text">{"The recommended depth value is 2"}</div>
                </div>
                <div class="col">
                    <label class="form-check-label" for="maxOutputSizeSelect">{"Select output size"}</label>
                    <select class="form-select" aria-label="Default select example" id="maxOutputSizeSelect" value="10" ref={output_max_size_node.clone()}>
                        <option value="10" selected=true>{"10"}</option>
                        <option value="50">{"50"}</option>
                        <option value="100">{"100"}</option>
                        <option value="200">{"200"}</option>
                        <option value="300">{"300"}</option>
                        <option value="400">{"400"}</option>
                        <option value="500">{"500"}</option>
                        <option value="600">{"600"}</option>
                        <option value="700">{"700"}</option>
                        <option value="800">{"800"}</option>
                        <option value="900">{"900"}</option>
                        <option value="1000">{"1000"}</option>
                    </select>
                </div>
                </div>
            </div>
            <div class="mb-3 form-check">
                <div class="row">
                <div class="col">
                    <input type="checkbox" class="form-check-input" id="exploreCitations"/>
                    <label class="form-check-label" for="exploreCitations">{"Explore citations"}</label>
                </div>
                <div class="col">
                    <input type="checkbox" class="form-check-input" id="exploreReferences"/>
                    <label class="form-check-label" for="exploreReferences">{"Explore references"}</label>
                </div>
                </div>
            </div>
            <div class="mb-3 form-check">
                <label for="blacklistInput" class="form-label">{"DOI blacklist"}</label>
                <input type="text" class="form-control" id="blacklistInput" name="blacklistInput" value={blacklist_str}/>
                <div id="blacklistInputHelp" class="form-text">{"You can enter multiple references separated by spaces."}</div>
            </div>
            <div class="text-center">
                <button type="submit" class="btn btn-primary">{"Search for related articles"}</button>
            </div>
        </form>
    }
}



fn main() {
    yew::Renderer::<App>::new().render();
}