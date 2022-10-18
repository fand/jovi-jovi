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

    // Yew doesn't support touchstart/touchend, so we have to add event listeners manually
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
                    Closure::<dyn Fn(_)>::wrap(Box::new(move |e: TouchEvent| {
                        e.prevent_default();

                        let touches = e.changed_touches();
                        for i in 0..touches.length() {
                            let t = touches.item(i);
                            let touch_id = t.map_or(0, |t| t.identifier() as usize);
                            log::info!(">> touchstart {:?}", touch_id);
                            onchange.emit((voice.index, touch_id, true));
                        }
                    }))
                };

                let ontouchend = {
                    let onchange = onchange.clone();
                    Closure::<dyn Fn(_)>::wrap(Box::new(move |e: TouchEvent| {
                        e.prevent_default();

                        let touches = e.changed_touches();
                        for i in 0..touches.length() {
                            let t = touches.item(i);
                            let touch_id = t.map_or(0, |t| t.identifier() as usize);
                            log::info!(">> touchend {:?}", touch_id);
                            onchange.emit((voice.index, touch_id, false));
                        }
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
            onchange.emit((voice.index, 0, true));
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
