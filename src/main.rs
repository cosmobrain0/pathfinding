mod path;
mod vector;
use std::fmt::Debug;

use leptos::*;
use path::*;
use vector::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|cx| view! {cx, <App />});
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let nodes: Vec<_> = vec![vec2d![0.0, 0.0], vec2d![10.0, 20.0]];
    let nodes_display = nodes
        .iter()
        .map(|pos| {
            view! {cx,
                <div class:node=true style={format!("left = {}px; top = {}px", pos.x, pos.y)}>
                </div>
            }
        })
        .collect_view(cx);
    let connections: Vec<Connection> = vec![0, 1]
        .chunks_exact(2)
        .map(|x| (x[0], x[1]))
        .map(Into::into)
        .collect();
    let mut pathfinder = Pathfinder::new(nodes, connections).unwrap();
    let path = pathfinder.pathfind(NodeIndex(0), NodeIndex(1));
    log!(
        "{:#?}",
        path.iter().map(|x| x.position()).collect::<Vec<_>>()
    );

    view! {cx,
        <link rel="stylesheet" src="style.css" />
        {nodes_display}
    }
}
