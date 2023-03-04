use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HeaderProps {}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    let HeaderProps {} = props;
    html! {
        <header class="fixed w-screen mx-auto p-5 text-6xl text-center
            dark:bg-dark-primary">
            {"ハッカソン成果物展示場"}
        </header>
    }
}
