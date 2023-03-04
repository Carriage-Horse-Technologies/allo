use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{HtmlElement, WebSocket};
use yew::prelude::*;
use yew_hooks::{use_bool_toggle, use_websocket, UseWebSocketHandle};

use crate::{app::models::Character, my_utils::px_to_tws};

#[derive(PartialEq, Properties)]
pub(crate) struct MyselfProps {
    pub(crate) ws: Rc<RefCell<Option<WebSocket>>>,
}

#[function_component]
pub(crate) fn Myself(props: &MyselfProps) -> Html {
    let MyselfProps { ws } = props;

    let myself = use_node_ref();
    let is_active = use_bool_toggle(false);

    {
        let myself = myself.clone();
        let is_active = is_active.clone();
        use_effect_with_deps(
            |(myself, is_active)| {
                let document = web_sys::window().unwrap().document().unwrap();

                // マウス移動時
                let mousemove_listener = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new({
                    let myself = myself.clone();
                    let is_active = is_active.clone();
                    move |e| {
                        if *is_active {
                            log::debug!("move! {},{}", e.page_x(), e.page_y());
                            let div = myself.cast::<HtmlElement>().unwrap();
                            let style = div.style();
                            style
                                .set_property(
                                    "transform",
                                    &format!("translate({}px, {}px)", e.page_x(), e.page_y()),
                                )
                                .unwrap();
                        }
                    }
                }));

                // マウスボタンが離された時
                let mouseup_listener = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new({
                    let is_active = is_active.clone();
                    move |e| {
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

    // Iconが押された時
    let onmousedown = {
        let is_active = is_active.clone();
        let ws = ws.clone();
        Callback::from(move |event: MouseEvent| {
            log::debug!("on active");
            is_active.set(true);
            let my_pos = Character {
                pos_x: event.page_x() as f64,
                pos_y: event.page_y() as f64,
                ..Default::default()
            };
            if let Err(e) = ws
                .borrow()
                .as_ref()
                .unwrap()
                .send_with_str(&serde_json::to_string(&my_pos).unwrap())
            {
                log::error!(
                    "Failed to WebSocket send error. {}",
                    e.as_string().unwrap_or_default()
                );
            }
        })
    };

    html! {
        <div ref={myself} onmousedown={onmousedown}
        class={classes!("absolute", "select-none",
                "-top-[32px]", "-left-[32px]",
                "w-[64px]", "h-[64px]",
                "rounded-full",
                "transform-gpu", "translate-x-[50vw]", "translate-y-[50vh]",
                "z-900", "ease-out", "duration-200",
                "overflow-hidden"
        )}
            id="myself" >
            <img src="https://avatars.githubusercontent.com/u/40430090?s=400&u=3833aeb5ec8671c98d415b620b5e6a65cfb0d6d2&v=4" width=64 alt="myself" />
        </div>
    }
}
