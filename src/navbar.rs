use yew::prelude::*;

#[function_component]
pub fn NavBar() -> Html {
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