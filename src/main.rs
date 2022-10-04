// use wasm_bindgen::prelude::Closure;
// use web_sys::*;
use yew::prelude::*;

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
    name: String,
}

#[function_component(Button)]
fn button(ButtonProps { name }: &ButtonProps) -> html {
    let audio = web_sys::HtmlAudioElement::new_with_src("wav/joy.wav").expect("failed to load");

    let onmousedown = Callback::from(move|_| {
        let prom = audio.play().expect("failed to play");
        ()
    });

    let onmouseup = Callback::from(move |_| {
        audio.pause();

        ()
    });

    html! {
        <>
            <button type="button" onmousedown={onmousedown} onmouseup={onmouseup}>{ name }</button>
        </>
    }
}

#[function_component(App)]
fn app() -> html {
    let names = vec!["Joy Division", "Joy", "Divi", "John"];

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
            { names.iter().map(|name| html! { <Button name={name.to_string()}/> }).collect::<Html>() }
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
