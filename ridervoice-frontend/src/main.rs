use crate::components::map_component::*;
use gloo_console::log;
use yew::prelude::*;
mod components;

#[function_component]
fn App() -> Html {

    let click_callback = use_callback((), |_e, ()| log!("Click"));

    html! {
        <>
            <MapComponent click_callback={click_callback} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
