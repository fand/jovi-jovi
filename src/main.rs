use wasm_bindgen::{prelude::Closure, JsCast};
// use wasm_bindgen::prelude::Closure;
// use web_sys::*;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
enum Voice {
    JoyDivision,
    Joy,
    Divi,
    John,
}

impl Voice {
    pub fn voices() -> Vec<Voice> {
        vec![Voice::JoyDivision, Voice::Joy, Voice::Divi, Voice::John]
    }
    pub fn name(&self) -> String {
        match self {
            Voice::JoyDivision => "Joy Division".to_string(),
            Voice::Joy => "Joy".to_string(),
            Voice::Divi => "Divi".to_string(),
            Voice::John => "John".to_string(),
        }
    }
}

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
struct ButtonProps {
    voice: Voice,
    onchange: Callback<bool>,
}

#[function_component(Button)]
fn button(ButtonProps { voice, onchange }: &ButtonProps) -> html {
    let onmousedown = {
        let onchange = onchange.clone();
        Callback::from(move |_| {
            // let prom = audio.play().expect("failed to play");
            onchange.emit(true);
            ()
        })
    };

    let onmouseup = {
        let onchange = onchange.clone();
        Callback::from(move |_| {
            // audio.pause();
            onchange.emit(false);
            ()
        })
    };

    html! {
        <>
            <button type="button" onmousedown={onmousedown} onmouseup={onmouseup}>{ voice.name() }</button>
        </>
    }
}

#[function_component(App)]
fn app() -> html {
    let voices = Voice::voices();

    let is_playing = use_state(|| false);

    let audio = use_ref(|| {
        let audio = web_sys::HtmlAudioElement::new_with_src("wav/joy.wav").expect("failed to load");
        audio.set_loop(true);
        audio
    });

    let onmouseup = {
        let audio = audio.clone();
        Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {
            log::info!("mouseup");
            audio.pause().expect("failed to pause");
            audio.set_current_time(0.0);
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

    use_effect_with_deps(
        move |is_playing| {
            if *is_playing {
                audio.play().expect("failed to play");
            }
            || {}
        },
        *is_playing,
    );

    let onchange = Callback::from(move |playing: bool| is_playing.set(playing));

    html! {
        <>
            <header>
                <div>
                    <h1>{"JOVI JOVI"}</h1>
                </div>
                <div>
                    <button>{"POLY"}</button>
                    <button>{"LOOP"}</button>
                </div>
            </header>
            <div class="buttons">
            { voices.iter().map(|voice| html! { <Button voice={voice.clone()} onchange={onchange.clone()}/> }).collect::<Html>() }
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
