use web_sys::{DomRect, HtmlElement};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub(crate) struct ProductProps {
    pub(crate) title: String,
    pub(crate) url: String,
    pub(crate) img_src: String,
}

#[function_component]
pub(crate) fn Product(props: &ProductProps) -> Html {
    let ProductProps {
        title,
        url,
        img_src,
    } = props;

    let node = use_node_ref();
    {
        let node = node.clone();
        use_effect_with_deps(
            |node| {
                let div = node.cast::<HtmlElement>().unwrap();
                let dom_rect = div.get_bounding_client_rect();
                log::debug!(
                    "DomRect top: {}; bottom: {}; left: {}; right: {}",
                    dom_rect.top(),
                    dom_rect.bottom(),
                    dom_rect.left(),
                    dom_rect.right(),
                );
            },
            node,
        );
    }

    html! {
        <div ref={node} class="w-fit">
            <a href={url.clone()} target="_blank" rel="noopener noreferrer"
                class="flex max-w-[512px]">
                <figure>
                    <img src={img_src.clone()} alt={title.clone()} width=512 />
                    <figcaption class="text-center">{title.clone()}</figcaption>
                </figure>
            </a>
        </div>
    }
}
