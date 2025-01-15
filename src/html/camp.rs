use std::sync::OnceLock;

use yew::{classes, function_component, html, use_state, Callback, Html, MouseEvent, Properties};

use crate::camps;
use crate::structs::SummerCamp;

static CAMPS_DIR: OnceLock<Vec<SummerCamp>> = OnceLock::new();
fn get_dir() -> &'static Vec<SummerCamp> {
    CAMPS_DIR.get_or_init(camps::all_camps)
}

#[derive(Properties, PartialEq)]
pub struct SpecProps {
    name: String,
    sc: SummerCamp,
}

#[function_component]
pub fn Specialization(props: &SpecProps) -> Html {
    let clicked = use_state(|| false);
    let onclick = {
        let clicked = clicked.clone();
        Callback::from(move |_: MouseEvent| {
            clicked.set(!*clicked);
        })
    };
    html! {
        <div class={"specialization"}>
            <button onclick={onclick} class={classes!(if *clicked {"active"} else {"inactive"}, "specialization_hook")}><span class="track_name">{&props.name}</span> <span class="dropdown_triangle">{"▼"}</span></button>
            <div class={classes!(if *clicked {"active"} else {"inactive"},"specialization_text")}>
                <div>
                    if props.sc.link.is_some() {
                        <p class="info">{"Link: "}<a href={props.sc.link}>{props.sc.link}</a></p>
                    }
                    if props.sc.length_wk.is_some() {
                        <p class="info">{"Duration: "}{props.sc.length_wk}{" week"}{if props.sc.length_wk == Some(1.0) {""} else {"s"}}</p>
                    }
                    {props.sc.description.iter().map(|val| {html! {<p class={classes!("info", "description")}>{val}</p>}}).collect::<Html>()}
                    if props.sc.requirements.len() != 0 {
                        <h3>{"Additional requirements"}</h3>
                        <ul class="requirements_list">
                            {props.sc.requirements.iter().map(|req| {html!{ <li class="requirement">{format!("{}",req)}</li>}}).collect::<Html>()}
                        </ul>
                    }
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CampProps {
    pub summer_camp: String,
}

#[function_component]
pub fn CampPage(props: &CampProps) -> Html {
    let pos = get_dir()
        .iter()
        .position(|v| v.identifier == props.summer_camp.as_str())
        .unwrap_or(0);
    let sc = get_dir()[pos].clone();
    html! {
        <div class="camp_listing">
            <h1 class="camp_name">{&props.summer_camp}</h1>
            {sc.description.iter().map(|val| {html! {<p class={"description"}>{val}</p>}}).collect::<Html>()}
            <p class="links">
            if let Some(link) = sc.link {
                <a href={link}>{"Website link"}</a>
                if let Some(_) = sc.apply_link {
                    {" "}
                }
            }
            if let Some(apply_link) = sc.apply_link {
                <a href={apply_link} class="info">{"Apply link"}</a>
            }
            </p>
            <strong>{"Application/Eligibility"}</strong>
            <ul class="requirements_list">
                {sc.requirements.iter().map(|req| {html!{ <li class="requirement">{format!("{}",req)}</li>}}).collect::<Html>()}
            </ul>
            <strong>{"Info"}</strong>
            <ul class="info_list">
                if let Some(tuition) = sc.tuition {
                    <li class="info">{"Tuition: ~$"}{tuition}</li>
                }
                if let Some(application_fee) = sc.application_fee {
                    <li class="info">{"Application fee: $"}{application_fee}</li>
                }
                if let Some(location) = sc.location {
                    <li class="info">{"Location: "}<strong>{location}</strong></li>
                }
                if let Some(organization) = sc.organization {
                    <li class="info">{"Organization: "}{organization}</li>
                }
                if let Some(deadline) = sc.deadline {
                    <li class="info">{"Deadline is "}<strong>{deadline}</strong></li>
                }
                if let Some(application_opens) = sc.application_opens {
                    <li class="info">{"Application opens on "}{application_opens}</li>
                }
                if let Some(length_wk) = sc.length_wk {
                    <li class="info">{"Duration: "}<strong>{length_wk}{" week"}{if length_wk == 1.0 {""} else {"s"}}</strong></li>
                }
                if let Some(acceptance_rate) = sc.acceptance_rate {
                    <li class="info">{"Acceptance rate: "}{acceptance_rate}{"%"}</li>
                }
                if let Some(last_updated) = sc.last_updated {
                    <li class="info">{"Listing last updated on "}{last_updated}</li>
                }
            </ul>
            if sc.notes.len() > 0 {
                <strong>{"Additional Notes"}</strong>
                <ul class="info_list">
                    {sc.notes.iter().map(|v| html!{ <li>{v}</li> }).collect::<Html>()}
                </ul>
            }
            if sc.specialization.is_some() {
                <strong style={"display: block; padding-top: 1rem"}>{"Tracks"}</strong>
                {sc.specialization
                .unwrap_or_else(|| vec![])
                .into_iter()
                .map(|(name, details)| {
                        html! {<Specialization name={name.to_string()} sc={details.clone()}/>}
                }).collect::<Html>()}
            }
        </div>
    }
}
