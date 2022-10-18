use std::collections::HashSet;
use std::rc::Rc;

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::button::Button;
use crate::models::voice::VOICES;
use crate::utils::{window_add_event_listener, window_remove_event_listener};

const MIN_BPM: u32 = 60;
const MAX_BPM: u32 = 180;

enum AppAction {
    Play(usize, usize),
    Pause(usize, usize),
    PauseAll,
}
struct AppState {
    is_playing: [HashSet<usize>; 6],
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            is_playing: VOICES.map(|_| HashSet::new()),
        }
    }
}
impl Reducible for AppState {
    type Action = AppAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next = match action {
            AppAction::Play(i, id) => {
                let mut a = self.is_playing.clone();
                a[i].insert(id);
                a
            }
            AppAction::Pause(i, id) => {
                let mut a = self.is_playing.clone();
                a[i].remove(&id);
                a
            }
            AppAction::PauseAll => VOICES.map(|_| HashSet::new()),
        };

        Self { is_playing: next }.into()
    }
}

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub play: Callback<(usize, usize)>,
    pub pause: Callback<(usize, usize)>,
    pub pause_all: Callback<()>,
    pub set_speed: Callback<f64>,
}

#[function_component(App)]
pub fn app(
    AppProps {
        play,
        pause,
        pause_all,
        set_speed,
    }: &AppProps,
) -> html {
    let state = use_reducer(AppState::default);

    let bpm = use_state(|| 120);
    let is_changing_bpm = use_state(|| false);

    // Register mouse up handler
    let onmouseup = {
        let pause_all = pause_all.clone();
        let state = state.clone();

        Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {
            log::info!("mouseup");
            pause_all.emit(());
            state.dispatch(AppAction::PauseAll);
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
        let state = state.clone();
        let is_changing_bpm = is_changing_bpm.clone();

        let play = play.clone();
        let pause = pause.clone();

        Callback::from(move |(i, count, playing): (usize, usize, bool)| {
            if playing {
                play.emit((i, count));
                state.dispatch(AppAction::Play(i, count));
            } else {
                pause.emit((i, count));
                state.dispatch(AppAction::Pause(i, count));
            }

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
                { VOICES.iter().enumerate().map(|(i, voice)| html! { <Button voice={voice.clone()} onchange={onchange.clone()} is_playing={!state.is_playing[i].is_empty()}/> }).collect::<Html>() }
            </div>
        </>
    }
}
