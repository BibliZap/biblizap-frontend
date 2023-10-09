use yew::prelude::*;

#[function_component]
pub fn Wall() -> Html {
    html! { 
        <div class="container text-center my-5">
            <h1 class="main-title">
                <img src="/icons/biblizap-snowball-round-fill.svg" id="logo" alt="" width="300vw"/>
                {"BibliZap"}
            </h1>
        </div>
    }
}