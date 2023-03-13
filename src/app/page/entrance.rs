use yew::prelude::*;

use crate::app::components::input_user_name::InputUserName;

#[derive(PartialEq, Properties)]
pub(crate) struct EntranceProps {}

#[function_component]
pub(crate) fn Entrance(props: &EntranceProps) -> Html {
    let EntranceProps {} = props;
    html! {
        <div class="pt-[100px] w-screen h-screen dark:bg-dark-content-background">
            <InputUserName />
        </div>
    }
}
