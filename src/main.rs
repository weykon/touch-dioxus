#![allow(unused)]
use dioxus::prelude::{rsx, use_state, Element, Scope};

use dioxus::events::*;
use dioxus::prelude::*;

fn main() {
    fn app(cx: Scope) -> Element {
        let mut count = use_state(&cx, || 0);

        cx.render(rsx!(
            div {
                h1 { "This is my first rust Frontend program" }
                button { onclick: move |_| count += 1, "Up high!" }
                button { onclick: move |_| count -= 1, "Down low!" }
                
            }
        ))
    };

    dioxus::desktop::launch(app);
}