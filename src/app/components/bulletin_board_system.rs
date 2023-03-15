use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::app::states::BBSModalState;

#[derive(PartialEq, Properties)]
pub struct BBSProps {
    pub(crate) classes: Option<Classes>,
}

#[function_component]
pub fn BBS(props: &BBSProps) -> Html {
    let BBSProps { classes } = props;
    let (bbs_modal_state, bbs_modal_state_dispatch) = use_store::<BBSModalState>();

    let onclick = {
        let bbs_modal_state_dispatch = bbs_modal_state_dispatch.clone();
        Callback::from(move |_| {
            bbs_modal_state_dispatch.reduce(|_| BBSModalState(true).into());
        })
    };

    html! {
        <button type="button" onclick={onclick} class={classes!(classes)}>
            <img src="https://dx-lord.com/wp-content/uploads/2021/05/draw-io.png" width="64" height="64" alt="draw-io" />
        </button>
    }
}
