use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct EnterButtonProps {}

#[function_component]
pub fn EnterButton(props: &EnterButtonProps) -> Html {
    let EnterButtonProps {} = props;
    html! {
        <a href="http://" target="_blank" rel="noopener noreferrer"
            class="fixed container mx-auto flex flex-row items-center z-901
            w-[256px] h-[64px] bottom-[50px] left-1/2 -translate-x-1/2
            rounded-full
            bg-enter-button text-5xl">
            <p class="mx-auto">{"Enter!"}</p>
        </a>
    }
}
