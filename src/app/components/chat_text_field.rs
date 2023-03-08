use web_sys::HtmlTextAreaElement;
use yew::{prelude::*, virtual_dom::Key};

#[derive(PartialEq, Properties)]
pub(crate) struct ChatTextFieldProps {}

#[function_component]
pub(crate) fn ChatTextField(props: &ChatTextFieldProps) -> Html {
    let ChatTextFieldProps {} = props;

    let node = use_node_ref();

    let onkeypress = {
        let node = node.clone();
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
                textarea.set_value("");
            }
        })
    };

    html! {
        <textarea ref={node} onkeypress={onkeypress} name="chat" id="chat" cols="40" rows="3"
            placeholder={"Input text message...\nCtrl+Enter to send"}
            class={classes!("fixed", "rounded-2xl", "bg-dark-primary-deep",
            "bottom-[50px]", "left-3/4", "p-2"
            )}
        />
    }
}
