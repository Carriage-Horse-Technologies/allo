use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub(crate) struct StageProps {
    pub(crate) classes: Option<Classes>,
}

#[function_component]
pub(crate) fn Stage(props: &StageProps) -> Html {
    let StageProps { classes } = props;
    html! {
        <div class={classes!(classes)}>
            <div class={classes!("stage")}>
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
        </div>
    }
}
