use crate::models::voice::Voice;
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
    let voice_index = voice.index;

    let ontouchstart = {
        let onchange = onchange.clone();

        Callback::from(move |e: TouchEvent| {
            e.prevent_default();

            let touches = e.changed_touches();
            for i in 0..touches.length() {
                let t = touches.item(i);
                let touch_id = t.map_or(0, |t| t.identifier() as usize);
                log::info!(">> touchstart {:?}", touch_id);
                onchange.emit((voice_index, touch_id, true));
            }
        })
    };

    let ontouchend = {
        let onchange = onchange.clone();
        Callback::from(move |e: TouchEvent| {
            e.prevent_default();

            let touches = e.changed_touches();
            for i in 0..touches.length() {
                let t = touches.item(i);
                let touch_id = t.map_or(0, |t| t.identifier() as usize);
                log::info!(">> touchend {:?}", touch_id);
                onchange.emit((voice_index, touch_id, false));
            }
        })
    };

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
            ontouchstart={ontouchstart}
            ontouchend={ontouchend}
            onmousedown={onmousedown}>
            { voice.name }
        </button>
    }
}
