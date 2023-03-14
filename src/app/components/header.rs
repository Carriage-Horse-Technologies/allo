use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HeaderProps {}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    let HeaderProps {} = props;
    html! {
        <header class="fixed w-screen mx-auto p-5 text-6xl text-center text-dark-text font-bold
            select-none dark:bg-dark-primary
            dark:bg-opacity-100 z-[890]">
            {"ハッカソン成果物展示場"}
        </header>
    }
}
