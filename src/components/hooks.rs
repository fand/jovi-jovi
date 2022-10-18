use crate::models::voice::VOICES;
use gloo_net::http::Request;
use js_sys::ArrayBuffer;
use std::collections::HashMap;
use std::f64::consts::PI;
use wasm_bindgen::JsValue;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    AnalyserNode, AudioBuffer, AudioBufferSourceNode, AudioContext, CanvasRenderingContext2d,
};
use yew::{use_effect_with_deps, use_mut_ref, use_state_eq, Callback, NodeRef};

trait Assume {
    fn assume(&self, msg: &str);
}

impl<T> Assume for Option<T> {
    /// Similar to .expect but only shows the error message
    fn assume(&self, msg: &str) {
        if self.is_none() {
            log::warn!("Failed: {}", msg);
        }
    }
}

impl<T, E> Assume for Result<T, E> {
    fn assume(&self, msg: &str) {
        if self.is_err() {
            log::warn!("Failed: {}", msg);
        }
    }
}

const DIM: u32 = 1024;
const DIM_F64: f64 = DIM as f64;
const FFT_SIZE: usize = (DIM / 2) as usize;

fn get_canvas_context(canvas_ref: NodeRef) -> CanvasRenderingContext2d {
    let canvas = canvas_ref
        .cast::<web_sys::HtmlCanvasElement>()
        .expect("Failed to create HTMLCanvasElement");

    canvas.set_width(DIM);
    canvas.set_height(DIM);

    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("Failed to get canvas context")
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

struct SamplerState {
    blobs: Vec<Option<ArrayBuffer>>,
    bufs: Vec<Option<AudioBuffer>>,
    nodes: Vec<HashMap<usize, AudioBufferSourceNode>>,
    ctx: Option<AudioContext>,
    analyzer: Option<Box<AnalyserNode>>,
    speed: f64,
}

pub struct AudioSampler {
    pub init: Callback<NodeRef>,
    pub play: Callback<(usize, usize)>,
    pub pause: Callback<(usize, usize)>,
    pub pause_all: Callback<()>,
    pub set_speed: Callback<f64>,
    pub is_loaded: bool,
}

pub fn use_audio_sampler() -> AudioSampler {
    let state = use_mut_ref(|| SamplerState {
        ctx: None,
        analyzer: None,
        blobs: VOICES.map(|_| None).to_vec(),
        bufs: VOICES.map(|_| None).to_vec(),
        nodes: VOICES.map(|_| HashMap::new()).to_vec(),
        speed: 1.0,
    });
    let is_loaded = use_state_eq(|| false);

    // Load audio files
    {
        let state = state.clone();
        let is_loaded = is_loaded.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    for i in 0..VOICES.len() {
                        let v = &VOICES[i];
                        let res = Request::get(v.filename).send().await.unwrap();

                        let res = res.as_raw().array_buffer().unwrap();
                        let res = JsFuture::from(res).await.unwrap();
                        let res = res.dyn_into::<js_sys::ArrayBuffer>().unwrap();
                        (*state.borrow_mut()).blobs[i] = Some(res);

                        // Upload progres
                        is_loaded.set(state.borrow_mut().blobs.iter().all(|x| x.is_some()));
                    }
                });
                || {}
            },
            (),
        );
    }

    let init = {
        let state = state.clone();

        Callback::from(move |canvas_ref: NodeRef| {
            // Create AudioContext
            let audio_ctx = web_sys::AudioContext::new().expect("failed to create AudioContext");

            // Create AnalyserNode
            let analyzer = Box::new(audio_ctx.create_analyser().unwrap());
            analyzer
                .connect_with_audio_node(&audio_ctx.destination())
                .expect("Failed to connect AnalyserNode to the destination");
            analyzer.set_fft_size(FFT_SIZE as u32);

            // Decode audios and store them to audio_bufs
            {
                let audio_ctx = audio_ctx.clone();
                let audio = state.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let mut audio = audio.borrow_mut();

                    for i in 0..audio.blobs.len() {
                        if let Some(data) = &audio.blobs[i] {
                            let audio_buf = audio_ctx.decode_audio_data(data).unwrap();
                            let audio_buf = JsFuture::from(audio_buf).await.unwrap();

                            audio.bufs[i] =
                                Some(audio_buf.dyn_into::<web_sys::AudioBuffer>().unwrap());
                        }
                    }
                });
            }

            // Setup canvas loop
            let canvas_ctx = get_canvas_context(canvas_ref.clone());
            setup_canvas(canvas_ctx, analyzer.clone());

            let mut state = state.borrow_mut();
            state.ctx = Some(audio_ctx);
            state.analyzer = Some(analyzer);
        })
    };

    let play = {
        let state = state.clone();

        Callback::from(move |(i, count): (usize, usize)| {
            let mut audio = state.borrow_mut();

            if let Some(node) = audio.nodes[i].get(&count) {
                node.stop().assume("Stop AudioBufferSourceNode");
                node.disconnect().assume("Disconnect AudioBufferSourceNode");
            }

            if let (Some(audio_ctx), Some(buf), Some(analyzer)) =
                (&audio.ctx, &audio.bufs[i], &audio.analyzer)
            {
                let node = audio_ctx
                    .create_buffer_source()
                    .expect("Failed to create AudioBufferSourceNode");
                node.set_buffer(Some(&buf));
                node.set_loop(true);
                node.playback_rate().set_value(audio.speed as f32);
                node.connect_with_audio_node(&*analyzer).unwrap();
                node.start().expect("Failed to start AudioBufferSourceNode");

                audio.nodes[i].insert(count, node);
            }
        })
    };

    let pause = {
        let state = state.clone();

        Callback::from(move |(i, count): (usize, usize)| {
            let state = state.borrow_mut();
            if let Some(node) = &state.nodes[i].get(&count) {
                node.set_loop(false);
            }
        })
    };

    let pause_all = {
        let state = state.clone();

        Callback::from(move |_| {
            let state = state.borrow_mut();
            for nodes in &state.nodes {
                for (_, node) in nodes {
                    node.set_loop(false);
                }
            }
        })
    };

    let set_speed = {
        let state = state.clone();
        Callback::from(move |s: f64| {
            let mut audio = state.borrow_mut();
            audio.speed = s;
        })
    };

    AudioSampler {
        init,
        play,
        pause,
        pause_all,
        set_speed,
        is_loaded: *is_loaded,
    }
}
