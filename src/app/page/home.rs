use std::borrow::Borrow;

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, Document, DomRect, HtmlElement};
use yew::prelude::*;
use yew_hooks::{
    use_bool_toggle, use_list, use_set, use_websocket_with_options, UseWebSocketOptions,
    UseWebSocketReadyState,
};

use crate::{
    app::{
        components::{
            enter_button::EnterButton, myself::Myself, other_character::OtherCharacter,
            product::Product, product_list::ProductList,
        },
        models::{CharacterLocations, LocationType, PageOffsetDomRect},
    },
    settings::{self, CONFIG},
};

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    let HomeProps {} = props;

    let other_characters = use_list(vec![]);

    let myself_rect = use_state(|| Option::<PageOffsetDomRect>::None);
    // WebSocket設定
    let ws = {
        let other_characters = other_characters.clone();
        use_websocket_with_options(
            format!(
                "{}/{}",
                CONFIG.location_provider_ws_url.to_string(),
                *settings::USER_ID
            ),
            UseWebSocketOptions {
                onopen: Some(Box::new(|event| {
                    log::info!("ws connected time_stamp: {}", event.time_stamp());
                })),
                onmessage: Some(Box::new(move |message| {
                    log::debug!("[receive] {:#?}", message);
                    let Ok(mut received_chara_locations) =
                        serde_json::from_str::<CharacterLocations>(&message) else {
                        log::warn!("Failed to json parse.");
                        return;
                        };
                    // debug用にランダムで移動させる
                    if cfg!(debug_assertions) {
                        let mut pos_x = vec![0, 0, 0, 0, 0];
                        let mut pos_y = vec![0, 0, 0, 0, 0];
                        getrandom::getrandom(&mut pos_x).unwrap();
                        getrandom::getrandom(&mut pos_y).unwrap();
                        log::debug!("rand x: {:?}; y: {:?}", pos_x, pos_y);
                        for i in 0..(received_chara_locations.characters.len()) {
                            received_chara_locations.characters[i].pos_x = pos_x[i] as f64 * 6.;
                            received_chara_locations.characters[i].pos_y = pos_y[i] as f64 * 3.;
                        }
                    }
                    match received_chara_locations.action {
                        LocationType::UpdateCharacterPosExample => {
                            other_characters.set(received_chara_locations.characters);
                        }
                        _ => (),
                    };
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

    let product_title = "RED".to_string();
    let url = "https://games.jyogi.net/".to_string();
    let img_src = "https://topaz.dev/_next/image?url=https%3A%2F%2Fptera-publish.topaz.dev%2Fproject%2F01GDGDQ2DYKE527HP55Z0R008H.png&w=1920&q=75".to_string();

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
        </div>
    }
}
