use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};

use futures::SinkExt;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{DomRect, HtmlElement, WebSocket};
use yew::prelude::*;
use yew_hooks::{use_bool_toggle, use_interval, use_timeout, use_websocket, UseWebSocketHandle};
use yewdux::prelude::{use_store, use_store_value};

use crate::{
    app::{
        components::balloon::Balloon,
        models::{Character, LocationType, MyLocation, PageOffsetDomRect},
        states::{ChatTextHashState, ChatTextState},
    },
    my_utils::px_to_tws,
    settings,
};

use super::move_node;

#[derive(PartialEq, Properties)]
pub(crate) struct MyselfProps {
    pub(crate) ws: Rc<RefCell<Option<WebSocket>>>,
    pub(crate) myself_rect: UseStateHandle<Option<PageOffsetDomRect>>,
}

#[function_component]
pub(crate) fn Myself(props: &MyselfProps) -> Html {
    let MyselfProps { ws, myself_rect } = props;

    let my_character_node_ref = use_node_ref();
    let balloon_node_ref = use_node_ref();
    let is_display_balloon = use_bool_toggle(false);
    let balloon_timeout = {
        let is_display_balloon = is_display_balloon.clone();
        use_timeout(move || is_display_balloon.set(false), 5000)
    };
    let is_active = use_bool_toggle(false);
    let chat_text_hash = use_store_value::<ChatTextHashState>();

    {
        let my_character_node_ref = my_character_node_ref.clone();
        let balloon_node_ref = balloon_node_ref.clone();
        let is_active = is_active.clone();
        let myself_rect = myself_rect.clone();
        let ws = ws.clone();

        use_effect_with_deps(
            move |(my_character_node_ref, is_active)| {
                let document = web_sys::window().unwrap().document().unwrap();

                // マウス移動時
                let mousemove_listener = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new({
                    let my_character_node_ref = my_character_node_ref.clone();
                    let balloon_node_ref = balloon_node_ref.clone();
                    let myself_rect = myself_rect.clone();
                    let is_active = is_active.clone();
                    move |e| {
                        if *is_active {
                            log::debug!("move! {},{}", e.page_x(), e.page_y());

                            // myself Nodeの移動
                            move_node(&my_character_node_ref, e.page_x(), e.page_y())
                                .expect("Failed to my_character_node_ref move_node");
                            // 吹き出しNodeの移動
                            move_node(&balloon_node_ref, e.page_x(), e.page_y())
                                .expect("Failed to balloon_node_ref move_node");

                            let win = web_sys::window().unwrap();
                            log::debug!(
                                "win-page {} {}",
                                win.page_x_offset().unwrap(),
                                win.page_y_offset().unwrap()
                            );
                            // 自キャラの短形取得
                            let element = my_character_node_ref.cast::<HtmlElement>().unwrap();
                            let rect = element.get_bounding_client_rect();
                            log::debug!(
                                "myself-rect top:{} bottom{} left{} right{} x{} y{}",
                                rect.top(),
                                rect.bottom(),
                                rect.left(),
                                rect.right(),
                                rect.x(),
                                rect.y()
                            );
                            let page_offset_dom_rect =
                                PageOffsetDomRect::from_dom_rect_and_page_offset(
                                    rect,
                                    (
                                        win.page_x_offset().unwrap_or_default(),
                                        win.page_y_offset().unwrap_or_default(),
                                    ),
                                );
                            log::debug!("page_offset_dom_rect {:#?}", page_offset_dom_rect);
                            myself_rect.set(Some(page_offset_dom_rect));
                        }
                    }
                }));

                // マウスボタンが離された時
                let mouseup_listener = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new({
                    let is_active = is_active.clone();
                    let myself_rect = myself_rect.clone();
                    let ws = ws.clone();
                    move |e| {
                        log::debug!("on disactive");
                        is_active.set(false);
                        if let Some(myself_rect) = (*myself_rect).clone() {
                            let my_pos = MyLocation {
                                action: LocationType::UpdateCharacterPos,
                                user_id: settings::USER_ID.to_string(),
                                pos_x: myself_rect.left(),
                                pos_y: myself_rect.top(),
                            };
                            // let s = (*ws).send(&serde_json::to_string(&my_pos).unwrap());
                            if let Err(send_result) = (*ws)
                                .borrow()
                                .clone()
                                .unwrap()
                                .send_with_str(&serde_json::to_string(&my_pos).unwrap())
                            {
                                log::error!(
                                    "Failed to Websocket send error. {}",
                                    send_result.as_string().unwrap_or_default()
                                );
                            } else {
                                log::debug!("Success websocket send");
                            }
                        }
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
            (my_character_node_ref, is_active),
        );
    }

    {}

    // Iconが押された時
    let onmousedown = {
        let is_active = is_active.clone();
        let ws = ws.clone();
        Callback::from(move |event: MouseEvent| {
            log::debug!("on active");
            is_active.set(true);
        })
    };

    let ChatTextState {
        message,
        is_display_balloon,
    } = chat_text_hash
        .get(settings::USER_ID.as_str())
        .map(|c| c.clone())
        .unwrap_or_default();

    html! {
        <div>
            <div ref={my_character_node_ref} onmousedown={onmousedown}
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
            <Balloon node_ref={balloon_node_ref} is_display_balloon={is_display_balloon}>
            {
                message
            }
            </Balloon>
        </div>
    }
}
