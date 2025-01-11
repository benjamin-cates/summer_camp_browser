use web_sys::HtmlInputElement;
use yew::{function_component, html, use_memo, use_state, Callback, Event, Html, MouseEvent};
use wasm_bindgen::JsCast;

use crate::{camps::all_camps, html::camp::CampPage, structs::SummerCamp};

#[function_component]
pub fn Home() -> Html {
    let query_handle = use_state(|| "".to_string());
    let query = (*query_handle).clone();
    let set_query = {
        let query = query_handle.clone();
        Callback::from(move |e: Event| {
            let input = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                query.set(input.value());
            }
        })
    };
    let set_query_none = {
        let query = query_handle.clone();
        Callback::from(move |_: MouseEvent| {
            query.set("".to_string());
        })
    };
    let all = use_memo(query_handle, |query| {
        let mut all = all_camps().into_iter().map(|sc| (sc.get_match_score(query.as_str()), sc)).collect::<Vec<(usize, SummerCamp)>>();
        all.sort_by_key(|v| -(v.0 as i32));
        all.into_iter().filter(|v| query.len() < 2 || v.0 != 0).map(|v| v.1).collect::<Vec<SummerCamp>>()
    });
    html! {
        <>
            <div class="header">
                <div>{"Summer Camp Browser"}</div>
                <div class="input_holder">
                <input type="text" placeholder="Search..." onchange={set_query} value={query.clone()}/>
                <button class="end_query_button" onclick={set_query_none}>{"×"}</button>
                </div>
                <div style="opacity: 0;">{"Summer Camp Browser"}</div>
            </div>
            <div class="home">
                if all.len() == 0 {
                    <div class="no_results">{"No results for \""}{query}{"\""}</div>
                }
                {all.iter().map(|camp: &SummerCamp| {
                    html! {
                        <>
                            <CampPage key={camp.identifier} summer_camp={camp.identifier}/>
                        </>
                    }
                }).collect::<Html>()}
            </div>
        </>
    }
}
