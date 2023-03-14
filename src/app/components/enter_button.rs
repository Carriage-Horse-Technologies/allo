use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlButtonElement;
use yew::prelude::*;

use yewdux::prelude::{use_store, use_store_value};

use crate::app::states::{ChatTextFieldState, CollisionState, EasterEggModalState};

#[derive(PartialEq, Properties)]
pub(crate) struct EnterButtonProps {
    pub(crate) href: Option<String>,
    pub(crate) disabled: Option<bool>,
}

#[function_component]
pub(crate) fn EnterButton(props: &EnterButtonProps) -> Html {
    let EnterButtonProps { href, disabled } = props;

    let collision_state = use_store_value::<CollisionState>();
    let button_node = use_node_ref();
    let chat_text_field_onfocus = use_store_value::<ChatTextFieldState>();
    let disable_click_counter = use_state(|| 0);
    let (easter_egg_modal_state, easter_egg_modal_state_dispatch) =
        use_store::<EasterEggModalState>();

    let onclick = {
        let href = href.clone().unwrap_or_default();
        let disabled = (*disabled).unwrap_or(true);
        let collision_state = collision_state.clone();
        let disable_click_counter = disable_click_counter.clone();
        let easter_egg_modal_state_dispatch = easter_egg_modal_state_dispatch.clone();
        Callback::from(move |_| {
            log::debug!("click enter");
            // タグを内包してしまっていることが原因でdisabledでも発火してしまうのでこっちで抑制
            if !disabled {
                let win = web_sys::window().unwrap();
                win.open_with_url(&href).unwrap();
            } else {
                // disable_click_counter.set(*disable_click_counter + 1);
            }
            if collision_state.on_collision_stay {
                let win = web_sys::window().unwrap();
                win.open_with_url(&collision_state.url).unwrap();
                disable_click_counter.set(0);
            } else {
                disable_click_counter.set(*disable_click_counter + 1);
            }
        })
    };

    {
        let disable_click_counter = disable_click_counter.clone();
        let easter_egg_modal_state_dispatch = easter_egg_modal_state_dispatch.clone();
        use_effect_with_deps(
            move |disable_click_counter| {
                if **disable_click_counter == 10 {
                    log::debug!("Easter Egg");
                    easter_egg_modal_state_dispatch.reduce(|_| EasterEggModalState(true).into());
                }
            },
            disable_click_counter,
        )
    }

    {
        // Enterキーでonclick発火
        let button_node = button_node.clone();
        let chat_text_field_onfocus = chat_text_field_onfocus.clone();
        use_effect_with_deps(
            move |(button_node, chat_text_field_onfocus)| {
                let button_node = button_node.clone();
                let document = web_sys::window().unwrap().document().unwrap();
                let chat_text_field_onfocus = chat_text_field_onfocus.clone();

                let keydown_listener = Closure::<dyn Fn(KeyboardEvent)>::wrap(Box::new({
                    move |e| {
                        if !chat_text_field_onfocus.onfocus
                            && (e.code() == "Enter" || e.code() == "NumpadEnter")
                        {
                            let button_element = button_node
                                .cast::<HtmlButtonElement>()
                                .expect("Failed to cast HtmlButtonElement");
                            let event = Event::new("click").expect("Failed to new Event");
                            button_element.dispatch_event(&event).unwrap();
                        }
                    }
                }));

                let register_listener = move || {
                    document
                        .add_event_listener_with_callback(
                            "keydown",
                            keydown_listener.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                };

                register_listener();

                register_listener
            },
            (button_node, chat_text_field_onfocus),
        );
    }

    html! {
        <button ref={button_node} type="button" onclick={onclick}
            disabled={!collision_state.on_collision_stay}
            class="fixed flex flex-row items-center z-[901]
            w-[256px] h-[64px] bottom-[50px] left-1/2 -translate-x-1/2
            rounded-full
            bg-enter-button text-5xl text-gray-700
            hover:bg-enter-button-deep
            disabled:opacity-30 disabled:bg-gray-600 disabled:text-gray-300
            disabled:w-[200px] disabled:h-[48px] disabled:text-4xl">
            <p class="mx-auto">{"Enter!"}</p>
        </button>
    }
}
