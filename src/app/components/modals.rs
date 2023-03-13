use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::app::states::ModalState;

#[derive(PartialEq, Properties)]
pub struct ModalProps {}

#[function_component]
pub fn Modal(props: &ModalProps) -> Html {
    let ModalProps {} = props;

    let (modal_state, modal_state_dispatch) = use_store::<ModalState>();

    let onclick = {
        let modal_state_dispatch = modal_state_dispatch.clone();
        Callback::from(move |_: MouseEvent| {
            log::debug!("close modal");
            modal_state_dispatch.reduce(|_| ModalState::default().into())
        })
    };

    html! {
        <div class="relative z-[960]" aria-labelledby="modal-title" role="dialog" aria-modal="true">
            <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"></div>

            <div class="fixed inset-0 z-[960] overflow-y-auto">
                <div class="flex items-end justify-center p-4 text-center sm:items-center sm:p-0">
                    <div class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-[90vw]">
                        <div class="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
                            <button onclick={onclick} type="button" class="inline-flex w-full justify-center rounded-md bg-red-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-red-500 sm:ml-3 sm:w-auto">
                                <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-x" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                    <path d="M18 6l-12 12"></path>
                                    <path d="M6 6l12 12"></path>
                                </svg>
                            </button>
                        </div>
                        <iframe class="w-[90vw] h-[85vh]" src={modal_state.product_info.topaz_url} frameborder="2"></iframe>
                    </div>
                </div>
            </div>
        </div>
    }
}
