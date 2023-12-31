use std::cell::RefCell;
use std::rc::Rc;

use serde::Serialize;
use yew::prelude::*;

use crate::common::{self, SearchFor, get_value};

use crate::table::article::Article;
use crate::common::*;

#[derive(Clone, PartialEq, Properties)]
pub struct FormProps {
    pub on_requesting_table: Callback<()>,
    pub on_receiving_response: Callback<Result<Rc<RefCell<Vec<Article>>>, Error>>,
}

#[derive(Clone, PartialEq, Properties, Debug, Default, Serialize)]
struct SnowballParameters {
    output_max_size: usize,
    depth: u8,
    input_id_list: Vec<String>,
    search_for: common::SearchFor
}

/*impl SnowballParameters {
    fn from_nodes() -> Result<SnowballParameters, common::Error> {

    }
}*/

//const CRATE_CONFIG: &str = include_str!("../example.json");

async fn get_response(form_content: &SnowballParameters) -> Result<Rc<RefCell<Vec<Article>>>, Error> {
    use gloo_utils::document;
    let url = document().document_uri();
    let url = match url {
        Ok(href) => Ok(href),
        Err(err) => Err(Error::JsValue(err.as_string().unwrap_or_default()))
    }?.replace('#', "");

    let response = gloo_net::http::Request::post(&format!("{}api", url))
        .header("Access-Control-Allow-Origin", "*")
        .body(serde_json::to_string(&form_content)?)?
        .send()
        .await?
        .text()
        .await?;

    let value = serde_json::from_str::<serde_json::Value>(&response)?;
    let mut articles = serde_json::from_value::<Vec<Article>>(value)?;

    articles.sort_by_key(|article| std::cmp::Reverse(article.score.unwrap_or_default()));
    
    Ok(Rc::new(RefCell::new(articles)))
}

#[function_component]
pub fn SnowballForm(props: &FormProps) -> Html {
    let id_list_node = use_node_ref();
    let depth_node = use_node_ref();
    let output_max_size_node = use_node_ref();
    let search_for_node = use_node_ref();
    
    let onsubmit: Callback<SubmitEvent> = {
        let id_list_node = id_list_node.clone();
        let depth_node = depth_node.clone();
        let output_max_size_node = output_max_size_node.clone();
        let search_for_node = search_for_node.clone();
        let on_receiving_response = props.on_receiving_response.clone();
        let on_requesting_table = props.on_requesting_table.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            on_requesting_table.emit(());
            

            let input_id_list = get_value(&id_list_node).unwrap()
                .trim()
                .split(' ')
                .map(str::to_string)
                .collect::<Vec<String>>();

            let depth = get_value(&depth_node).unwrap().parse::<u8>().unwrap();

            let output_max_size = get_value(&output_max_size_node).unwrap().parse::<usize>().unwrap();
            
            let search_for = match get_value(&search_for_node).unwrap().as_str() {
                "References" => SearchFor::References,
                "Citations" => SearchFor::Citations,
                "Both" => SearchFor::Both,
                &_ => SearchFor::Both
            };
            
            let form_content = SnowballParameters {
                output_max_size,
                depth,
                input_id_list,
                search_for
            };
            
            let on_receiving_response = on_receiving_response.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = get_response(&form_content).await;
                on_receiving_response.emit(response);
            });
        })
    };
    html! {
        <form class="container-md" onsubmit={onsubmit} style={"margin-bottom: 50px;"}>
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
                    <select class="form-select" aria-label="Default select example" id="maxOutputSizeSelect" value="20" ref={output_max_size_node.clone()}>
                        <option value="20" selected=true>{"20"}</option>
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
                <label class="form-check-label" for="searchForSelect">{"Search direction"}</label>
                <select class="form-select" aria-label="Default select example" id="searchForSelect" ref={search_for_node.clone()}>
                    <option value="Both" selected=true>{"Both"}</option>
                    <option value="Citations">{"Citations"}</option>
                    <option value="References">{"References"}</option>
                </select>
            </div>
            <div class="text-center">
                <button type="submit" class="btn btn-outline-secondary btn-lg">{"Search for related articles"}</button>
            </div>
        </form>
    }
}
