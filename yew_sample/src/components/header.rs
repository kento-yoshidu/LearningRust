use yew::{function_component, html, Html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav>
            <div>
                <a class="navbar-brand" href="#">{"Yew Todo App"}</a>
            </div>
        </nav>
    }
}
