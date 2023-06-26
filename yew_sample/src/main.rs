use yew::prelude::*;
use components::header::Header;

use stylist::{css, style};
use stylist::yew::{styled_component, Global};

mod components;

#[function_component(App)]
fn app() -> Html {
    let global_style = css!(r#"
        html {
            color: red;
        }
    "#);

    html! {
        <>
            <Global css={global_style} />

            <Header />
            <h1>{ "Hello World" }</h1>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
