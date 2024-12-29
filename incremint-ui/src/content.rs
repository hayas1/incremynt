use incremint::{increment::Incremint, interface::Application, space::Width};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component(Content)]
pub fn content() -> HtmlResult {
    let program = Application::<Incremint> {
        d: (0, 0).into(),
        space: Width::Full,
        scale: 1,
    };
    let prev = use_state(|| 2024);
    let next = use_state(|| 3024);
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
            <div class="flex justify-center pt-4">
                <div class="flex-initial mx-2"> <UsizeInput label="prev" value_handler={prev.clone()} /> </div>
                <div class="flex-initial mx-2"> <UsizeInput label="next" value_handler={next.clone()} /> </div>
            </div>
            <p>{ *prev }</p>
            <p>{ " → " }</p>
            <p>{ *next }</p>
        </div>
    })
}

#[autoprops]
#[function_component(UsizeInput)]
pub fn usize_input(label: &String, value_handler: &UseStateHandle<usize>) -> HtmlResult {
    let onchange = {
        let value_handler = value_handler.clone();
        Callback::from(move |e: Event| {
            let Some(input): Option<HtmlInputElement> = e.target_dyn_into() else {
                return gloo_console::error!("target is not HtmlInputElement");
            };
            let Ok(value) = input.value().parse() else {
                return gloo_console::error!("fail to parse value");
            };
            value_handler.set(value);
        })
    };

    let initial = *value_handler.clone();
    let input_id = format!("input-int-{}", label);

    Ok(html! {
        <div class="flex items-center border-b border-slate-500 px-2">
            <label for={input_id.clone()} class="text-sm text-right text-slate-500 dark:text-slate-50">{ label }</label>
            <input type="number" value={initial.to_string()} min="0" id={input_id.clone()} onchange={onchange}
                class="border-none rounded-sm bg-transparent text-center text-slate-900 dark:text-slate-50 leading-tight
                    focus:outline-none focus:shadow-outline appearance-none"
            />
        </div>
    })
}
