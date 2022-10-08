use crate::models::voice::Voice;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{AddEventListenerOptions, HtmlElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub voice: Voice,
    pub onchange: Callback<(usize, bool)>,
    pub is_playing: bool,
}

#[function_component(Button)]
pub fn button(
    ButtonProps {
        voice,
        onchange,
        is_playing,
    }: &ButtonProps,
) -> html {
    let button_ref = use_node_ref();

    {
        let button_ref = button_ref.clone();
        let voice = voice.clone();
        let onchange = onchange.clone();

        use_effect_with_deps(
            move |button_ref| {
                let button = button_ref
                    .cast::<HtmlElement>()
                    .expect("div_ref not attached to div element");

                let ontouchstart = {
                    let onchange = onchange.clone();
                    Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                        log::info!("touchstart");
                        e.prevent_default();
                        onchange.emit((voice.index, true));
                    }))
                };

                let ontouchend = {
                    let onchange = onchange.clone();
                    Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                        log::info!("touchend");
                        e.prevent_default();
                        onchange.emit((voice.index, false));
                    }))
                };

                button
                    .add_event_listener_with_callback_and_add_event_listener_options(
                        "touchstart",
                        ontouchstart.as_ref().unchecked_ref(),
                        AddEventListenerOptions::new().passive(false),
                    )
                    .expect("Failed to listen touchstart");
                button
                    .add_event_listener_with_callback_and_add_event_listener_options(
                        "touchend",
                        ontouchend.as_ref().unchecked_ref(),
                        AddEventListenerOptions::new().passive(false),
                    )
                    .expect("Failed to listen touchend");

                move || {
                    button
                        .remove_event_listener_with_callback(
                            "touchstart",
                            ontouchstart.as_ref().unchecked_ref(),
                        )
                        .expect("Failed to unlisten touchstart");
                    button
                        .remove_event_listener_with_callback(
                            "touchend",
                            ontouchend.as_ref().unchecked_ref(),
                        )
                        .expect("Failed to unlisten touchend");
                }
            },
            button_ref,
        );
    }

    let onmousedown = {
        let onchange = onchange.clone();
        let voice = voice.clone();
        Callback::from(move |e: web_sys::MouseEvent| {
            log::info!("mousedown");
            onchange.emit((voice.index, true));
        })
    };

    html! {
        <>
            <button type="button"
                class={if *is_playing { "playing" } else { "" } }
                ref={button_ref}
                onmousedown={onmousedown}>
                { voice.name }
            </button>
        </>
    }
}
