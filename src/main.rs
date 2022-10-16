mod components;
mod models;
mod utils;

use components::init::Init;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Init>();
}
