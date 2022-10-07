use wasm_bindgen::{prelude::Closure, JsCast};
use yew::prelude::*;

use crate::components::button::Button;
use crate::models::voice::VOICES;

#[function_component(App)]
pub fn app() -> html {
    let is_playing = use_state(|| false);

    let audio = use_ref(|| {
        let audio = web_sys::HtmlAudioElement::new_with_src("wav/joy.wav").expect("failed to load");
        audio.set_loop(true);
        audio
    });

    let onmouseup = {
        let audio = audio.clone();
        Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {
            log::info!("mouseup");
            audio.pause().expect("failed to pause");
            audio.set_current_time(0.0);
        })
    };

    use_effect_with_deps(
        move |_| {
            let window = web_sys::window().expect("Failed to get Window");

            window
                .add_event_listener_with_callback("pointerup", onmouseup.as_ref().unchecked_ref())
                .expect("addEventListener failed");

            move || {
                let window = web_sys::window().expect("Failed to get Window");
                window
                    .remove_event_listener_with_callback(
                        "mouseup",
                        onmouseup.as_ref().unchecked_ref(),
                    )
                    .expect("addEventListener failed");
            }
        },
        (),
    );

    use_effect_with_deps(
        move |is_playing| {
            if *is_playing {
                audio.play().expect("failed to play");
            } else {
                audio.pause().expect("failed to pause");
                audio.set_current_time(0.0);
            }
            || {}
        },
        *is_playing,
    );

    let onchange = Callback::from(move |(voice, playing)| is_playing.set(playing));

    html! {
        <>
            <header>
                <div>
                    <h1>{"JOVI JOVI"}</h1>
                </div>
                <div>
                    <button>{"POLY"}</button>
                    <button>{"LOOP"}</button>
                </div>
            </header>
            <div class="buttons">
                { VOICES.iter().map(|voice| html! { <Button voice={voice.clone()} onchange={onchange.clone()}/> }).collect::<Html>() }
            </div>
        </>
    }
}
