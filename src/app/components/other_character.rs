use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::use_timeout;
use yewdux::prelude::{use_store, use_store_value};

use crate::{
    app::{
        components::{balloon::Balloon, move_node},
        models::Character,
        states::{ChatTextHashState, ChatTextState},
    },
    my_utils::px_to_tws,
    settings,
};

#[derive(PartialEq, Properties)]
pub(crate) struct OtherCharacterProps {
    pub(crate) character: Character,
}

#[function_component]
pub(crate) fn OtherCharacter(props: &OtherCharacterProps) -> Html {
    let OtherCharacterProps { character } = props;

    let character_node = use_node_ref();
    let balloon_node_ref = use_node_ref();
    let (chat_text_hash, chat_text_hash_dispatch) = use_store::<ChatTextHashState>();
    {
        let character = character.clone();
        let character_node = character_node.clone();
        let balloon_node_ref = balloon_node_ref.clone();
        use_effect_with_deps(
            move |(character, character_node, balloon_node_ref)| {
                move_node(&character_node, &character.pos_x, &character.pos_y)
                    .expect("Failed to character_node move_node.");
                move_node(&balloon_node_ref, &character.pos_x, &character.pos_y)
                    .expect("Failed to balloon_node move_node");

                let ele = character_node.cast::<HtmlElement>().unwrap();
                log::debug!(
                    "chara-pos {} {} {} {}; pos {} {}",
                    ele.offset_top(),
                    ele.offset_left(),
                    ele.offset_width(),
                    ele.offset_height(),
                    &character.pos_x,
                    &character.pos_y
                );
            },
            (character, character_node, balloon_node_ref),
        );
    }

    let ChatTextState {
        message,
        is_display_balloon,
    } = chat_text_hash
        .get(character.user_id.as_str())
        .map(|c| c.clone())
        .unwrap_or_default();

    let balloon_timeout = {
        let character = character.clone();
        use_timeout(
            move || {
                chat_text_hash_dispatch.reduce_mut(|state| {
                    state
                        .hash
                        .insert(character.user_id.clone(), ChatTextState::default())
                })
            },
            5000,
        )
    };

    {
        let is_display_balloon = is_display_balloon.clone();
        let balloon_timeout = balloon_timeout.clone();
        use_effect_with_deps(
            move |is_display_balloon| {
                log::debug!("other_chara balloon {}", is_display_balloon);
                if *is_display_balloon {
                    balloon_timeout.reset();
                }
            },
            is_display_balloon,
        );
    }

    html! {
        <div>
            <div ref={character_node}
            class={classes!("absolute", "select-none",
                    "-top-[32px]", "-left-[32px]",
                    "w-[64px]", "h-[64px]",
                    "rounded-full",
                    "transform-gpu", "translate-x-[50vw]", "translate-y-[50vh]",
                    "z-[800]", "ease-character-move", "duration-700",
                    "overflow-hidden"
            )}>
                <img src={character.url.clone()} width=64 alt={character.user_id.clone()} />

            </div>
            <Balloon node_ref={balloon_node_ref} is_display_balloon={is_display_balloon} is_myself={false}>
            {
                message
            }
            </Balloon>
        </div>
    }
}
