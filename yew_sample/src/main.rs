use yew::prelude::*;
use components::header::Header;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <h1>{ "Hello World" }</h1>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
