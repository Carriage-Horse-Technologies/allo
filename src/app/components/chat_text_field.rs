

use web_sys::HtmlTextAreaElement;
use yew::{prelude::*};
use yew_hooks::use_timeout;
use yewdux::prelude::{use_store};

use crate::{
    app::states::{ChatTextHashState, ChatTextState},
    settings,
};

#[derive(PartialEq, Properties)]
pub(crate) struct ChatTextFieldProps {}

#[function_component]
pub(crate) fn ChatTextField(props: &ChatTextFieldProps) -> Html {
    let ChatTextFieldProps {} = props;

    let node = use_node_ref();
    let (_, chat_text_dispatch) = use_store::<ChatTextHashState>();

    // 送信5秒後に吹き出しを非表示にするcallback
    let balloon_timeout = {
        let chat_text_dispatch = chat_text_dispatch.clone();

        use_timeout(
            move || {
                chat_text_dispatch.reduce_mut(|state| {
                    state
                        .hash
                        .insert(settings::USER_ID.to_string(), ChatTextState::default())
                })
            },
            5000,
        )
    };

    let onkeypress = {
        let node = node.clone();
        let chat_text_dispatch = chat_text_dispatch;
        let balloon_timeout = balloon_timeout;
        Callback::from(move |e: KeyboardEvent| {
            log::debug!(
                "keypress ctrl: {}; enter: {} {} {}",
                e.ctrl_key(),
                e.key_code(),
                e.code(),
                e.char_code()
            );
            if e.ctrl_key() && (e.code() == "Enter" || e.code() == "NumpadEnter") {
                let textarea = node.cast::<HtmlTextAreaElement>().unwrap();
                log::debug!("Send chat message. value: {}", textarea.value());
                chat_text_dispatch.reduce_mut(|state| {
                    state.hash.insert(
                        settings::USER_ID.to_string(),
                        ChatTextState {
                            message: textarea.value(),
                            is_display_balloon: true,
                        },
                    );
                });
                balloon_timeout.reset();
                textarea.set_value("");
            }
        })
    };

    html! {
        <textarea ref={node} onkeypress={onkeypress} name="chat" id="chat" cols="40" rows="3"
            placeholder={"Input text message...\nCtrl+Enter to send"}
            class={classes!("fixed", "rounded-2xl", "bg-dark-primary-deep",
            "bottom-[50px]", "left-[70vw]", "p-2"
            )}
        />
    }
}
