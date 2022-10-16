use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::button::Button;
use crate::models::voice::VOICES;
use crate::utils::{window_add_event_listener, window_remove_event_listener};

const MIN_BPM: u32 = 60;
const MAX_BPM: u32 = 180;

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub play: Callback<usize>,
    pub pause: Callback<usize>,
    pub set_speed: Callback<f64>,
}

#[function_component(App)]
pub fn app(
    AppProps {
        play,
        pause,
        set_speed,
    }: &AppProps,
) -> html {
    let is_playing = use_state(|| VOICES.map(|_| false));
    let bpm = use_state(|| 120);
    let is_changing_bpm = use_state(|| false);

    // Register mouse up handler
    let onmouseup = {
        let pause = pause.clone();
        let is_playing = is_playing.clone();

        Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {
            log::info!("mouseup");

            for i in 0..VOICES.len() {
                pause.emit(i);

                let mut isp = *is_playing;
                isp[i] = false;
                is_playing.set(isp);
            }
        })
    };
    use_effect_with_deps(
        move |_| {
            window_add_event_listener("mouseup", &onmouseup);
            move || {
                window_remove_event_listener("mouseup", &onmouseup);
            }
        },
        (),
    );

    let onchange = {
        let is_playing = is_playing.clone();
        let is_changing_bpm = is_changing_bpm.clone();

        let play = play.clone();
        let pause = pause.clone();

        Callback::from(move |(i, playing): (usize, bool)| {
            if playing {
                play.emit(i);
            } else {
                pause.emit(i);
            }

            let mut isp = *is_playing;
            isp[i] = playing;
            is_playing.set(isp);

            is_changing_bpm.set(false);
        })
    };

    // Setup BPM input
    let toggle_bpm_slider = {
        let is_changing_bpm = is_changing_bpm.clone();
        Callback::from(move |_| is_changing_bpm.set(!*is_changing_bpm))
    };

    let onchange_bpm = {
        let bpm = bpm.clone();
        let set_speed = set_speed.clone();

        Callback::from(move |e: web_sys::InputEvent| {
            let target = e.target().unwrap();
            let input = target.dyn_ref::<HtmlInputElement>().unwrap();

            if let Ok(v) = input.value().parse::<u32>() {
                let v = v.clamp(MIN_BPM, MAX_BPM);
                bpm.set(v);

                let speed = v as f64 / 120.0;
                set_speed.emit(speed);
            }
        })
    };

    html! {
        <>
            <header>
                <div class={if *is_changing_bpm {"title hidden"} else {"title"}}>
                    <h1>{"JOVI JOVI"}</h1>
                </div>
                <div class="bpm">
                    <label>{"BPM"}</label>
                    <div class={if *is_changing_bpm {"bpm_slider_wrapper visible"} else { "bpm_slider_wrapper" }}>
                        <input class="bpm_slider" type="range" step="1" min={MIN_BPM.to_string()} max={MAX_BPM.to_string()} value={bpm.to_string()} oninput={onchange_bpm.clone()} />
                    </div>
                    <button class="bpm_button" type="button" onclick={toggle_bpm_slider.clone()}>{bpm.to_string()}</button>
                </div>
            </header>
            <div class="buttons">
                { VOICES.iter().enumerate().map(|(i, voice)| html! { <Button voice={voice.clone()} onchange={onchange.clone()} is_playing={is_playing[i]}/> }).collect::<Html>() }
            </div>
        </>
    }
}
