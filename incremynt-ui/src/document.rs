use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::application::ApplicationMain;

#[autoprops]
#[function_component(Document)]
pub fn document() -> HtmlResult {
    Ok(html! {
        <body class="min-h-screen min-w-screen text-slate-900 bg-slate-50 dark:text-slate-50 dark:bg-slate-800">
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
        <header class="sticky top-0 shadow text-slate-900 bg-slate-100 dark:text-slate-100 dark:bg-slate-900">
            <nav class="flex justify-between items-center p-2">
                <div class="flex items-center">
                    <h1>{ "incremynt" }</h1>
                </div>
            </nav>
        </header>
    })
}

#[autoprops]
#[function_component(Main)]
pub fn main() -> HtmlResult {
    Ok(html! {
        <main>
            <ApplicationMain />
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
                <a href={ env!("CARGO_PKG_REPOSITORY") }
                    target="_blank" rel="noopener noreferrer"
                    class="underline"
                >
                    { env!("CARGO_PKG_NAME") }
                </a>
            </p>
        </footer>
    })
}
