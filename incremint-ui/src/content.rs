use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component(Content)]
pub fn content() -> HtmlResult {
    Ok(html! {
        <div class="container w-full h-full mx-auto">
            <h1>{ "Incremint" }</h1>
            { for (0..20).map(|_| html! { <p>{ "Incremental number generator" }</p> }) }
        </div>
    })
}
