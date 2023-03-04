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
            enter_button::EnterButton,
            myself::Myself,
            other_character::OtherCharacter,
            product::{Product, _ProductProps::title},
        },
        models::{CharacterLocations, LocationType},
    },
    settings::CONFIG,
};

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    let HomeProps {} = props;

    let other_characters = use_list(vec![]);

    // WebSocket設定
    let ws = {
        let other_characters = other_characters.clone();
        use_websocket_with_options(
            CONFIG.location_provider_ws_url.to_string(),
            UseWebSocketOptions {
                onopen: Some(Box::new(|event| {
                    log::info!("ws connected time_stamp: {}", event.time_stamp());
                })),
                onmessage: Some(Box::new(move |message| {
                    log::debug!("[receive] {:#?}", message);
                    let mut received_chara_locations =
                        serde_json::from_str::<CharacterLocations>(&message).unwrap();
                    // debug用にランダムで移動させる
                    if cfg!(debug_assertions) {
                        let mut pos_x = vec![0, 0];
                        let mut pos_y = vec![0, 0];
                        getrandom::getrandom(&mut pos_x).unwrap();
                        getrandom::getrandom(&mut pos_y).unwrap();
                        log::debug!("rand x: {:?}; y: {:?}", pos_x, pos_y);
                        for i in 0..(pos_x.len()) {
                            received_chara_locations.characters[i].pos_x = pos_x[i] as f64 * 6.;
                            received_chara_locations.characters[i].pos_y = pos_y[i] as f64 * 3.;
                        }
                    }
                    match received_chara_locations.action {
                        LocationType::UpdateCharacterPos => {
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
            <Myself ws={ws.ws.clone()} />
            <div>
                {
                    for other_characters.current().iter().map(|chara| {
                        html! {
                            <OtherCharacter character={chara.clone()} />
                        }
                    })
                }
            </div>
            {
                for (0..4).into_iter().map(|_| {
                    html! {
                        <Product title={product_title.clone()} url={url.clone()} img_src={img_src.clone()} />
                    }
                })
            }
            <EnterButton />
        </div>
    }
}
