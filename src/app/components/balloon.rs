use yew::prelude::*;
use yew_hooks::{use_bool_toggle, use_timeout};

#[derive(PartialEq, Properties)]
pub(crate) struct BalloonProps {
    pub(crate) node_ref: NodeRef,
    pub(crate) is_display_balloon: bool,
    pub(crate) children: Children,
}

#[function_component]
pub(crate) fn Balloon(props: &BalloonProps) -> Html {
    let BalloonProps {
        node_ref,
        is_display_balloon,
        children,
    } = props;

    html! {
        <div ref={node_ref} class={classes!(
                    if *is_display_balloon { "" } else { "hidden" },
                    "absolute", "select-none", "p-2", "bg-red-500",
                    "rounded-xl",
                    "-top-[155px]", "-left-[100px]",
                    "w-[200px]", "h-[100px]",
                    "transform-gpu", "translate-x-[50vw]", "translate-y-[50vh]",
                    "z-900", "ease-out", "duration-200",
                    "after:content-['']",
                    "after:absolute",
                    "after:w-0",
                    "after:h-0",
                    "after:left-0",
                    "after:right-0",
                    "after:border-solid",
                    "after:border-t-red-500",
                    "after:border-x-[transparent]",
                    "after:border-b-[transparent]",
                    "after:border-x-[20px]",
                    "after:border-t-[20px]",
                    "after:border-b-[0px]",
                    "after:m-auto",
                    "after:-bottom-[20px]"
        )}>
            if *is_display_balloon {
                <div class={classes!("w-full", "h-full", "overflow-auto")}>
                {
                    for children.iter()
                }
                </div>
            }
        </div>
    }
}
