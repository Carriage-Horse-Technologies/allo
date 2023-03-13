use yew::prelude::*;
use yew_hooks::{use_list, use_websocket_with_options, UseWebSocketOptions};
use yewdux::prelude::{use_store, use_store_value};

use crate::{
    app::{
        components::{
            chat_text_field::ChatTextField, enter_button::EnterButton, modals::Modal,
            myself::Myself, other_character::OtherCharacter, product_list::ProductList,
        },
        models::{Character, CharacterLocations, ChatMessage, LocationType, PageOffsetDomRect},
        states::{ChatTextHashState, ChatTextState, Username},
    },
    settings::{self, CONFIG},
};

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    let HomeProps {} = props;

    let username = use_store_value::<Username>();
    let other_characters = use_list(vec![]);
    let (_, chat_text_hash_dispatch) = use_store::<ChatTextHashState>();

    let myself_rect = use_state(|| Option::<PageOffsetDomRect>::None);
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

    if username.0.is_empty() {
        web_sys::window()
            .unwrap()
            .location()
            .set_href("/entrance")
            .unwrap();
    }

    html! {
        <div class="pt-[100px] w-[2000px] h-[1500px] dark:bg-dark-content-background">
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
        </div>
    }
}
