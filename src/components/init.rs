use gloo_net::http::Request;
use std::f64::consts::PI;
use wasm_bindgen::JsValue;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{AnalyserNode, CanvasRenderingContext2d};
use yew::prelude::*;

use crate::components::app::App;
use crate::models::voice::VOICES;

const DIM: u32 = 1024;
const DIM_F64: f64 = DIM as f64;
const FFT_SIZE: usize = (DIM / 2) as usize;

fn get_canvas_context(canvas_ref: NodeRef) -> CanvasRenderingContext2d {
    let canvas = canvas_ref.cast::<web_sys::HtmlCanvasElement>().unwrap();

    canvas.set_width(DIM);
    canvas.set_height(DIM);

    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

// Setup waveform analyzer
fn setup_canvas(canvas_ctx: CanvasRenderingContext2d, analyzer: Box<AnalyserNode>) {
    let mut i = 0;
    let cb = Closure::<dyn FnMut(_)>::new(move |_: i32| {
        let mut arr: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
        analyzer.get_float_time_domain_data(&mut arr);

        // Params
        let shift = -8.0;
        let y_offset = DIM_F64 * 0.8;
        let amp = DIM_F64 / 2.0 * 0.4;

        // Feedback
        canvas_ctx.set_fill_style(&JsValue::from_str("rgba(0, 0, 0, 0.015)"));
        canvas_ctx.fill_rect(0.0, 0.0, DIM_F64, DIM_F64);
        let prev = canvas_ctx
            .get_image_data(0.0, 0.0, DIM_F64, DIM_F64)
            .unwrap();
        canvas_ctx.put_image_data(&prev, 0.0, shift).unwrap();

        // Draw wave
        if i % 2 == 0 {
            canvas_ctx.set_stroke_style(&JsValue::from_str("white"));
            canvas_ctx.set_line_width(3.0);

            canvas_ctx.move_to(0.0, y_offset);
            canvas_ctx.begin_path();

            let mut volume = 0.0;

            for (i, x) in arr.iter().enumerate() {
                let wave = (i as f64 / arr.len() as f64 * PI).sin();
                canvas_ctx.line_to((i * 2) as f64, -x.abs() as f64 * amp * wave + y_offset);
                volume += x * x;
            }
            canvas_ctx.move_to(DIM_F64, y_offset);
            canvas_ctx.close_path();

            if volume > 0.2 {
                canvas_ctx.stroke();
            }
        }

        i += 1;
    });

    web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 30)
        .unwrap();

    cb.forget();
}

#[function_component(Init)]
pub fn init() -> html {
    let canvas_ref = use_node_ref();

    let audio_blobs = use_mut_ref(|| VOICES.map(|_| None));
    let audio_bufs = use_mut_ref(|| VOICES.map(|_| None));
    let audio_nodes = use_mut_ref(|| VOICES.map(|_| None));
    let audio_ctx_ref = use_mut_ref(|| None);
    let analyzer_ref = use_mut_ref(|| None);
    let speed = use_mut_ref(|| 1.0 as f64);

    // Load audio files
    {
        let audio_blobs = audio_blobs.clone();
        use_effect_with_deps(
            move |_| {
                let audio_blobs = audio_blobs.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    for i in 0..VOICES.len() {
                        let v = &VOICES[i];
                        let res = Request::get(v.filename).send().await.unwrap();

                        let res = res.as_raw().array_buffer().unwrap();
                        let res = JsFuture::from(res).await.unwrap();
                        let res = res.dyn_into::<js_sys::ArrayBuffer>().unwrap();
                        (*audio_blobs.borrow_mut())[i] = Some(res);
                    }
                });

                || {}
            },
            (),
        );
    }

    // Setup loading mask
    let is_loading = use_state(|| true);
    let onclick_mask = {
        let is_loading = is_loading.clone();

        let canvas_ref = canvas_ref.clone();

        let audio_ctx_ref = audio_ctx_ref.clone();
        let analyzer_ref = analyzer_ref.clone();
        let audio_bufs = audio_bufs.clone();

        Callback::from(move |_| {
            is_loading.set(!*is_loading);

            // Create AudioContext
            let audio_ctx = web_sys::AudioContext::new().expect("failed to create AudioContext");

            // Create AnalyserNode
            let analyzer = Box::new(audio_ctx.create_analyser().unwrap());
            analyzer
                .connect_with_audio_node(&audio_ctx.destination())
                .unwrap();
            analyzer.set_fft_size(FFT_SIZE as u32);

            // Decode audios and store them to audio_bufs
            let audio_blobs = audio_blobs.clone();
            let audio_bufs = audio_bufs.clone();
            {
                let audio_ctx = audio_ctx.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let audio_ctx = audio_ctx.clone();
                    let blobs = audio_blobs.borrow_mut();
                    let mut bufs = audio_bufs.borrow_mut();
                    for i in 0..blobs.len() {
                        let blob = &blobs[i];
                        if let Some(data) = blob {
                            let audio_buf = audio_ctx.decode_audio_data(data).unwrap();
                            let audio_buf = JsFuture::from(audio_buf).await.unwrap();

                            bufs[i] = Some(audio_buf.dyn_into::<web_sys::AudioBuffer>().unwrap());
                        }
                    }
                });
            }

            // Setup canvas loop
            let canvas_ctx = get_canvas_context(canvas_ref.clone());
            setup_canvas(canvas_ctx, analyzer.clone());

            *audio_ctx_ref.borrow_mut() = Some(audio_ctx);
            *analyzer_ref.borrow_mut() = Some(analyzer);
        })
    };

    let play = {
        let audio_ctx_ref = audio_ctx_ref.clone();
        let analyzer_ref = analyzer_ref.clone();
        let audio_bufs = audio_bufs.clone();
        let audio_nodes = audio_nodes.clone();
        let speed = speed.clone();

        Callback::from(move |i: usize| {
            let audio_ctx = audio_ctx_ref.borrow_mut();
            let analyzer = analyzer_ref.borrow_mut();
            let bufs = audio_bufs.borrow_mut();
            let mut nodes = audio_nodes.borrow_mut();

            if let (Some(audio_ctx), Some(buf), Some(analyzer)) =
                (&*audio_ctx, &bufs[i], &*analyzer)
            {
                let node = audio_ctx.create_buffer_source().unwrap();
                node.set_buffer(Some(&buf));
                node.set_loop(true);
                node.playback_rate().set_value(*speed.borrow() as f32);
                node.connect_with_audio_node(&*analyzer).unwrap();
                node.start().unwrap();

                (*nodes)[i] = Some(node);
            }
        })
    };

    let pause = {
        let audio_nodes = audio_nodes.clone();

        Callback::from(move |i: usize| {
            let nodes = audio_nodes.borrow_mut();
            if let Some(node) = &nodes[i] {
                node.set_loop(false);
            }
        })
    };

    let set_speed = {
        Callback::from(move |s: f64| {
            *speed.borrow_mut() = s;
        })
    };

    html! {
        <div class="app">
            {if !*is_loading { html! {
                <App play={play.clone()} pause={pause.clone()} set_speed={set_speed.clone()}/>
            }} else { html! {
                <div class="mask" onclick={onclick_mask.clone()}>
                    <h1>{"JOVI JOVI"}</h1>
                    <button>{"PLAY"}</button>
                </div>
            }}}
            <canvas ref={canvas_ref}/>
        </div>
    }
}
