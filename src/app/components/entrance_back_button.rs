use yew::prelude::*;
use yew_router::prelude::Link;

use crate::routes::Route;

#[derive(PartialEq, Properties)]
pub struct EntranceBackButtonProps {}

#[function_component]
pub fn EntranceBackButton(props: &EntranceBackButtonProps) -> Html {
    let EntranceBackButtonProps {} = props;
    html! {
        <Link<Route> classes={classes!("fixed", "flex", "justify-center", "items-center", "z-[901]",
        "bg-light-button-bg", "text-light-text", "text-xl",
        "rounded-full", "w-[156px]", "h-[48px]", "bottom-[50px]", "left-[10vw]",
        "hover:bg-light-button-bg-hover")}
        to={Route::Entrance}>
            <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-back" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                <path d="M9 11l-4 4l4 4m-4 -4h11a4 4 0 0 0 0 -8h-1"></path>
            </svg>
            <p class={classes!("pl-2")}>{"Entrance"}</p>
        </Link<Route>>
    }
}
