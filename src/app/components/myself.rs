use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::use_bool_toggle;

use crate::my_utils::px_to_tws;

#[derive(PartialEq, Properties)]
pub(crate) struct MyselfProps {}

#[function_component]
pub(crate) fn Myself(props: &MyselfProps) -> Html {
    let MyselfProps {} = props;

    let pos = use_state(|| (0, 0));
    let myself = use_node_ref();
    let is_active = use_bool_toggle(false);

    {
        let pos = pos.clone();
        let myself = myself.clone();
        let is_active = is_active.clone();
        use_effect_with_deps(
            |(myself, is_active)| {
                let document = web_sys::window().unwrap().document().unwrap();
                let mousemove_listener = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new({
                    let myself = myself.clone();
                    let is_active = is_active.clone();
                    move |e| {
                        if *is_active {
                            log::debug!("move! {},{}", e.client_x(), e.client_y());
                            pos.set((e.client_x(), e.client_y()));
                            let div = myself.cast::<HtmlElement>().unwrap();
                            let style = div.style();
                            style
                                .set_property(
                                    "transform",
                                    &format!("translate({}px, {}px)", e.client_x(), e.client_y()),
                                )
                                .unwrap();
                        }
                    }
                }));

                let mouseup_listener = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new({
                    let is_active = is_active.clone();
                    move |_| {
                        log::debug!("on disactive");
                        is_active.set(false);
                    }
                }));

                let register_listener = move || {
                    document
                        .add_event_listener_with_callback(
                            "mousemove",
                            mousemove_listener.as_ref().unchecked_ref(),
                        )
                        .unwrap();

                    document
                        .add_event_listener_with_callback(
                            "mouseup",
                            mouseup_listener.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                };
                register_listener();

                register_listener
            },
            (myself, is_active),
        );
    }

    let onmousedown = {
        let is_active = is_active.clone();
        Callback::from(move |_| {
            log::debug!("on active");
            is_active.set(true);
        })
    };

    html! {
        <div ref={myself} onmousedown={onmousedown}
        class={classes!("fixed", "select-none",
                "-top-[32px]", "-left-[32px]",
                "w-[64px]", "h-[64px]",
                "rounded-full",
                "transform-gpu", "translate-x-[50vw]", "translate-y-[50vh]",
                "z-900", "ease-out", "duration-200",
                "overflow-hidden"
        )}
            id="myself" >
            <img src="https://avatars.githubusercontent.com/u/40430090?s=400&u=3833aeb5ec8671c98d415b620b5e6a65cfb0d6d2&v=4" width="64" alt="myself" />
        </div>
    }
}
