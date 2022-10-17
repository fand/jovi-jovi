use super::hooks::use_audio_sampler;
use crate::components::app::App;
use yew::prelude::*;

#[function_component(Init)]
pub fn init() -> html {
    let canvas_ref = use_node_ref();
    let sampler = use_audio_sampler();

    // Setup loading mask
    let is_loading = use_state(|| true);
    let onclick_mask = {
        let is_loading = is_loading.clone();
        let canvas_ref = canvas_ref.clone();

        Callback::from(move |_| {
            is_loading.set(!*is_loading);
            sampler.init.emit(canvas_ref.clone());
        })
    };

    html! {
        <div class="app">
            {if !*is_loading { html! {
                <App play={sampler.play.clone()} pause={sampler.pause.clone()} set_speed={sampler.set_speed.clone()}/>
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
