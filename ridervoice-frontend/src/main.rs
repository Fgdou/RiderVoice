use crate::components::map_component::MapComponent;
use yew::prelude::*;
mod components;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <MapComponent />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
