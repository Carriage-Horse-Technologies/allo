use crate::routes::Route;

use yew::prelude::*;

use yew_router::Switch;

use super::{components::header::Header, page::home::Home};

#[function_component]
pub fn Content() -> Html {
    html!(
        <div class={"dark"}>
            <div class={classes!("min-w-screen", "min-h-screen",
            "bg-light-background", "text-light-text",
            "dark:bg-dark-background", "dark:text-dark-text")}>
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
                    <Header />
                    <Home />
                </Suspense>
            }
        }
        Route::NotFound => {
            html! { <h1>{"NotFound"}</h1> }
        }
    }
}
