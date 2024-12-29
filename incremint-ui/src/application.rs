use incremint::{increment::Incremint, interface::Application, space::Width};
use web_sys::{HtmlInputElement, HtmlSelectElement};
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
#[function_component(ApplicationMain)]
pub fn application_main() -> HtmlResult {
    let interface = use_state(|| Interface {
        prev: 2024,
        next: 3024,
        space: Width::Half,
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
            <div class="rounded-2xl text-[4vh] text-slate-700 bg-white dark:text-slate-100 dark:bg-slate-700">
                <pre>{ String::from_utf8_lossy(&buf) }</pre>
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
    let space = use_state(|| initial.space.clone());
    let scale = use_state(|| initial.scale);
    value_handler.set(Interface {
        prev: *prev,
        next: *next,
        space: (&*space).clone(),
        scale: *scale,
    });
    Ok(html! {
        <div class="flex flex-col">
            <div class="flex justify-center pt-4">
                <div class="flex-initial mx-2"> <UsizeInput label="prev" value_handler={prev.clone()} /> </div>
                <div class="flex-initial mx-2"> <UsizeInput label="next" value_handler={next.clone()} /> </div>
            </div>
            <div class="flex justify-center pt-4">
                <div class="flex-initial mx-2"> <WidthSelect label="space" value_handler={space.clone()} /> </div>
                <div class="flex-initial mx-2"> <UsizeInput label="scale" value_handler={scale.clone()} /> </div>
            </div>
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
                return gloo_console::error!("application dom may be changed");
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
            <input type="number" id={input_id.clone()} value={initial.to_string()} min="0" onchange={onchange}
                class="border-none rounded-sm bg-transparent text-center text-slate-900 dark:text-slate-50 leading-tight
                    focus:outline-none focus:shadow-outline appearance-none"
            />
        </div>
    })
}

#[autoprops]
#[function_component(WidthSelect)]
pub fn width_select(label: &String, value_handler: &UseStateHandle<Width>) -> HtmlResult {
    let onchange = {
        let value_handler = value_handler.clone();
        Callback::from(move |e: Event| {
            let Some(input): Option<HtmlSelectElement> = e.target_dyn_into() else {
                return gloo_console::error!("application dom may be changed");
            };
            let Ok(value) = input.value().parse() else {
                return gloo_console::error!("fail to parse value");
            };
            value_handler.set(match value {
                0 => Width::Full,
                1 => Width::Half,
                _ => unreachable!(),
            });
        })
    };

    // let initial = *value_handler.clone();
    let select_id = format!("select-width-{}", label);

    Ok(html! {
        <div class="flex items-center border-b border-slate-500 px-2 ">
            <label for={select_id.clone()} class="text-sm text-right text-slate-500 dark:text-slate-50">{ label }</label>
            <select id={select_id.clone()} onchange={onchange}
                // TODO mx-12
                class="border-none rounded-sm mx-12 bg-transparent text-center text-slate-900 dark:text-slate-50 leading-tight
                    focus:outline-none focus:shadow-outline appearance-none"
            >
                // TODO selected
                <option value="0">{ "full" }</option>
                <option value="1" selected=true>{ "half" }</option>
            </select>
        </div>
    })
}
