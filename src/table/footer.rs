use yew::prelude::*;
use std::ops::Deref;
use web_sys::HtmlElement;

#[derive(Clone, PartialEq, Properties)]
pub struct TableFooterProps {
    pub article_total_number: usize,
    pub article_per_page: UseStateHandle<i32>,
    pub table_current_page: UseStateHandle<i32>
}

#[function_component(TableFooter)]
pub fn table_footer(props: &TableFooterProps) -> Html {
    let table_current_page = props.table_current_page.deref().to_owned();
    let article_per_page = props.article_per_page.deref().to_owned();
    let first_article = table_current_page * article_per_page + 1;
    let last_article = first_article + article_per_page - 1;

    let total_page_number = (props.article_total_number as i32) / article_per_page;
    let last_page_index = total_page_number - 1;
    
    let contiguous_window_radius = 2;
    let contiguous_low_bound = (table_current_page - contiguous_window_radius).clamp(0, total_page_number);
    let contiguous_high_bound = (contiguous_low_bound + 2*contiguous_window_radius + 1).clamp(0, total_page_number);
    let contiguous_low_bound = (contiguous_high_bound - 2*contiguous_window_radius - 1).clamp(0, total_page_number); //Recalculate in case high bound got clamped

    let contiguous_range = contiguous_low_bound..contiguous_high_bound;
    html! {
        <div class="row" id="table_footer">
            <div class="col">
                <div role="status" aria-live="polite">{format!("Showing {} to {} of {} entries", first_article, last_article, props.article_total_number)}</div>
            </div>

            <div class="col">
                <div class="float-end">
                    <ul class="pagination pagination-lg">
                        if contiguous_low_bound != 0 {
                            <PageItem table_current_page={props.table_current_page.clone()} page_index={0}/>
                            if contiguous_low_bound >= 2 {
                                <li class="page-item disabled"><a  aria-disabled="true" role="link" tabindex="-1" class="page-link">{"…"}</a></li>
                            }
                        }

                        { contiguous_range.into_iter().map(|index| html!{<PageItem table_current_page={props.table_current_page.clone()} page_index={index}/>} ).collect::<Html>() }
                        
                        if contiguous_high_bound != total_page_number {
                            if total_page_number-contiguous_low_bound >= 2 {
                                <li class="page-item disabled"><a  aria-disabled="true" role="link" tabindex="-1" class="page-link">{"…"}</a></li>
                            }
                            <PageItem table_current_page={props.table_current_page.clone()} page_index={last_page_index}/>
                        }
                    </ul>
                </div>
            </div>
        </div>
    }
}



#[derive(Clone, PartialEq, Properties)]
struct PageItemProps {
    table_current_page: UseStateHandle<i32>,
    page_index: i32
}
#[function_component(PageItem)]
fn page_item(props: &PageItemProps) -> Html {
    let onclick = {
        let table_current_page = props.table_current_page.clone();
        let page_index = props.page_index;
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let element = event.target_dyn_into::<HtmlElement>(); 
            if let Some(element) = element { element.scroll_into_view() }
            table_current_page.set(page_index);
        })
    };

    let class = match *props.table_current_page.deref() == props.page_index {
        true => "page-item active",
        false => "page-item"
    };

    html! {
        <li class={class}><button class="page-link" {onclick}>{props.page_index+1}</button></li>
    }
}