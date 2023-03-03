use crate::routes::Route;

use yew::prelude::*;
use yew_router::Switch;

use super::page::home::Home;

#[function_component]
pub fn Content() -> Html {
    let mousemove = {
        Callback::from(move |e: MouseEvent| {
            // log::info!("move! {},{}", e.client_x(), e.client_y());
        })
    };

    html!(
        <div class={"light"}>
            <div class={classes!("min-w-screen", "min-h-screen",
            "bg-light-background", "text-light-text",
            "dark:bg-dark-background", "dark:text-dark-text")}
            onmousemove={mousemove}>
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </div>
    )
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            let fallback = html! {<div>{"Loading..."}</div>};
            html! {
                <Suspense {fallback}>
                 <Home />
                </Suspense>
            }
        }
        Route::NotFound => {
            html! { <h1>{"NotFound"}</h1> }
        }
    }
}
