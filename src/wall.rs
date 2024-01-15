use yew::prelude::*;
use rand::Rng;

#[function_component]
pub fn Wall() -> Html {
    let possible_catchwords =
        ["The reference of my reference is my reference",
        "Cite us !",
        "Open-source",
        "Fork us on github",
        "Created by Bastien, Raphaël and Victor",
        "Doesn't make coffee",
        "Powered by the Lens",
        "Feedback is appreciated",
        "Don't get hit by the snowball",
        "Snowballing from the Mont-Blanc"];
    
    let index = rand::thread_rng().gen_range(0..(possible_catchwords.len()));

    let catchword = possible_catchwords[index];

    html! { 
        <div class="container text-center my-5">
            <h1 class="main-title">
                <img src="/icons/biblizap-snowball-round-fill.svg" id="logo" alt="" width="300vw" style="margin-bottom: 50px"/>
                {"BibliZap"}
            </h1>
            <h5 class="text-end">{catchword}</h5>
        </div>
    }
}