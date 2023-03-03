use web_sys::HtmlElement;
use yew::prelude::*;

use crate::{app::models::Character, my_utils::px_to_tws};

#[derive(PartialEq, Properties)]
pub(crate) struct OtherCharacterProps {
    pub(crate) character: Character,
}

#[function_component]
pub(crate) fn OtherCharacter(props: &OtherCharacterProps) -> Html {
    let OtherCharacterProps { character } = props;

    let character_node = use_node_ref();

    use_effect({
        let character = character.clone();
        let character_node = character_node.clone();
        move || {
            let style = character_node.cast::<HtmlElement>().unwrap().style();
            style
                .set_property(
                    "transform",
                    &format!("translate({}px, {}px)", character.pos_x, character.pos_y),
                )
                .unwrap();
        }
    });

    html! {
        <div ref={character_node}
        class={classes!("fixed", "select-none",
                "-top-[32px]", "-left-[32px]",
                "w-[64px]", "h-[64px]",
                "rounded-full",
                "transform-gpu", "translate-x-[50vw]", "translate-y-[50vh]",
                "z-800", "ease-character-move", "duration-700",
                "overflow-hidden"
        )}>
            <img src={character.url.clone()} width="64" alt={character.user_id.clone()} />
        </div>
    }
}
