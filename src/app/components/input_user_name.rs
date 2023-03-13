use reqwasm::http::Request;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_async_with_options, use_local_storage, UseAsyncOptions};
use yewdux::prelude::use_store;

use crate::{
    app::states::Username,
    my_utils::{self, github_user_icon_url},
};

#[derive(PartialEq, Properties)]
pub(crate) struct InputUserNameProps {}

#[function_component]
pub(crate) fn InputUserName(props: &InputUserNameProps) -> Html {
    let InputUserNameProps {} = props;

    let input_node = use_node_ref();
    let (username, username_dispatch) = use_store::<Username>();

    let onsubmit = {
        let input_node = input_node.clone();
        let username_dispatch = username_dispatch.clone();
        Callback::from(move |e: SubmitEvent| {
            e.stop_propagation();
            e.prevent_default();
            let input_node = input_node
                .cast::<HtmlInputElement>()
                .expect("Failed to cast input_node");
            let username = input_node.value();
            username_dispatch.reduce(|_| Username(username).into());
            let win = web_sys::window().unwrap();
            win.location().set_href("/").unwrap();
        })
    };

    html! {
        <form onsubmit={onsubmit} class={classes!("flex", "flex-col", "container", "mx-auto", "items-center",
                            "my-3", "p-3")}>
            <label for="github_username">{"GitHub Username"}</label>
            <input ref={input_node} value={(*username).clone().0} type="text" id="github_username" name="github_username" autofocus={true} required={true} maxlength="16" size="16"
                title="must be alphanumeric in 6-12 chars"
                class={classes!("rounded-xl", "p-2", "m-3",
                        "dark:bg-dark-content-background", "dark:border-dark-button-border", "border-2",
                        )}/>
            <p>{""}</p>
            <button type="submit" class={classes!("flex", "flex-row", "items-center", "z-[901]",
            "w-[156px]", "h-[48px]",
            "rounded-full",
            "bg-enter-button", "text-4xl",
            "disabled:opacity-30", "disabled:bg-gray-600")}>
                <p class="mx-auto">{"Go!"}</p>
            </button>
        </form>
    }
}
