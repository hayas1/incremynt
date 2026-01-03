use html::IntoEventCallback;
use incremynt::{Space, Spacer};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::reducer::{Application, SpacerAction, State};

#[autoprops]
#[function_component(ApplicationMain)]
pub fn application_main() -> HtmlResult {
    let app = use_reducer_eq(|| State(Application::init()));
    let dispatcher = app.dispatcher();
    Ok(html! {
        <div class="container w-full h-full mx-auto">
            <ApplicationPane value_handler={app.clone()} />
            <ApplicationForm value_handler={app.clone()} pane_dispatcher={dispatcher.clone()} />
        </div>
    })
}

#[autoprops]
#[function_component(ApplicationPane)]
pub fn application_pane(value_handler: &UseReducerHandle<State<Application>>) -> HtmlResult {
    use std::fmt::Write;
    let application = &(*value_handler.clone()).0;

    let mut buf = String::new();
    write!(
        application.spacer.clone().fmt_write(&mut buf), // TODO do not clone
        "{}",
        application.area
    )
    .unwrap_or_else(|e| gloo_console::error!(e.to_string()));

    let onclick = {
        let copy = buf.to_string();
        Callback::from(move |_| {
            let Some(window) = web_sys::window() else {
                return gloo_console::error!("cannot get window");
            };
            let promise = window.navigator().clipboard().write_text(&copy);
            wasm_bindgen_futures::spawn_local(async move {
                match wasm_bindgen_futures::JsFuture::from(promise).await {
                    Ok(_) => (),
                    Err(e) => gloo_console::error!(e),
                }
            });
        })
    };

    Ok(html! {
        <div class="flex flex-col justify-center items-center">
            <button onclick={onclick} title="copy"
                class="h-[50vh] rounded-2xl text-[4vh]
                    text-slate-700 bg-white dark:text-slate-100 dark:bg-slate-700
                    hover:bg-slate-100 hover:dark:bg-slate-800"
            >
                <pre class="text-left">{ buf }</pre>
            </button>
        </div>
    })
}

#[autoprops]
#[function_component(ApplicationForm)]
pub fn application_form(
    value_handler: &UseReducerHandle<State<Application>>,
    pane_dispatcher: &UseReducerDispatcher<State<Application>>,
) -> HtmlResult {
    let area = use_reducer_eq(|| State(value_handler.0.area.clone()));
    let space = use_reducer_eq(|| State(value_handler.0.spacer.clone()));
    let scale = value_handler.0.spacer.scale;
    let scale_onchange = {
        let pane_dispatcher = pane_dispatcher.clone();
        Callback::from(move |e: Event| {
            let Some(input): Option<HtmlInputElement> = e.target_dyn_into() else {
                return gloo_console::error!("application dom may be changed");
            };
            let Ok(value) = input.value().parse() else {
                return gloo_console::error!("fail to parse value");
            };
            pane_dispatcher.dispatch(SpacerAction::UpdateScale(value).into());
        })
    };

    Ok(html! {
        <div class="flex flex-col">
            // <div class="md:flex justify-center pt-4">
            //     <div class="flex-initial px-4 w-full"> <AreaForm value_handler={area.clone()} /> </div>
            // </div>
            <div class="md:flex justify-center pt-4">
                <div class="flex-initial px-4 w-full">
                    <SpaceSelect label="space" value_handler={space.clone()} pane_dispatcher={pane_dispatcher.clone()} />
                </div>
                <div class="flex-initial px-4 w-full">
                    <UsizeInput::<_, _> label="scale" value={scale.clone()} onchange={scale_onchange} />
                </div>
            </div>
        </div>
    })
}

// #[autoprops]
// #[function_component(AreaForm)]
// pub fn slots_form(value_handler: &UseReducerHandle<State<SlotsArea>>) -> HtmlResult {
//     Ok(html! {
//         <div class="flex flex-col">
//             <div class="md:flex justify-center pt-4">
//                 {
//                     slots.slots.into_iter().map(|s| {
//                         let slot = use_state(|| s.clone());
//                         html! {
//                             <div class="flex-initial px-4 w-full"> <SlotForm value_handler={slot.clone()} /> </div>
//                         }
//                     }).collect::<Html>()
//                 }
//             </div>
//         </div>
//     })
// }
// #[autoprops]
// #[function_component(SlotForm)]
// pub fn slot_form(value_handler: &UseReducerHandle<State<Slot>>) -> HtmlResult {
//     let initial = &*value_handler.clone();
//     let prev = use_state(|| initial.prev.clone());
//     // let next = use_state(|| initial.prev); // TODO next
//     value_handler.set(Slot {
//         prev: (*prev).clone(),
//         next: None,
//     });
//     Ok(html! {
//         <div class="flex flex-col">
//             <div class="md:flex justify-center pt-4">
//                 <div class="flex-initial px-4 w-full"> <DigitSelect label="prev" value_handler={prev.clone()} /> </div>
//             </div>
//             <div class="md:flex justify-center pt-4">
//                 <div class="flex-initial px-4 w-full"> <DigitSelect label="next" value_handler={prev.clone()} /> </div>
//             </div>
//         </div>
//     })
// }
// #[autoprops]
// #[function_component(DigitSelect)]
// pub fn digit_select(label: &String, value_handler: &UseReducerHandle<State<Digit>>) -> HtmlResult {
//     let onchange = {
//         let value_handler = value_handler.clone();
//         Callback::from(move |e: Event| {
//             let Some(input): Option<HtmlSelectElement> = e.target_dyn_into() else {
//                 return gloo_console::error!("application dom may be changed");
//             };
//             let Ok(value) = input.value().parse() else {
//                 return gloo_console::error!("fail to parse value");
//             };
//             value_handler.set(match value {
//                 0 => Digit::Zero,
//                 1 => Digit::One,
//                 2 => Digit::Two,
//                 3 => Digit::Three,
//                 4 => Digit::Four,
//                 5 => Digit::Five,
//                 6 => Digit::Six,
//                 7 => Digit::Seven,
//                 8 => Digit::Eight,
//                 9 => Digit::Nine,
//                 _ => unreachable!(),
//             });
//         })
//     };

