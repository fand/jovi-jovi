use crate::models::voice::Voice;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub voice: Voice,
    pub onchange: Callback<bool>,
}

#[function_component(Button)]
pub fn button(ButtonProps { voice, onchange }: &ButtonProps) -> html {
    let onmousedown = {
        let onchange = onchange.clone();
        Callback::from(move |_| {
            // let prom = audio.play().expect("failed to play");
            onchange.emit(true);
            ()
        })
    };

    let onmouseup = {
        let onchange = onchange.clone();
        Callback::from(move |_| {
            // audio.pause();
            onchange.emit(false);
            ()
        })
    };

    html! {
        <>
            <button type="button" onmousedown={onmousedown} onmouseup={onmouseup}>{ voice.name() }</button>
        </>
    }
}
