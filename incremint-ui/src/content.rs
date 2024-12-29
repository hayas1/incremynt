use incremint::{increment::Incremint, interface::Application, space::Width};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_autoprops::autoprops;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Interface {
    prev: usize,
    next: usize,
    space: Width,
    scale: usize,
}
impl From<Interface> for Application<Incremint> {
    fn from(interface: Interface) -> Self {
        Application::<Incremint> {
            d: (interface.prev, interface.next).into(),
            space: interface.space.into(),
            scale: interface.scale,
        }
    }
}

#[autoprops]
#[function_component(Content)]
pub fn content() -> HtmlResult {
    let interface = use_state(|| Interface {
        prev: 2024,
        next: 3024,
        space: Width::Full,
        scale: 1,
    });
    Ok(html! {
        <div class="container w-full h-full mx-auto">
            <ApplicationPane value_handler={interface.clone()} />
            <ApplicationForm value_handler={interface.clone()} />
        </div>
    })
}

#[autoprops]
#[function_component(ApplicationPane)]
pub fn application_pane(value_handler: &UseStateHandle<Interface>) -> HtmlResult {
    let application: Application<_> = (*value_handler.clone()).clone().into();

    let mut buf = Vec::new();
    application
        .run(&mut buf)
        .unwrap_or_else(|e| gloo_console::error!(e.to_string()));

    Ok(html! {
        <div class="h-[50vh] flex justify-center items-center">
            <div class="text-[4vh]">
               { for String::from_utf8_lossy(&buf).split("\n").map(ToString::to_string).map(|l| html! { <p>{ l }</p> }) }
            </div>
        </div>
    })
}

#[autoprops]
#[function_component(ApplicationForm)]
pub fn application_form(value_handler: &UseStateHandle<Interface>) -> HtmlResult {
    let initial = &*value_handler.clone();
    let prev = use_state(|| initial.prev);
    let next = use_state(|| initial.next);
    value_handler.set(Interface {
        prev: *prev,
        next: *next,
        space: Width::Full,
        scale: 1,
    });
    Ok(html! {
        <div class="flex justify-center pt-4">
            <div class="flex-initial mx-2"> <UsizeForm label="prev" value_handler={prev.clone()} /> </div>
            <div class="flex-initial mx-2"> <UsizeForm label="next" value_handler={next.clone()} /> </div>
        </div>
    })
}

#[autoprops]
#[function_component(UsizeForm)]
pub fn usize_form(label: &String, value_handler: &UseStateHandle<usize>) -> HtmlResult {
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
