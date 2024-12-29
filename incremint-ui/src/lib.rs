use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component(Content)]
pub fn content() -> HtmlResult {
    Ok(html! {
        <div class="container w-full h-full mx-auto px-4 text-slate-900 bg-slate-100 dark:text-slate-50 dark:bg-slate-900">
            <h1>{ "Incremint" }</h1>
            { for (0..20).map(|_| html! { <p>{ "Incremental number generator" }</p> }) }
        </div>
    })
}

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
