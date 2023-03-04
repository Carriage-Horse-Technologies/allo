use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct EnterButtonProps {
    href: Option<String>,
    disabled: Option<bool>,
}

#[function_component]
pub fn EnterButton(props: &EnterButtonProps) -> Html {
    let EnterButtonProps { href, disabled } = props;

    let onclick = {
        let href = href.clone().unwrap_or_default();
        Callback::from(move |_| {
            let win = web_sys::window().unwrap();
            win.open_with_url(&href).unwrap();
        })
    };

    html! {
        <button type="button" onclick={onclick}
            disabled={disabled.unwrap_or_else(|| true)}
            class="fixed flex flex-row items-center z-901
            w-[256px] h-[64px] bottom-[50px] left-1/2 -translate-x-1/2
            rounded-full
            bg-enter-button text-5xl
            disabled:opacity-30 disabled:bg-gray-600">
            <p class="mx-auto">{"Enter!"}</p>
        </button>
    }
}
