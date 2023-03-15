use yew::prelude::*;
use yew_hooks::{use_list, use_scroll, use_websocket_with_options, UseWebSocketOptions};
use yewdux::prelude::{use_store, use_store_value};

use crate::{
    app::{
        components::{
            bbs_modals::BBSModal, chat_text_field::ChatTextField,
            easter_egg_modals::EasterEggModal, enter_button::EnterButton,
            entrance_back_button::EntranceBackButton, first_visit_modals::FirstVisitModal,
            modals::Modal, myself::Myself, other_character::OtherCharacter,
            product_list::ProductList,
        },
        models::{Character, CharacterLocations, ChatMessage, LocationType, PageOffsetDomRect},
        states::{
            BBSModalState, ChatTextHashState, ChatTextState, EasterEggModalState, FirstVisitState,
            Username,
        },
    },
    settings::{self, CONFIG, WORLD_SIZE, WORLD_SIZE_CLASS_H, WORLD_SIZE_CLASS_W},
};

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    let HomeProps {} = props;

    let username = use_store_value::<Username>();
    let other_characters = use_list(vec![]);
    let (_, chat_text_hash_dispatch) = use_store::<ChatTextHashState>();
    let first_visit = use_store_value::<FirstVisitState>();

    let myself_rect = use_state(|| Option::<PageOffsetDomRect>::None);

    let easter_egg_modal_state = use_store_value::<EasterEggModalState>();
    let bbs_modal_state = use_store_value::<BBSModalState>();

    // WebSocket設定
    let ws = {
        let username = username.clone();
        let other_characters = other_characters.clone();
        let chat_text_hash_dispatch = chat_text_hash_dispatch;
        use_websocket_with_options(
            format!("{}/{}", CONFIG.location_provider_ws_url, username.0),
            UseWebSocketOptions {
                onopen: Some(Box::new(|event| {
                    log::info!("ws connected time_stamp: {}", event.time_stamp());
                })),
                onmessage: Some(Box::new(move |message| {
                    log::debug!("[receive] {:#?}", message);

                    // CharacterLocationsの処理
                    if let Ok(mut received_chara_locations) =
                        serde_json::from_str::<CharacterLocations>(&message)
                    {
                        match received_chara_locations.action {
                            LocationType::UpdateCharacterPos => {
                                let other_vec = received_chara_locations
                                    .characters
                                    .into_iter()
                                    .filter(|c| c.user_id != username.0)
                                    .collect::<Vec<Character>>();
                                other_characters.set(other_vec);
                            }
                            _ => (),
                        };
                    // ChatMessageの処理
                    } else if let Ok(received_chat_message) =
                        serde_json::from_str::<ChatMessage>(&message)
                    {
                        log::debug!(
                            "chat message {} by {}",
                            received_chat_message.message,
                            received_chat_message.user_id
                        );

                        chat_text_hash_dispatch.reduce_mut(|state| {
                            state.hash.insert(
                                received_chat_message.user_id,
                                ChatTextState {
                                    message: received_chat_message.message,
                                    is_display_balloon: true,
                                },
                            )
                        });
                    } else {
                        log::warn!("Failed to json parse.");
                        return;
                    }
                })),
                onerror: Some(Box::new(|event| {
                    log::error!(
                        "Failed to websocket. {}",
                        event.as_string().unwrap_or_default()
                    );
                })),
                onclose: Some(Box::new(|event| {
                    log::info!("ws closed time_stamp: {}", event.time_stamp());
                })),
                manual: Some(false),
                ..Default::default()
            },
        )
    };

    // 最初に中央付近にスクロールする
    use_effect_with_deps(
        |_| {
            let win = web_sys::window().unwrap();
            win.scroll_to_with_x_and_y(WORLD_SIZE.0 as f64 * 0.15, 100.);
        },
        (),
    );

    if username.0.is_empty() {
        web_sys::window()
            .unwrap()
            .location()
            .set_href("/entrance")
            .unwrap();
    }

    html! {
        <div class={classes!("pt-[100px]", WORLD_SIZE_CLASS_W.as_str(), WORLD_SIZE_CLASS_H.as_str(), "dark:bg-tile-bg-img", "dark:bg-repeat", "dark:bg-auto", "dark:dark:bg-dark-content-background")}>
            <Myself ws={ws.ws.clone()} myself_rect={myself_rect.clone()} />
            <div>
                {
                    for other_characters.current().iter().map(|chara| {
                        html! {
                            <OtherCharacter character={chara.clone()} />
                        }
                    })
                }
            </div>
            <ProductList myself_rect={(*myself_rect).clone()} />
            <EnterButton />
            <ChatTextField ws={ws.ws.clone()} />
            <EntranceBackButton />
            // if first_visit.0 {
            //     <FirstVisitModal />
            // }
            if easter_egg_modal_state.0 {
                <EasterEggModal />
            }
            if bbs_modal_state.0 {
                <BBSModal />
            }
        </div>
    }
}
