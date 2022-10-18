use crate::{
    models::voice::Voice,
    utils::{el_add_event_listener, el_remove_event_listener},
};
use wasm_bindgen::prelude::Closure;
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub voice: Voice,
    pub onchange: Callback<(usize, usize, bool)>,
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
    let count = use_mut_ref(|| 0 as usize);

    // Yew doesn't support touchstart/touchend, so we have to add event listeners manually
    {
        let button_ref = button_ref.clone();
        let count = count.clone();

        let voice = voice.clone();
        let onchange = onchange.clone();

        use_effect_with_deps(
            move |button_ref| {
                let button = button_ref
                    .cast::<HtmlElement>()
                    .expect("div_ref not attached to div element");

                let ontouchstart = {
                    let count = count.clone();
                    let onchange = onchange.clone();
                    Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                        log::info!("touchstart");
                        e.prevent_default();
                        *count.borrow_mut() += 1;
                        onchange.emit((voice.index, *count.borrow(), true));
                    }))
                };

                let ontouchend = {
                    let onchange = onchange.clone();
                    Closure::<dyn Fn(_)>::wrap(Box::new(move |e: MouseEvent| {
                        log::info!("touchend");
                        e.prevent_default();
                        onchange.emit((voice.index, *count.borrow(), false));
                    }))
                };

                el_add_event_listener(&button, "touchstart", &ontouchstart);
                el_add_event_listener(&button, "touchend", &ontouchend);

                move || {
                    el_remove_event_listener(&button, "touchstart", &ontouchstart);
                    el_remove_event_listener(&button, "touchend", &ontouchend);
                }
            },
            button_ref,
        );
    }

    let onmousedown = {
        let onchange = onchange.clone();
        let voice = voice.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            log::info!("mousedown");
            *count.borrow_mut() += 1;
            onchange.emit((voice.index, *count.borrow(), true));
        })
    };

    html! {
        <button type="button"
            class={if *is_playing { "playing" } else { "" } }
            ref={button_ref}
            onmousedown={onmousedown}>
            { voice.name }
        </button>
    }
}
