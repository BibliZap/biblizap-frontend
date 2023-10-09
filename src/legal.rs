use yew::prelude::*;

use yew::prelude::*;

#[function_component(Legal)]
pub fn legal() -> Html {
    html! {
        <div>
            <h1>{"General principle"}</h1>
            <p>{"BibliZap is a free and open-source project"}<br/>{"BibliZap aims to catalog articles similar to the source article based on both upward and downward citations. Downward citations correspond to the references of the articles (their bibliography). Upward citations correspond to the articles citing the source article. Here is a diagram summarizing the process:"}</p>
        </div>
    }
}

#[function_component(HowItWorks)]
pub fn how_it_works() -> Html {
    html! {
        <div class="container-md">
            <h1 class="mb-4">{"General principle"}</h1>
            <h3>{"BibliZap is a free and open-source project"}</h3>
            <p class="p-3">
                {"BibliZap aims to catalog articles similar to the source article based on both upward and downward citations."}<br/>
                {"Downward citations correspond to the references of the articles (their bibliography)."}<br/>
                {"Upward citations correspond to the articles citing the source article."}
            </p>
            <h3>{"Here is a diagram summarizing the process:"}</h3>
            <img src="icons/BibliZapFig1.svg" class="p-3"/>
            <p>{"At each level, the number of times each PMID appears is recorded. At the end of the process, the sum of occurrences provides the score. For instance, if an article is found once in the references of the source article, then is discovered 6 times in the articles cited by the articles that are cited by the source article, and is not found elsewhere, its score will be 7."}</p>
            <h1 class="mb-4">{"Data sources"}</h1>
            <p>{"Meta-data from articles are provided by The Lens, a not-for-profit service from Cambia. The Lens gathers and harmonises bibliographic data from different sources (Crossref, PubMed, Microsoft Academic, ...)"}</p>
            <img src="icons/scholar-venn.png" class="p-3 img-fluid"/>
            <img src="icons/scholar-chart.png" class="p-3 img-fluid"/>
            <p>
                {"Using the BibliZap web-app freely is possible thanks to The Lens generously providing an API access to all users of the BibliZap web-app."}<br/>
                {"Users of the R package will need a spectific individual token which can be obtained through The Lens for 14 days."}<br/>
                {"BibliZap does not receive financial support from The Lens or Cambia, or any other enterprise or journal."}
            </p>
        </div>
    }
}