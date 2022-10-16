use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::HtmlElement;

pub fn window_add_event_listener<T>(type_: &str, callback: &Closure<dyn FnMut(T)>) {
    let window = web_sys::window().expect("Failed to get Window");
    window
        .add_event_listener_with_callback(type_, callback.as_ref().unchecked_ref())
        .expect("addEventListener failed");
}

pub fn window_remove_event_listener<T>(type_: &str, callback: &Closure<dyn FnMut(T)>) {
    let window = web_sys::window().expect("Failed to get Window");
    window
        .remove_event_listener_with_callback(type_, callback.as_ref().unchecked_ref())
        .expect("removeEventListener failed");
}

pub fn el_add_event_listener<T>(el: &HtmlElement, type_: &str, callback: &Closure<dyn Fn(T)>) {
    el.add_event_listener_with_callback(type_, callback.as_ref().unchecked_ref())
        .expect("addEventListener failed");
}

pub fn el_remove_event_listener<T>(el: &HtmlElement, type_: &str, callback: &Closure<dyn Fn(T)>) {
    el.remove_event_listener_with_callback(type_, callback.as_ref().unchecked_ref())
        .expect("removeEventListener failed");
}
