use super::hooks::use_audio_sampler;
use crate::components::app::App;
use yew::prelude::*;

#[function_component(Init)]
pub fn init() -> html {
    let canvas_ref = use_node_ref();
    let sampler = use_audio_sampler();
    let is_playing = use_state(|| false);

    // Setup loading mask
    let onclick_mask = {
        let is_playing = is_playing.clone();
        let canvas_ref = canvas_ref.clone();

        Callback::from(move |_| {
            if sampler.is_loaded {
                is_playing.set(true);
                sampler.init.emit(canvas_ref.clone());
            }
        })
    };

    html! {
        <div class="app">
            {if *is_playing { html! {
                <App play={sampler.play.clone()} pause={sampler.pause.clone()} set_speed={sampler.set_speed.clone()}/>
            }} else { html! {
                <div class="mask" onclick={onclick_mask.clone()}>
                    <h1>{"JOVI JOVI"}</h1>
                    {if sampler.is_loaded { html! {
                        <button>{"PLAY"}</button>
                    }} else { html! {
                        <button class="loading">{"Loading..."}</button>
                    }}}
                </div>
            }}}
            <canvas ref={canvas_ref}/>
        </div>
    }
}
