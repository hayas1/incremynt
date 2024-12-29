use yew::prelude::*;

#[function_component(Main)]
pub fn main() -> HtmlResult {
    Ok(html! {
        <div>
            <h1>{ "Incremint" }</h1>
            <p>{ "Incremental number generator" }</p>
        </div>
    })
}
