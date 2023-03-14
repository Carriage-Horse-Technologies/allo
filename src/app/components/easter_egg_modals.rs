use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::app::states::EasterEggModalState;

#[derive(PartialEq, Properties)]
pub struct EasterEggModalProps {}

#[function_component]
pub fn EasterEggModal(props: &EasterEggModalProps) -> Html {
    let EasterEggModalProps {} = props;

    let (easter_egg_modal_state, easter_egg_modal_state_dispatch) =
        use_store::<EasterEggModalState>();

    let onclick = {
        let easter_egg_modal_state_dispatch = easter_egg_modal_state_dispatch.clone();
        Callback::from(move |_| {
            easter_egg_modal_state_dispatch.reduce(|_| EasterEggModalState::default().into());
        })
    };

    html! {
        <div class="relative z-[960]" aria-labelledby="modal-title" role="dialog" aria-modal="true">
            <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"></div>

            <div class="fixed inset-0 z-[960] overflow-y-auto translate-y-[25vh]">
                <div class="flex items-end justify-center p-4 text-center sm:items-center sm:p-0">
                    <div class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-[560px] text-light-text">
                        <div class="bg-gray-50 px-4 py-3 sm:flex sm:flex-row justify-between items-center sm:px-6">
                            <p>{"ニッシー☆ 卒業おめでとう！"}</p>
                            <button type="button" onclick={onclick} class="flex w-full justify-center rounded-md bg-red-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-red-500 sm:ml-3 sm:w-auto">
                                <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-x" width="20" height="20" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                    <path d="M18 6l-12 12"></path>
                                    <path d="M6 6l12 12"></path>
                                </svg>
                            </button>
                        </div>
                        <div class="w-[560px] h-[315px] flex flex-row justify-center items-center">
                            <img class={classes!("object-contain", "-scale-x-100")} width="200" height="200" src="https://objectstorage.ap-tokyo-1.oraclecloud.com/n/nr7eduszgfzb/b/image-bucket/o/allo%2Fcracker.gif" alt="" />
                            <img class="rounded-full" src="https://pbs.twimg.com/profile_images/1521499615010463747/_s_S-96Q_400x400.jpg" width="156" height="156" alt="" />
                            <img class={classes!("object-contain")} width="200" height="200" src="https://objectstorage.ap-tokyo-1.oraclecloud.com/n/nr7eduszgfzb/b/image-bucket/o/allo%2Fcracker.gif" alt="" />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
