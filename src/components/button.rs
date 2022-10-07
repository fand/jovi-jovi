use crate::models::voice::Voice;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{AddEventListenerOptions, HtmlElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub voice: Voice,
    pub onchange: Callback<(Voice, bool)>,
}

#[function_component(Button)]
pub fn button(ButtonProps { voice, onchange }: &ButtonProps) -> html {
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
                    let voice = voice.clone();
                    let onchange = onchange.clone();
                    Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
                        log::info!("touchstart!");
                        onchange.emit((voice.to_owned(), true));
                    }))
                };

                let ontouchend = {
                    let voice = voice.clone();
                    let onchange = onchange.clone();
                    Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
                        log::info!("touchend!");
                        onchange.emit((voice.to_owned(), false));
                    }))
                };

                button
                    .add_event_listener_with_callback_and_add_event_listener_options(
                        "touchstart",
                        ontouchstart.as_ref().unchecked_ref(),
                        AddEventListenerOptions::new().passive(true),
                    )
                    .expect("Failed to listen touchstart");
                button
                    .add_event_listener_with_callback_and_add_event_listener_options(
                        "touchend",
                        ontouchend.as_ref().unchecked_ref(),
                        AddEventListenerOptions::new().passive(true),
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
    // ontouchstart.forget();
    // ontouchend.forget();
    // let onmousedown = {
    //     let onchange = onchange.clone();
    //     let voice = voice.clone();
    //     Callback::from(move |_| {
    //         // let prom = audio.play().expect("failed to play");
    //         onchange.emit((voice.to_owned(), true));
    //         ()
    //     })
    // };

    // let onmouseup = {
    //     let onchange = onchange.clone();
    //     let voice = voice.clone();
    //     Callback::from(move |_| {
    //         onchange.emit((voice.to_owned(), false));
    //         ()
    //     })
    // };

    // let onpointerdown = {
    //     let onchange = onchange.clone();
    //     let voice = voice.clone();
    //     Callback::from(move |_| {
    //         log::info!(">> down! {}", voice.name);
    //         onchange.emit((voice.to_owned(), true));
    //         ()
    //     })
    // };

    // let onpointerup = {
    //     let onchange = onchange.clone();
    //     let voice = voice.clone();
    //     Callback::from(move |_| {
    //         log::info!(">> up! {}", voice.name);
    //         onchange.emit((voice.to_owned(), false));
    //         ()
    //     })
    // };

    html! {
        <>
            <button type="button"
                ref={button_ref}>
                // onpointerdown={onpointerdown}
                // onpointerup={onpointerup}>
                { voice.name }
            </button>
        </>
    }
}
