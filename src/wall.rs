use yew::prelude::*;

#[function_component]
pub fn Wall() -> Html {
    let possible_catchwords =
        ["The reference of my reference is my reference",
        "Please cite us",
        "Empowering citation bias",
        "Open-source",
        "Fork us on github",
        "We might need a real UI designer",
        "Probably not the next Google",
        "Created by Bastien, Raphaël and Victor"];

    let index = (js_sys::Math::random()*(possible_catchwords.len() as f64)) as usize;
    let index = index.clamp(0, possible_catchwords.len());

    let catchword = possible_catchwords[index];

    html! { 
        <div class="container text-center my-5">
            <h1 class="main-title">
                <img src="/icons/biblizap-snowball-round-fill.svg" id="logo" alt="" width="300vw"/>
                {"BibliZap"}
            </h1>
            <h5 class="text-end">{catchword}</h5>
        </div>
    }
}