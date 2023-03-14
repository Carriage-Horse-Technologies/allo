use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct StageProps {}

#[function_component]
pub fn Stage(props: &StageProps) -> Html {
    let StageProps {} = props;
    html! {
        <div class="stage">
            <div class="top"></div>
            <div class="top-back"></div>
            <div class="bottom"></div>
            <div class="bottom-back"></div>

            <div class="side-group">
                <div></div><div></div><div></div><div></div><div></div>
                <div></div><div></div><div></div><div></div><div></div>
                <div></div><div></div><div></div><div></div><div></div>
                <div></div><div></div><div></div><div></div><div></div>
                <div></div><div></div><div></div><div></div><div></div>
                <div></div><div></div><div></div><div></div><div></div>
                <div></div><div></div><div></div><div></div><div></div>
                <div></div>
            </div>
        </div>
    }
}
