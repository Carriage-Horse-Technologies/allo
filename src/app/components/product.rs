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
    html! {
        <div>
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
