use wasm_bindgen::JsValue;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{CanvasRenderingContext2d, HtmlInputElement};
use yew::prelude::*;

use crate::components::button::Button;
use crate::models::voice::VOICES;

const MIN_BPM: u32 = 60;
const MAX_BPM: u32 = 180;

async fn then<F: Fn(JsValue) -> ()>(p: js_sys::Promise, f: F) -> () {
    let x = JsFuture::from(p).await.unwrap();
    f(x);
}

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub canvas_ctx: CanvasRenderingContext2d,
}

#[function_component(App)]
pub fn app(AppProps { canvas_ctx }: &AppProps) -> html {
    log::debug!(">> App.render: {:?}", canvas_ctx);

    let is_playing = use_state(|| VOICES.map(|_| false));
    let bpm = use_state(|| 120);
    let is_changing_bpm = use_state(|| false);

    // Load audio files and connect them to the analyzer
    let audios = use_ref(|| {
        VOICES.map(|v| web_sys::HtmlAudioElement::new_with_src(v.filename).expect("failed to load"))
    });

    // Register mouse up handler
    let onmouseup = {
        let audios = audios.clone();
        let is_playing = is_playing.clone();
        Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {
            log::info!("mouseup");

            for i in 0..VOICES.len() {
                let a = &audios[i];
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
        let audios = audios.clone();
        let is_playing = is_playing.clone();
        let is_changing_bpm = is_changing_bpm.clone();

        Callback::from(move |(i, playing): (usize, bool)| {
            if playing {
                audios[i].set_current_time(0.0);
                audios[i].set_loop(true);

                let a = audios[i].play().expect("failed to play");

                spawn_local(then(a, |x| {
                    log::info!("resolved: {:?}", x);
                }))
            } else {
                audios[i].set_loop(false);
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
        let audios = audios.clone();

        Callback::from(move |e: web_sys::InputEvent| {
            let target = e.target().unwrap();
            let input = target.dyn_ref::<HtmlInputElement>().unwrap();

            if let Ok(v) = input.value().parse::<u32>() {
                let v = v.clamp(MIN_BPM, MAX_BPM);
                bpm.set(v);

                let speed = v as f64 / 120.0;
                for a in audios.iter() {
                    a.set_playback_rate(speed);
                }
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
