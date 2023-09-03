mod path;
mod vector;
use std::fmt::Debug;

use leptos::*;
use path::*;
use vector::*;

fn main() {
    mount_to_body(|cx| view! {cx, <App />});
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx, <p>"Hello World"</p>}
}
