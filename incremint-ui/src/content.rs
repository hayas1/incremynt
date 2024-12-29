use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component(Content)]
pub fn content() -> HtmlResult {
    Ok(html! {
        <div class="container w-full h-full mx-auto">
            <div class="h-[50vh] flex justify-center items-center">
                <div class="text-[4vh]">
                    {"┏━┛┃            "} <br />
                    {"┗━┓┃┏━━┓┏━━┓┏┓┏┓"} <br />
                    {"┏━┛┃┃┏┓┃┗━┓┃┃┃┃┃"} <br />
                    {"┗━━┛┃┃┃┃┏━┛┃┃┗┛┃"} <br />
                    {"┏━━┓┃┃┃┃┃┏━┛┗━┓┃"} <br />
                    {"┗━┓┃┃┗┛┃┃┗━┓　　┃┃"} <br />
                    {"┏━┛┃┗━━┛┗━━┛　　┗┛"} <br />
                    {"┃┏━┛            "} <br />
                </div>
            </div>
            <div class="">
                { for (0..20).map(|_| html! { <p>{ "Incremental number generator" }</p> }) }
            </div>
        </div>
    })
}
