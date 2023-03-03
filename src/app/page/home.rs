use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, Document, HtmlElement};
use yew::prelude::*;
use yew_hooks::{
    use_bool_toggle, use_list, use_websocket_with_options, UseWebSocketOptions,
    UseWebSocketReadyState,
};

use crate::{
    app::{
        components::{myself::Myself, other_character::OtherCharacter},
        models::{CharacterLocations, LocationType},
    },
    settings::CONFIG,
};

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    let HomeProps {} = props;

    let characters = use_list(vec![]);
    let ws = {
        let characters = characters.clone();
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
                            characters.set(received_chara_locations.characters);
                        }
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

    html! {
        <div>
            <Myself />
            <div>
                {
                    for characters.current().iter().map(|chara| {
                        html! {
                            <OtherCharacter character={chara.clone()} />
                        }
                    })
                }
            </div>
        </div>
    }
}
