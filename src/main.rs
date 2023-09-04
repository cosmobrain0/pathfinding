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

    let connections: Vec<Connection> = vec![0, 1]
        .chunks_exact(2)
        .map(|x| (x[0], x[1]))
        .map(Into::into)
        .collect();
    let mut pathfinder = Pathfinder::new(&nodes, connections).unwrap();
    let path = pathfinder.pathfind(NodeIndex(0), NodeIndex(1));
    log!(
        "{:#?}",
        path.iter().map(|x| x.position()).collect::<Vec<_>>()
    );

    let nodes_display = nodes
        .iter()
        .map(|pos| {
            let basic_styles = "display: inline-block; position: absolute; width: 5px; height: 5px;";
            let position_style = format!("left: {}px; top: {}px;", pos.x, pos.y);
            let path_index = path.iter().enumerate().find(|(_, node)| node.position() == *pos).map(|(index, _)| index);
            let path_style = if let Some(path_index) = path_index {
                let percentage = path_index as f32 / (path.len()-1) as f32; // stuff goes horribly wrong if there's only one node in the path
                const PATH_START: [f32; 3] = [26.0, 108.0, 240.0];
                const PATH_END: [f32; 3] = [22.0, 193.0, 219.0];
                let lerp = |a, b, t| a + (b-a) * t;
                let colour: Vec<f32> = (0..3).map(|i| lerp(PATH_START[i], PATH_END[i], percentage)).collect::<Vec<_>>(); // maybe find a way to get an array?
                format!("background: rgb({}, {}, {});", colour[0], colour[1], colour[2])
            } else {
                "background: #000".into()
            };

            view! {cx,
                <div class:node=true style={format!("{basic_styles} {position_style} {path_style}")}>
                </div>
            }
        })
        .collect_view(cx);

    view! {cx,
        <link data-trunk rel="css" href="style.css" />
        {nodes_display}
    }
}
