use wasm_bindgen::JsValue;
use web_sys::HtmlElement;
use yew::NodeRef;

pub(crate) mod balloon;
pub(crate) mod chat_text_field;
pub(crate) mod enter_button;
pub(crate) mod header;
pub(crate) mod myself;
pub(crate) mod other_character;
pub(crate) mod product;
pub(crate) mod product_list;

fn move_node(node: &NodeRef, x: i32, y: i32) -> Result<(), JsValue> {
    let element = node.cast::<HtmlElement>().unwrap();
    let style = element.style();
    style.set_property("transform", &format!("translate({}px, {}px)", x, y))
}