//     let select_id = format!("select-width-{}", label);

//     Ok(html! {
//         <div class="flex items-center border-b border-slate-500">
//             <label for={select_id.clone()} class="text-sm text-right text-slate-500 dark:text-slate-50">{ label }</label>
//             <select id={select_id.clone()} onchange={onchange}
//                 class="border-none rounded-sm bg-transparent w-full text-center text-slate-900 dark:text-slate-50 leading-tight
//                     focus:outline-none focus:shadow-outline appearance-none"
//             >
//                 <option value="0" selected={*value_handler.clone() == Digit::Zero}>{ "0" }</option>
//                 <option value="1" selected={*value_handler.clone() == Digit::One}>{ "1" }</option>
//                 <option value="2" selected={*value_handler.clone() == Digit::Two}>{ "2" }</option>
//                 <option value="3" selected={*value_handler.clone() == Digit::Three}>{ "3" }</option>
//                 <option value="4" selected={*value_handler.clone() == Digit::Four}>{ "4" }</option>
//                 <option value="5" selected={*value_handler.clone() == Digit::Five}>{ "5" }</option>
//                 <option value="6" selected={*value_handler.clone() == Digit::Six}>{ "6" }</option>
//                 <option value="7" selected={*value_handler.clone() == Digit::Seven}>{ "7" }</option>
//                 <option value="8" selected={*value_handler.clone() == Digit::Eight}>{ "8" }</option>
//                 <option value="9" selected={*value_handler.clone() == Digit::Nine}>{ "9" }</option>
//             </select>
//         </div>
//     })
// }

#[autoprops]
#[function_component(UsizeInput)]
pub fn usize_input<I, O>(label: &String, value: &usize, onchange: Callback<I, O>) -> HtmlResult
where
    Callback<I, O>: IntoEventCallback<web_sys::Event>,
{
    let input_id = format!("input-int-{}", label);

    Ok(html! {
        <div class="flex items-center border-b border-slate-500">
            <label for={input_id.clone()} class="text-sm text-right text-slate-500 dark:text-slate-50">{ label }</label>
            <input type="number" id={input_id.clone()} value={value.to_string()} min="0" onchange={onchange}
                class="border-none rounded-sm bg-transparent w-full text-center text-slate-900 dark:text-slate-50 leading-tight
                    focus:outline-none focus:shadow-outline appearance-none"
            />
        </div>
    })
}

#[autoprops]
#[function_component(SpaceSelect)]
pub fn space_select(
    label: &String,
    value_handler: &UseReducerHandle<State<Spacer>>,
    pane_dispatcher: &UseReducerDispatcher<State<Application>>,
) -> HtmlResult {
    let onchange = {
        let pane_dispatcher = pane_dispatcher.clone();
        Callback::from(move |e: Event| {
            let Some(input): Option<HtmlSelectElement> = e.target_dyn_into() else {
                return gloo_console::error!("application dom may be changed");
            };
            pane_dispatcher.dispatch(
                SpacerAction::UpdateSpace(match &input.value()[..] {
                    "full" => Space::Full,
                    "half" => Space::Half,
                    _ => unreachable!(),
                })
                .into(),
            );
        })
    };

    let select_id = format!("select-width-{}", label);

    Ok(html! {
        <div class="flex items-center border-b border-slate-500">
            <label for={select_id.clone()} class="text-sm text-right text-slate-500 dark:text-slate-50">{ label }</label>
            <select id={select_id.clone()} onchange={onchange}
                class="border-none rounded-sm bg-transparent w-full text-center text-slate-900 dark:text-slate-50 leading-tight
                    focus:outline-none focus:shadow-outline appearance-none"
            >
                <option value="full" selected={(*value_handler.clone()).0.space == Space::Full}>{ "full" }</option>
                <option value="half" selected={(*value_handler.clone()).0.space == Space::Half}>{ "half" }</option>
            </select>
        </div>
    })
}
