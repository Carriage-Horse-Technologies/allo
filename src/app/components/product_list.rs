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
};

const PRODUCT_INFO_LIST: [ProductInfo; 4] = [ProductInfo {
    title: "RED",
    url: "https://games.jyogi.net/",
    topaz_url: "https://topaz.dev/projects/0bdca801952a9a59bba7",
    img_src: "https://topaz.dev/_next/image?url=https%3A%2F%2Fptera-publish.topaz.dev%2Fproject%2F01GDGDQ2DYKE527HP55Z0R008H.png&w=1920&q=75"
},
ProductInfo {
    title: "奇声",
    url: "https://kisei.yukinissie.com/",
    topaz_url: "",
    img_src: "https://topaz.dev/_next/image?url=https%3A%2F%2Fptera-publish.topaz.dev%2Fproject%2F01GDGDQ2DYKE527HP55Z0R008H.png&w=1920&q=75"
},
ProductInfo {
    title: "デスマTV",
    url: "https://viewer.deathmatv.online/",
    topaz_url: "https://topaz.dev/projects/50a804868af6407eb504",
    img_src: "https://topaz.dev/_next/image?url=https%3A%2F%2Fptera-publish.topaz.dev%2Fproject%2F01G5X2YKFPN5EDNTJB9VGRFT5R.png&w=3840&q=75"
},
ProductInfo {
    title: "RED4",
    url: "https://games.jyogi.net/",
    topaz_url: "",
    img_src: "https://topaz.dev/_next/image?url=https%3A%2F%2Fptera-publish.topaz.dev%2Fproject%2F01GDGDQ2DYKE527HP55Z0R008H.png&w=1920&q=75"
}
];

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
                            on_collision_stay = CollisionState {
                                on_collision_stay: true,
                                url: PRODUCT_INFO_LIST
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
        <div class={classes!("grid", "grid-cols-4")}>
            {
                for PRODUCT_INFO_LIST.iter().map(|info| {
                    html! {
                        <Product product_info={(*info).clone()} rect_map={products_rect_map.clone()} />
                    }
                })
            }
            if modal_state.is_display {
                <Modal />
            }
        </div>
    }
}
