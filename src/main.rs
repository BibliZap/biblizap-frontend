use yew::prelude::*;
use std::collections::HashMap;
use std::ops::Deref;

mod legal;
use legal::*;

mod table;
use table::*;

mod navbar;
use navbar::*;

mod wall;
use wall::*;

mod form;
use form::SnowballForm;

mod common;
use common::{Error, CurrentPage};



#[function_component(App)]
fn app() -> Html {
    let current_page = use_state(|| CurrentPage::BibliZapApp);
    
    let content = match current_page.deref() {
        CurrentPage::BibliZapApp => { html!{<BibliZapApp/>} },
        CurrentPage::HowItWorks => { html!{<HowItWorks/>} },
        CurrentPage::LegalInformation => { html!{<LegalInformation/>} },
        CurrentPage::Contact => { html!{<Contact/>} }
    };
    html! {
        <main>
            <NavBar current_page={current_page}/>
            <Wall/>
            {content}
        </main>
    }
}

#[function_component(BibliZapApp)]
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
            <SnowballForm {on_requesting_table} {on_receiving_response} {blacklist}/>
            <TableContainer table_status={table_status.clone()} update_blacklist={update_blacklist}/>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}