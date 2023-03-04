pub(crate) mod components;
pub(crate) mod content;
pub(crate) mod models;
pub(crate) mod page;
pub(crate) mod states;

use futures::{SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use yew::prelude::*;
use yew_hooks::{
    use_async_with_options, use_list, use_websocket_with_options, UseAsyncOptions,
    UseWebSocketOptions, UseWebSocketReadyState,
};
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
