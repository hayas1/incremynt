use html::IntoEventCallback;
use incremynt::{Digit, Progress, Slot, SlotsArea, Space, Spacer};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::reducer::{Application, SlotsAction, SpacerAction, State};

#[autoprops]
#[function_component(ApplicationMain)]
pub fn application_main() -> HtmlResult {
    let app = use_reducer(|| State(Application::init()));
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
    let area = value_handler.0.area.clone();
    let spacer = value_handler.0.spacer.clone();
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
            <div class="md:flex justify-center pt-4">
                <div class="flex-initial px-4 w-full">
                    <AreaForm area={area.clone()}  pane_dispatcher={pane_dispatcher.clone()} />
                </div>
            </div>
            <div class="md:flex justify-center pt-4">
                <div class="flex-initial px-4 w-full">
                    <SpaceSelect label="space" spacer={spacer.clone()} pane_dispatcher={pane_dispatcher.clone()} />
                </div>
                <div class="flex-initial px-4 w-full">
                    <UsizeInput::<_, _> label="scale" value={scale.clone()} onchange={scale_onchange} />
                </div>
            </div>
        </div>
    })
}

#[autoprops]
#[function_component(AreaForm)]
pub fn slots_form(
    area: &SlotsArea,
    pane_dispatcher: &UseReducerDispatcher<State<Application>>,
) -> HtmlResult {
    Ok(html! {
        <div class="flex flex-col">
            <div class="md:flex justify-center pt-4">
                {
                    area.slots.iter().enumerate().map(|(i, s)| {
                        html! {
                            <div class="flex-initial px-4 w-full">
                                <SlotForm index={i} slot={s.clone()} pane_dispatcher={pane_dispatcher.clone()} />
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    })
}
#[autoprops]
#[function_component(SlotForm)]
pub fn slot_form(
    index: usize,
    slot: &Slot,
    pane_dispatcher: &UseReducerDispatcher<State<Application>>,
) -> HtmlResult {
    let prev_digit_onchange = {
        let pane_dispatcher = pane_dispatcher.clone();
        Callback::from(move |e: Event| {
            let Some(input): Option<HtmlSelectElement> = e.target_dyn_into() else {
                return gloo_console::error!("application dom may be changed");
            };
            let Ok(value) = input.value().parse() else {
                return gloo_console::error!("fail to parse value");
            };
            pane_dispatcher.dispatch(
                SlotsAction::UpdateSlotPrev {
                    index,
                    new: if value < 10 {
                        Digit::mod_10(value)
                    } else {
                        unreachable!()
                    },
                }
                .into(),
            );
        })
    };
    let progress_open = {
        let pane_dispatcher = pane_dispatcher.clone();
        Callback::from(move |_| {
            pane_dispatcher.dispatch(SlotsAction::AddProgress { index }.into());
        })
    };
    let progress_close = {
        let pane_dispatcher = pane_dispatcher.clone();
        Callback::from(move |_| {
            pane_dispatcher.dispatch(SlotsAction::RemoveProgress { index }.into());
        })
    };
    Ok(html! {
        <div class="flex flex-col">
            <div class="md:flex justify-center pt-4">
                <div class="flex-initial px-4 w-full">
                    if let Some(progress) = &slot.next {
                        <div class="flex justify-start px-4 w-full">
                            <a href="#" onclick={progress_close}>{"×"}</a>
                            <ProgressForm index={index} progress={progress.clone()} pane_dispatcher={pane_dispatcher.clone()} />
                        </div>
                    } else {
                        <div class="flex justify-start px-4 w-full">
                            <a href="#" onclick={progress_open}>{"+"}</a>
                        </div>
                    }
                </div>
            </div>
            <div class="md:flex justify-center pt-4">
                <div class="flex-initial px-4 w-full">
                    <DigitSelect::<_, _> label="prev" digit={slot.prev.clone()} onchange={prev_digit_onchange} />
                </div>
            </div>
        </div>
    })
}
#[autoprops]
#[function_component(ProgressForm)]
pub fn progress_form(
    index: usize,
    progress: &Progress,
    pane_dispatcher: &UseReducerDispatcher<State<Application>>,
) -> HtmlResult {
    let next_digit_onchange = {
        let pane_dispatcher = pane_dispatcher.clone();
        Callback::from(move |e: Event| {
            let Some(input): Option<HtmlSelectElement> = e.target_dyn_into() else {
                return gloo_console::error!("application dom may be changed");
            };
            let Ok(value) = input.value().parse() else {
                return gloo_console::error!("fail to parse value");
            };
            pane_dispatcher.dispatch(
                SlotsAction::UpdateSlotNextDigit {
                    index,
                    new: if value < 10 {
                        Digit::mod_10(value)
                    } else {
                        unreachable!()
                    },
                }
                .into(),
            );
        })
    };
    let progress_onchange = {
        let pane_dispatcher = pane_dispatcher.clone();
        Callback::from(move |e: Event| {
            let Some(input): Option<HtmlInputElement> = e.target_dyn_into() else {
                return gloo_console::error!("application dom may be changed");
            };
            let Ok(value) = input.value().parse() else {
                return gloo_console::error!("fail to parse value");
            };
            pane_dispatcher
                .dispatch(SlotsAction::UpdateSlotNextProgress { index, new: value }.into());
        })
    };

    Ok(html! {
        <div class="flex flex-col">
            <div class="md:flex justify-center pt-4">
                <div class="flex-initial px-4 w-full">
                    <DigitSelect::<_, _> label="next" digit={progress.next.clone()} onchange={next_digit_onchange} />
                </div>
                <div class="flex-initial px-4 w-full">
                    <UsizeInputRange::<_, _> label="progress" value={progress.progress} min=2 max=6 onchange={progress_onchange} />
                </div>
            </div>
        </div>
    })
}
#[autoprops]
#[function_component(DigitSelect)]
pub fn digit_select<I, O>(label: &String, digit: &Digit, onchange: Callback<I, O>) -> HtmlResult
where
    Callback<I, O>: IntoEventCallback<web_sys::Event>,
{
    let select_id = format!("select-width-{}", label);

    Ok(html! {
        <div class="flex items-center border-b border-slate-500">
            <label for={select_id.clone()} class="text-sm text-right text-slate-500 dark:text-slate-50">{ label }</label>
            <select id={select_id.clone()} onchange={onchange}
                class="border-none rounded-sm bg-transparent w-full text-center text-slate-900 dark:text-slate-50 leading-tight
                    focus:outline-none focus:shadow-outline appearance-none"
            >
                <option value="0" selected={digit == &Digit::Zero}>{ "0" }</option>
                <option value="1" selected={digit == &Digit::One}>{ "1" }</option>
                <option value="2" selected={digit == &Digit::Two}>{ "2" }</option>
                <option value="3" selected={digit == &Digit::Three}>{ "3" }</option>
                <option value="4" selected={digit == &Digit::Four}>{ "4" }</option>
                <option value="5" selected={digit == &Digit::Five}>{ "5" }</option>
                <option value="6" selected={digit == &Digit::Six}>{ "6" }</option>
                <option value="7" selected={digit == &Digit::Seven}>{ "7" }</option>
                <option value="8" selected={digit == &Digit::Eight}>{ "8" }</option>
                <option value="9" selected={digit == &Digit::Nine}>{ "9" }</option>
            </select>
        </div>
    })
}

#[autoprops]
#[function_component(SpaceSelect)]
pub fn space_select(
    label: &String,
    spacer: &Spacer,
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
                <option value="full" selected={spacer.space == Space::Full}>{ "full" }</option>
                <option value="half" selected={spacer.space == Space::Half}>{ "half" }</option>
            </select>
        </div>
    })
}

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
#[function_component(UsizeInputRange)]
pub fn usize_input_range<I, O>(
    label: &String,
    value: &usize,
    min: &usize,
    max: &usize,
    onchange: Callback<I, O>,
) -> HtmlResult
where
    Callback<I, O>: IntoEventCallback<web_sys::Event>,
{
    let input_id = format!("input-int-{}", label);

    Ok(html! {
        <div class="flex items-center border-b border-slate-500">
            <label for={input_id.clone()} class="text-sm text-right text-slate-500 dark:text-slate-50">{ label }</label>
            <input type="range" id={input_id.clone()} value={value.to_string()} min={min.to_string()} max={max.to_string()} onchange={onchange}
                class="border-none rounded-sm bg-transparent w-full text-center text-slate-900 dark:text-slate-50 leading-tight
                    focus:outline-none focus:shadow-outline appearance-none"
            />
        </div>
    })
}
