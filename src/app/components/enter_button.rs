use yew::prelude::*;

use yewdux::prelude::use_store_value;

use crate::app::states::CollisionState;

#[derive(PartialEq, Properties)]
pub(crate) struct EnterButtonProps {
    pub(crate) href: Option<String>,
    pub(crate) disabled: Option<bool>,
}

#[function_component]
pub(crate) fn EnterButton(props: &EnterButtonProps) -> Html {
    let EnterButtonProps { href, disabled } = props;

    let collision_state = use_store_value::<CollisionState>();

    let onclick = {
        let href = href.clone().unwrap_or_default();
        let disabled = (*disabled).unwrap_or(true);
        let collision_state = collision_state.clone();
        Callback::from(move |_| {
            // タグを内包してしまっていることが原因でdisabledでも発火してしまうのでこっちで抑制
            if !disabled {
                let win = web_sys::window().unwrap();
                win.open_with_url(&href).unwrap();
            }
            if collision_state.on_collision_stay {
                let win = web_sys::window().unwrap();
                win.open_with_url(&collision_state.url).unwrap();
            }
        })
    };

    html! {
        <button type="button" onclick={onclick}
            disabled={!collision_state.on_collision_stay}
            class="fixed flex flex-row items-center z-901
            w-[256px] h-[64px] bottom-[50px] left-1/2 -translate-x-1/2
            rounded-full
            bg-enter-button text-5xl
            disabled:opacity-30 disabled:bg-gray-600">
            <p class="mx-auto">{"Enter!"}</p>
        </button>
    }
}
