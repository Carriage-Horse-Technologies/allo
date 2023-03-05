use web_sys::{DomRect, HtmlElement};
use yew::prelude::*;
use yew_hooks::UseMapHandle;

use crate::{app::models::ProductInfo, my_utils::check_collision_with_dom_rect};

#[derive(PartialEq, Properties)]
pub(crate) struct ProductProps {
    pub(crate) product_info: ProductInfo,
    pub(crate) rect_map: UseMapHandle<String, DomRect>,
}

#[function_component]
pub(crate) fn Product(props: &ProductProps) -> Html {
    let ProductProps {
        product_info,
        rect_map,
    } = props;

    let node = use_node_ref();
    {
        let title = product_info.title.clone();
        let node = node.clone();
        let rect_map = rect_map.clone();
        use_effect_with_deps(
            move |node| {
                let div = node.cast::<HtmlElement>().unwrap();
                let dom_rect = div.get_bounding_client_rect();

                log::debug!(
                    "DomRect top: {}; bottom: {}; left: {}; right: {}",
                    dom_rect.top(),
                    dom_rect.bottom(),
                    dom_rect.left(),
                    dom_rect.right(),
                );

                rect_map.insert(title.to_string(), dom_rect);
            },
            node,
        );
    }

    let onload = {
        let title = product_info.title.clone();
        let node = node.clone();
        let rect_map = rect_map.clone();
        Callback::from(move |_| {
            let div = node.cast::<HtmlElement>().unwrap();
            let dom_rect = div.get_bounding_client_rect();

            log::debug!(
                "DomRect top: {}; bottom: {}; left: {}; right: {}",
                dom_rect.top(),
                dom_rect.bottom(),
                dom_rect.left(),
                dom_rect.right(),
            );

            rect_map.insert(title.to_string(), dom_rect);
        })
    };

    html! {
        <div ref={node} class="w-fit h-fit">
            <a href={product_info.url.clone()} target="_blank" rel="noopener noreferrer"
                class="flex max-w-[512px]">
                <figure class="h-fit">
                    <img src={product_info.img_src.clone()} onload={onload} alt={product_info.title.clone()} width=512 />
                    <figcaption class="text-center">{product_info.title.clone()}</figcaption>
                </figure>
            </a>
        </div>
    }
}
