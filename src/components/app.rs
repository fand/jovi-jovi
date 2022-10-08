use wasm_bindgen::{prelude::Closure, JsCast};
use yew::prelude::*;

use crate::components::button::Button;
use crate::models::voice::VOICES;

#[function_component(App)]
pub fn app() -> html {
    let is_playing = use_state(|| VOICES.map(|_| false));
    let is_loop_mode = use_state(|| true);

    let audio = use_ref(|| {
        VOICES.map(|v| {
            let audio =
                web_sys::HtmlAudioElement::new_with_src(v.filename).expect("failed to load");
            audio.set_loop(true);
            audio
        })
    });

    let onmouseup = {
        let audio = audio.clone();
        let is_playing = is_playing.clone();
        Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {
            log::info!("mouseup");

            for i in 0..VOICES.len() {
                let a = &audio[i];
                a.set_loop(false);

                let mut isp = *is_playing;
                isp[i] = false;
                is_playing.set(isp);
            }
        })
    };

    use_effect_with_deps(
        move |_| {
            let window = web_sys::window().expect("Failed to get Window");

            window
                .add_event_listener_with_callback("mouseup", onmouseup.as_ref().unchecked_ref())
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

    let onchange = {
        let audio = audio.clone();
        let is_playing = is_playing.clone();
        let is_loop_mode = is_loop_mode.clone();

        // log::info!(">> creating onchange {}", *is_loop_mode);

        Callback::from(move |(i, playing): (usize, bool)| {
            // log::info!(">> play {}", playing);
            if playing {
                audio[i].set_current_time(0.0);
                audio[i].set_loop(*is_loop_mode.to_owned());
                audio[i].play().expect("failed to play");
            } else {
                // audio[i].pause().expect("failed to pause");
                audio[i].set_loop(false);
            }

            let mut isp = *is_playing;
            isp[i] = playing;
            is_playing.set(isp);
        })
    };

    let toggle_loop_mode = {
        let is_loop_mode = is_loop_mode.clone();
        Callback::from(move |_| is_loop_mode.set(!*is_loop_mode))
    };

    html! {
        <div class="app">
            <header>
                <div>
                    <h1>{"JOVI JOVI"}</h1>
                </div>
                <div>
                    <button>{"POLY"}</button>
                    <button
                        class={if *is_loop_mode {"enabled"} else {""}}
                        onclick={toggle_loop_mode}>
                        {"LOOP"}
                    </button>
                </div>
            </header>
            <div class="buttons">
                { VOICES.iter().enumerate().map(|(i, voice)| html! { <Button voice={voice.clone()} onchange={onchange.clone()} is_playing={is_playing[i]}/> }).collect::<Html>() }
            </div>
        </div>
    }
}
