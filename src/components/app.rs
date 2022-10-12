use std::f64::consts::PI;
use wasm_bindgen::JsValue;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::button::Button;
use crate::models::voice::VOICES;

const DIM: u32 = 1024;
const DIM_F64: f64 = DIM as f64;
const FFT_SIZE: usize = (DIM / 2) as usize;

async fn then<F: Fn(JsValue) -> ()>(p: js_sys::Promise, f: F) -> () {
    let x = JsFuture::from(p).await.unwrap();
    f(x);
}

#[function_component(App)]
pub fn app() -> html {
    let is_playing = use_state(|| VOICES.map(|_| false));
    let is_loop_mode = use_state(|| true);

    // Setup canvas
    let canvas_ref = use_node_ref();
    let canvas_ctx = use_mut_ref(|| None);
    {
        let canvas_ref = canvas_ref.clone();
        let canvas_ctx = canvas_ctx.clone();
        use_effect_with_deps(
            move |canvas_ref| {
                *canvas_ctx.borrow_mut() =
                    canvas_ref
                        .cast::<web_sys::HtmlCanvasElement>()
                        .map(|canvas| {
                            canvas.set_width(DIM);
                            canvas.set_height(DIM);

                            canvas
                                .get_context("2d")
                                .unwrap()
                                .unwrap()
                                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                                .unwrap()
                        });

                || {}
            },
            canvas_ref,
        );
    }

    // Setup waveform analyzer
    let audio_ctx =
        use_ref(|| web_sys::AudioContext::new().expect("failed to create AudioContext"));
    let analyzer = use_ref(|| {
        let analyzer = audio_ctx.create_analyser().unwrap();
        analyzer
            .connect_with_audio_node(&audio_ctx.destination())
            .unwrap();
        analyzer.set_fft_size(FFT_SIZE as u32);

        {
            let analyzer = analyzer.clone();

            let mut i = 0;
            let cb = Closure::<dyn FnMut(_)>::new(move |_: i32| {
                let mut arr: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
                analyzer.get_float_time_domain_data(&mut arr);

                if let Some(canvas_ctx) = &*canvas_ctx.borrow_mut() {
                    let canvas = canvas_ctx.canvas().unwrap();

                    // Params
                    let shift = -8.0;
                    let y_offset = DIM_F64 * 0.8;
                    let amp = DIM_F64 / 2.0 * 0.4;

                    // Feedback
                    canvas_ctx.set_fill_style(&JsValue::from_str("rgba(0, 0, 0, 0.015)"));
                    canvas_ctx.fill_rect(0.0, 0.0, DIM_F64, DIM_F64);
                    canvas_ctx.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&canvas, 0.0, 0.0, DIM_F64, DIM_F64 - shift, 0.0, shift, DIM_F64, DIM_F64 - shift).unwrap();

                    // Draw wave
                    if i % 2 == 0 {
                        canvas_ctx.set_stroke_style(&JsValue::from_str("white"));
                        canvas_ctx.set_line_width(3.0);

                        canvas_ctx.move_to(0.0, y_offset);
                        canvas_ctx.begin_path();

                        let mut volume = 0.0;

                        for (i, x) in arr.iter().enumerate() {
                            let wave = (i as f64 / arr.len() as f64 * PI).sin();
                            canvas_ctx
                                .line_to((i * 2) as f64, -x.abs() as f64 * amp * wave + y_offset);
                            volume += x * x;
                        }
                        canvas_ctx.move_to(DIM_F64, y_offset);
                        canvas_ctx.close_path();

                        if volume > 0.2 {
                            canvas_ctx.stroke();
                        }
                    }
                }

                i += 1;
            });

            web_sys::window()
                .unwrap()
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    30,
                )
                .unwrap();

            cb.forget();
        }

        analyzer
    });

    // Load audio files and connect them to the analyzer
    let audios = use_ref(|| {
        VOICES.map(|v| {
            let audio =
                web_sys::HtmlAudioElement::new_with_src(v.filename).expect("failed to load");
            audio.set_loop(true);

            let src = audio_ctx.create_media_element_source(&audio).unwrap();
            src.connect_with_audio_node(&*analyzer).unwrap();

            audio
        })
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
        let is_loop_mode = is_loop_mode.clone();

        Callback::from(move |(i, playing): (usize, bool)| {
            // log::info!(">> play {}", playing);
            if playing {
                audios[i].set_current_time(0.0);
                audios[i].set_loop(*is_loop_mode.to_owned());

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
        })
    };

    let toggle_loop_mode = {
        let is_loop_mode = is_loop_mode.clone();
        Callback::from(move |_| is_loop_mode.set(!*is_loop_mode))
    };

    // Setup BPM input
    let bpm = use_state(|| 120);
    let bpm_last = use_state(|| 120);
    let onchange_bpm = {
        let bpm = bpm.clone();
        let audios = audios.clone();

        Callback::from(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input = target.dyn_ref::<HtmlInputElement>().unwrap();
            if let Ok(v) = input.value().parse::<u32>() {
                let v = v.clamp(60, 240);
                bpm.set(v);
                bpm_last.set(v);

                let speed = v as f64 / 120.0;
                for a in audios.iter() {
                    a.set_playback_rate(speed);
                }
            } else {
                log::debug!("PARSE ERRRO");
                bpm.set(*bpm_last);
            }
        })
    };

    html! {
        <div class="app">
            <header>
                <div>
                    <h1>{"JOVI JOVI"}</h1>
                </div>
                <div>
                    <label>{"BPM"}</label>
                    <input type="number" step="1" value={bpm.to_string()} onchange={onchange_bpm.clone()} />
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
