use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component(Main)]
pub fn main() -> HtmlResult {
    Ok(html! {
        <Screen>
            <div class="container w-full h-full mx-auto px-4 text-slate-900 bg-slate-100 dark:text-slate-50 dark:bg-slate-900">
                <h1>{ "Incremint" }</h1>
                <p>{ "Incremental number generator" }</p>
            </div>
        </Screen>
    })
}

#[autoprops]
#[function_component(Screen)]
pub fn screen(children: &Children) -> HtmlResult {
    Ok(html! {
        <div class="flex flex-col min-h-screen">
            <div class="flex-1">
                { children.clone() }
            </div>
            <Footer />
        </div>
    })
}

#[autoprops]
#[function_component(Footer)]
pub fn footer() -> HtmlResult {
    Ok(html! {
        <footer class="text-xs text-center p-4">
            <p>
                { "Powered by " }
                <a href="https://github.com/hayas1/"
                    target="_blank" rel="noopener noreferrer"
                >
                    { env!("CARGO_PKG_NAME") }
                </a>
            </p>
        </footer>
    })
}
