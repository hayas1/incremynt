use std::rc::Rc;

use chrono::{Datelike, Local};
use incremynt::{Digit, Slot, SlotsArea, Space, Spacer};
use yew::Reducible;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct State<T>(pub T);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Application {
    pub area: SlotsArea,
    pub spacer: Spacer,
}
impl Application {
    pub fn init() -> Self {
        Self {
            area: SlotsArea::digits2(
                Digit::digits(Local::now().year() as usize),
                Digit::digits(Local::now().year() as usize + 1000),
            ),
            spacer: Spacer::new(Space::Half, 1),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppAction {
    SlotsAction(SlotsAction),
    SpacerAction(SpacerAction),
}
impl Reducible for State<Application> {
    type Action = AppAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut app = self.0.clone();
        match action {
            AppAction::SlotsAction(action) => {
                app.area = Rc::new(State(app.area)).reduce(action).as_ref().0.clone();
            }
            AppAction::SpacerAction(action) => {
                app.spacer = Rc::new(State(app.spacer)).reduce(action).as_ref().0.clone();
            }
        }
        State(app).into()
    }
}

#[derive(Debug, Clone)]
pub enum SlotsAction {
    UpdateSlot { index: usize, new: Slot },
}
impl Reducible for State<SlotsArea> {
    type Action = SlotsAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            SlotsAction::UpdateSlot { index, new } => {
                let mut slots = self.0.slots.clone();
                slots[index] = new;
                Rc::new(Self(SlotsArea::new(slots, SlotsArea::rows_hight())))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SpacerAction {
    UpdateSpace(Space),
    UpdateScale(usize),
}
impl From<SpacerAction> for AppAction {
    fn from(action: SpacerAction) -> Self {
        AppAction::SpacerAction(action)
    }
}
impl Reducible for State<Spacer> {
    type Action = SpacerAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut spacer = self.0.clone();
        match action {
            SpacerAction::UpdateSpace(space) => {
                spacer.space = space;
            }
            SpacerAction::UpdateScale(scale) => {
                spacer.scale = scale;
            }
        };
        State(spacer).into()
    }
}
