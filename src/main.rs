use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[function_component(App)]
fn app() -> Html {
    let on_click: Callback<MouseEvent> = Callback::from(move |_| {
        let greeting = "aaa!".to_string();
        web_sys::console::log_1(&greeting.into()); // if uncommented will print
    });
    html! {
        <main>
            <NavBar/>
            <Wall/>
            <SnowballForm/>
            <Button {on_click}/>
        </main>
    }
}

#[function_component]
fn NavBar() -> Html {
    html! {
    <nav class="navbar navbar-expand-lg bg-body-tertiary">
        <div class="container-fluid">
            <a class="navbar-brand" href="#" id="navbar-title">
                <img src="/icons/biblizap-nosnowball-round-fill.svg" alt="" width="50" height="50" class="px-2"/>
                {"BibliZap"}
            </a>
            <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                <span class="navbar-toggler-icon"></span>
            </button>
            <div class="collapse navbar-collapse" id="navbarSupportedContent">
                <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                <li class="nav-item">
                    <a class="nav-link active" aria-current="page" href="#">
                    <i class="bi bi-house-fill px-2"></i>
                    {"Home"}
                    </a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" href="#">
                    <i class="bi bi-lightbulb-fill px-2"></i>
                    {"How it works"}
                    </a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" href="#">
                    <i class="bi bi-send-fill px-2"></i>
                    {"Contact"}
                    </a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" href="#">
                    <i class="bi bi-info-circle-fill px-2"></i>
                    {"Legal information"}
                    </a>
                </li>
                </ul>
            </div>
        </div>
    </nav>
    }
}

#[function_component]
fn Wall() -> Html {
    html! { 
        <div class="container text-center mt-5">
            <h1 class="main-title">
                <img src="/icons/biblizap-snowball-round-fill.svg" id="logo" alt="" width="300vw"/>
                {"BibliZap"}
            </h1>
        </div>
    }
}

#[function_component]
fn Button(props: &Props) -> Html {
    html! {
        <button class="btn btn-primary" onclick={&props.on_click}>{"Search for related articles"}</button>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct FormProps {
    
}

#[derive(Clone, PartialEq, Properties, Debug, Default, Serialize, Deserialize)]
struct SnowballFormContent {
    id_list: String,
}

#[function_component]
fn SnowballForm(props: &FormProps) -> Html {
    let form_content = use_state(|| SnowballFormContent::default());
    let id_list_node = use_node_ref();

    let onsubmit: Callback<SubmitEvent> = {
            let id_list_node = id_list_node.clone();    
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let id_list = id_list_node.cast::<web_sys::HtmlInputElement>().unwrap().value();
            web_sys::console::log_1(&id_list.to_string().into()); // if uncommented will print
        })
    };
    html! {
        <form class="container-md" onsubmit={onsubmit}>
            <div class="mb-3 form-check">
                <label for="idInput" class="form-label">{"Enter a PMID, a DOI or a Lens ID"}</label>
                <input type="text" class="form-control" id="idInput" name="idListInput" ref={id_list_node.clone()}/>
                <div id="idInputHelp" class="form-text">{"You can enter multiple references separated by commas."}</div>
            </div>
            <div class="mb-3 form-check">
                <div class="row">
                <div class="col">
                    <label class="form-check-label" for="depthSelect">{"Select depth"}</label>
                    <select class="form-select" aria-label="Default select example" id="depthSelect" value="2">
                        <option value="1">{"1"}</option>
                        <option value="2" selected=true>{"2"}</option>
                        <option value="3">{"3"}</option>
                    </select>
                    <div id="depthSelectHelp" class="form-text">{"The recommended depth value is 2"}</div>
                </div>
                <div class="col">
                    <label class="form-check-label" for="maxOutputSizeSelect">{"Select output size"}</label>
                    <select class="form-select" aria-label="Default select example" id="maxOutputSizeSelect" value="2">
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
            <div class="text-center">
                <button type="submit" class="btn btn-primary">{"Search for related articles"}</button>
            </div>
        </form>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub on_click: Callback<MouseEvent>,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    //props.on_name_entry.emit(String::from("Bob"));

    html! { "Hello" }
}


fn main() {
    yew::Renderer::<App>::new().render();
}