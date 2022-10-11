use wasm_bindgen::{prelude::Closure, JsCast};
use yew::prelude::*;

use crate::components::button::Button;
use crate::models::voice::VOICES;

#[function_component(App)]
pub fn app() -> html {
    let is_playing = use_state(|| VOICES.map(|_| false));
    let is_loop_mode = use_state(|| true);
    let canvas_ref = use_node_ref();

    let ctx = use_ref(|| web_sys::AudioContext::new().expect("failed to create AudioContext"));
    let analyzer = use_ref(|| {
        let analyzer = ctx.create_analyser().unwrap();
        analyzer.connect_with_audio_node(&ctx.destination());
        analyzer.set_fft_size(512);

        {
            log::debug!(">> gonna setInterval");
            let analyzer = analyzer.clone();

            let cb = Closure::<dyn Fn()>::new(move || {
                let mut arr: [f32; 512] = [0.0; 512];
                analyzer.get_float_time_domain_data(&mut arr);

                log::info!("{:?}", &arr[0..10]);
            });

            web_sys::window()
                .unwrap()
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    100,
                )
                .unwrap();

            cb.forget();
        }

        analyzer
    });

    let audios = use_ref(|| {
        VOICES.map(|v| {
            let audio =
                web_sys::HtmlAudioElement::new_with_src(v.filename).expect("failed to load");
            audio.set_loop(true);

            // var source = context.createMediaElementSource(audio);
            // source.connect(analyser);
            // analyser.connect(context.destination);
            let src = ctx.create_media_element_source(&audio).unwrap();
            src.connect_with_audio_node(&*analyzer);

            audio
        })
    });

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
        let is_loop_mode = is_loop_mode.clone();

        // log::info!(">> creating onchange {}", *is_loop_mode);

        Callback::from(move |(i, playing): (usize, bool)| {
            // log::info!(">> play {}", playing);
            if playing {
                audios[i].set_current_time(0.0);
                audios[i].set_loop(*is_loop_mode.to_owned());
                audios[i].play().expect("failed to play");
            } else {
                // audios[i].pause().expect("failed to pause");
                audios[i].set_loop(false);
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

            <canvas ref={canvas_ref}/>
        </div>
    }
}
