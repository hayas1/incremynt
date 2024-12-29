use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::content::Content;

#[autoprops]
#[function_component(Document)]
pub fn document() -> HtmlResult {
    Ok(html! {
        <body class="min-h-screen min-w-screen">
            <Header />
            <Main />
            <Footer />
        </body>
    })
}

#[autoprops]
#[function_component(Header)]
pub fn header() -> HtmlResult {
    Ok(html! {
        <header class="sticky top-0">
            <div>
                <h1>{ "Incremint" }</h1>
            </div>
        </header>
    })
}

#[autoprops]
#[function_component(Main)]
pub fn main() -> HtmlResult {
    Ok(html! {
        <main>
            <Content />
        </main>
    })
}

#[autoprops]
#[function_component(Footer)]
pub fn footer() -> HtmlResult {
    Ok(html! {
        <footer class="sticky top-[100vh]">
            <p class="text-xs text-center p-4">
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
