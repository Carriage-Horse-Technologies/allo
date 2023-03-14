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

const PRODUCT_INFO_LIST: [ProductInfo; 7] = [
    ProductInfo {
        title: "RED",
        url: "https://games.jyogi.net/",
        topaz_url: "https://topaz.dev/projects/0bdca801952a9a59bba7",
        img_src: "https://topaz.dev/_next/image?url=https%3A%2F%2Fptera-publish.topaz.dev%2Fproject%2F01GDGDQ2DYKE527HP55Z0R008H.png&w=1920&q=75"
    },
    ProductInfo {
        title: "奇声",
        url: "https://kisei.yukinissie.com/",
        topaz_url: "https://topaz.dev/projects/4f42624a1a4028f63363",
        img_src: "https://topaz.dev/_next/image?url=https%3A%2F%2Fptera-publish.topaz.dev%2Fproject%2F01FY73Y30XEE6T5BCAC0JD5JSZ.jpeg&w=2048&q=75"
    },
    ProductInfo {
        title: "デスマTV",
        url: "https://viewer.deathmatv.online/",
        topaz_url: "https://topaz.dev/projects/50a804868af6407eb504",
        img_src: "https://topaz.dev/_next/image?url=https%3A%2F%2Fptera-publish.topaz.dev%2Fproject%2F01G5X2YKFPN5EDNTJB9VGRFT5R.png&w=3840&q=75"
    },
    ProductInfo {
        title: "まさかり",
        url: "https://masakari.yukinissie.com/",
        topaz_url: "https://masakari.yukinissie.com/",
        img_src: "https://topaz.dev/_next/image?url=https%3A%2F%2Fptera-publish.topaz.dev%2Fproject%2F01GDGDQ2DYKE527HP55Z0R008H.png&w=1920&q=75"
    },
    ProductInfo {
        title: "Bears Sandbag",
        url: "https://bears-sandbag.yukinissie.com/",
        topaz_url: "https://topaz.dev/projects/207c286d525ad968e969",
        img_src: "https://topaz.dev/_next/image?url=https%3A%2F%2Fptera-publish.topaz.dev%2Fproject%2F01FAA15EKREPQ82JT53QQ5YGEW.png&w=3840&q=75"
    },
    ProductInfo {
        title: "Meguru Cosmos",
        url: "https://lonely1.yukinissie.com/",
        topaz_url: "https://topaz.dev/projects/d41a0662268f7a1aca4a",
        img_src: "https://topaz.dev/_next/image?url=https%3A%2F%2Fptera-publish.topaz.dev%2Fproject%2F01GVESENFPRVRG50GS3WQ6ETQA.png&w=3840&q=75"
    },
    ProductInfo {
        title: "Wa:talk",
        url: "https://watalk.yukinissie.com/",
        topaz_url: "https://topaz.dev/projects/b319ba459e32910d7015",
        img_src: "https://topaz.dev/_next/image?url=https%3A%2F%2Fptera-publish.topaz.dev%2Fproject%2F01GVET9N3Q8P879HEP6V3R680R.png&w=3840&q=75"
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
        <div class={classes!("grid", "grid-cols-3", "justify-items-center", "place-content-around", "place-items-center",
                            )}>
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
