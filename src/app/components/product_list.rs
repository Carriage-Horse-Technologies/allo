use std::collections::HashMap;

use yew::prelude::*;
use yew_hooks::{use_map, UseMapHandle};
use yewdux::prelude::use_store;

use crate::{
    app::{
        components::{modals::Modal, product::Product},
        models::{PageOffsetDomRect, ProductInfo},
        states::{CollisionState, ModalState},
    },
    my_utils::check_collision_with_page_offset_dom_rect,
    static_data::{NEW_PRODUCT_INFO, PAST_PRODUCT_INFO_LIST},
};

#[derive(PartialEq, Properties)]
pub(crate) struct ProductListProps {
    pub(crate) myself_rect: Option<PageOffsetDomRect>,
}

#[function_component]
pub(crate) fn ProductList(props: &ProductListProps) -> Html {
    let ProductListProps { myself_rect } = props;

    let products_rect_map: UseMapHandle<String, PageOffsetDomRect> = use_map(HashMap::new());
    let (_collision_state, collision_state_dispatch) = use_store::<CollisionState>();
    let (modal_state, modal_state_dispatch) = use_store::<ModalState>();

    {
        let myself_rect = myself_rect.clone();
        let products_rect_map = products_rect_map.clone();
        let collision_state_dispatch = collision_state_dispatch;
        use_effect_with_deps(
            move |(myself_rect, products_rect_map)| {
                if let Some(myself_rect) = myself_rect {
                    let mut on_collision_stay = CollisionState::default();
                    for (title, prod_rect) in products_rect_map.current().iter() {
                        log::debug!(
                            "rect-my {} {} {} {}",
                            myself_rect.top(),
                            myself_rect.bottom(),
                            myself_rect.left(),
                            myself_rect.right()
                        );
                        log::debug!(
                            "rect-prod {} {} {} {} {}",
                            title,
                            prod_rect.top(),
                            prod_rect.bottom(),
                            prod_rect.left(),
                            prod_rect.right()
                        );
                        if check_collision_with_page_offset_dom_rect(myself_rect, prod_rect) {
                            let mut all_product_info_list = PAST_PRODUCT_INFO_LIST.to_vec();
                            all_product_info_list.push(NEW_PRODUCT_INFO);
                            on_collision_stay = CollisionState {
                                on_collision_stay: true,
                                url: all_product_info_list
                                    .iter()
                                    .find(|prod_info| prod_info.title == title)
                                    .unwrap()
                                    .url
                                    .to_string(),
                            };
                            log::debug!("hit! {}", title);
                        }
                    }
                    // 何も接触していなかったらfalse(default)でreduce．
                    collision_state_dispatch.reduce(|_| on_collision_stay.into());
                }
            },
            (myself_rect, products_rect_map),
        );
    }

    html! {
        <div>
            <Product classes={classes!("container", "mx-auto", "mt-[300px]", "mb-[100px]",
                            "bg-gradient-to-r", "from-[#6080B0]", "via-[#08DCF9]", "to-[#FF2775]", "p-2"
                            )} product_info={NEW_PRODUCT_INFO} rect_map={products_rect_map.clone()} new={true} />
            <div class={classes!("grid", "grid-cols-3", "justify-items-center", "place-content-around", "place-items-center")}>
                {
                    for PAST_PRODUCT_INFO_LIST.iter().map(|info| {
                        html! {
                            <Product product_info={(*info).clone()} rect_map={products_rect_map.clone()} />
                        }
                    })
                }
                if modal_state.is_display {
                    <Modal />
                }
            </div>
        </div>
    }
}
