pub(crate) mod components;
pub(crate) mod content;
pub(crate) mod models;
pub(crate) mod page;
pub(crate) mod states;

use yew::prelude::*;

use yew_router::BrowserRouter;

use crate::app::content::Content;

#[function_component]
pub fn App() -> Html {
    let fallback = html! {<div>{"loading..."}</div>};

    html! {
        <BrowserRouter>
            <Suspense fallback={fallback}>
                <Content />
            </Suspense>
        </BrowserRouter>
    }
}
