/*
   This module defines a Slider component for Yew, which is used to create
   input sliders with customizable labels, values, and callback functions.

   The Slider component utilizes thread-local mutable memory for generating
   unique IDs and integrates with HTML input elements for input manipulation.
*/

use std::cell::Cell;
use web_sys::HtmlInputElement;
use yew::events::InputEvent;
use yew::{html, Callback, Component, Context, Html, Properties, TargetCast};

thread_local! {
    static SLIDER_ID: Cell<usize> = Cell::default();
}

fn next_slider_id() -> usize {
    SLIDER_ID.with(|cell| cell.replace(cell.get() + 1))
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub label: &'static str,
    pub value: u64,
    pub onchange: Callback<u64>,
    pub min: u64,
    pub max: u64,
}

pub struct Slider {
    id: usize,
}

impl Component for Slider {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            id: next_slider_id(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            label,
            value,
            ref onchange,
            min,
            max,
        } = *ctx.props();

        let display_value = value;

        let id = format!("slider-{}", self.id);

        let oninput = onchange.reform(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input.value_as_number() as u64
        });

        html! {
            <div class="slider">
                <label for={id.clone()} class="slider__label">{ label }{": "}{ display_value }</label>
                <input type="range"
                    value={value.to_string()}
                    {id}
                    class="slider__input"
                    min={min.to_string()} max={max.to_string()} step={"1"}
                    {oninput}
                />
            </div>
        }
    }
}
