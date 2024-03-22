use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <h1 class="text-3xl font-bold underline">
            { "Hello from the Frontend!" }
        </h1>
    }
}
