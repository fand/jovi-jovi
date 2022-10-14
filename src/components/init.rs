use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
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

const MIN_BPM: u32 = 60;
const MAX_BPM: u32 = 180;

async fn then<F: Fn(JsValue) -> ()>(p: js_sys::Promise, f: F) -> () {
    let x = JsFuture::from(p).await.unwrap();
    f(x);
}

// Setup waveform analyzer
fn setup_canvas(canvas_ctx: Rc<RefCell<Option<CanvasRenderingContext2d>>>, analyzer: AnalyserNode) {
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
            canvas_ctx
                .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &canvas,
                    0.0,
                    0.0,
                    DIM_F64,
                    DIM_F64 - shift,
                    0.0,
                    shift,
                    DIM_F64,
                    DIM_F64 - shift,
                )
                .unwrap();

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
    // Setup canvas
    let canvas_ref = use_node_ref();
    let canvas_ctx = use_mut_ref(|| None);
    {
        let canvas_ref = canvas_ref.clone();
        let canvas_ctx = canvas_ctx.clone();
        use_effect_with_deps(
            move |canvas_ref| {
                let ctx = canvas_ref
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

                *canvas_ctx.borrow_mut() = ctx;

                || {}
            },
            canvas_ref,
        );
    }

    // Load audio files and connect them to the analyzer
    let audios = use_ref(|| {
        VOICES.map(|v| web_sys::HtmlAudioElement::new_with_src(v.filename).expect("failed to load"))
    });

    // Setup loading mask
    let is_loading = use_state(|| true);
    let onclick_mask = {
        let is_loading = is_loading.clone();
        let audios = audios.clone();
        let canvas_ctx = canvas_ctx.clone();

        Callback::from(move |_| {
            is_loading.set(!*is_loading);

            // Create AudioContext
            let audio_ctx = web_sys::AudioContext::new().expect("failed to create AudioContext");

            // Create AnalyserNode
            let analyzer = audio_ctx.create_analyser().unwrap();
            analyzer
                .connect_with_audio_node(&audio_ctx.destination())
                .unwrap();
            analyzer.set_fft_size(FFT_SIZE as u32);

            // Connect audio files to the analyzer
            for audio in audios.iter() {
                let src = audio_ctx.create_media_element_source(&audio).unwrap();
                src.connect_with_audio_node(&*analyzer).unwrap();
            }

            // Setup canvas loop
            setup_canvas(canvas_ctx.clone(), analyzer);
        })
    };

    html! {
        <div class="app">
            {if let Some(canvas_ctx) = canvas_ctx.borrow_mut().take() { html! {
                <App canvas_ctx={canvas_ctx}/>
            }} else { html!{
                <div class="mask" onclick={onclick_mask.clone()}>
                    <h1>{"JOVI JOVI"}</h1>
                    <button>{"PLAY"}</button>
                </div>
            } }}
            <canvas ref={canvas_ref}/>
        </div>
    }
}
