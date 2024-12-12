use cuelib::cue::CueNumber;
use gtk::prelude::*;

use relm4::{
    binding::{Binding, U8Binding},
    factory::{DynamicIndex, FactoryComponent, FactorySender, FactoryVecDeque},
    prelude::*,
    typed_view::list::{RelmListItem, TypedListView},
    RelmObjectExt,
};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cue {
    pub number: CueNumber,
    pub uuid: Uuid,
    pub label: String,
}

impl Cue {
    fn new() -> Self {
        Self {
            number: CueNumber::new(),
            uuid: Uuid::new_v4(),
            label: "test".to_string(),
        }
    }
}

#[derive(Debug)]
enum CueMsg {
    Increment,
    Decrement,
}

#[derive(Debug)]
enum CueOutput {
    SendFront(DynamicIndex),
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
}

#[relm4::factory]
impl FactoryComponent for Cue {
    type Init = Uuid;
    type Input = CueMsg;
    type Output = CueOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 10,


            #[name(number)]
            gtk::Label {
                #[watch]
                set_label: &self.number.to_string(),
                set_width_chars: 4,
            },

            #[name(label)]
            gtk::Label {
                #[watch]
                set_label: &self.label,
                set_width_chars: 16,
            },


            #[name(add_button)]
            gtk::Button {
                set_label: "+",
                connect_clicked => CueMsg::Increment,
            },

            #[name(remove_button)]
            gtk::Button {
                set_label: "-",
                connect_clicked => CueMsg::Decrement,
            },

            #[name(move_up_button)]
            gtk::Button {
                set_label: "Up",
                connect_clicked[sender, index] => move |_| {
                    sender.output(CueOutput::MoveUp(index.clone())).unwrap();
                }
            },

            #[name(move_down_button)]
            gtk::Button {
                set_label: "Down",
                connect_clicked[sender, index] => move |_| {
                    sender.output(CueOutput::MoveDown(index.clone())).unwrap();
                }
            },

            #[name(to_front_button)]
            gtk::Button {
                set_label: "To Start",
                connect_clicked[sender, index] => move |_| {
                    sender.output(CueOutput::SendFront(index.clone())).unwrap();
                }
            }

        }
    }
    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            number: CueNumber::new(),
            uuid: value,
            label: format!("cue {}", value),
        }
    }
    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            CueMsg::Increment => match self.number.0 {
                None => self.number.0 = Some(1),
                Some(n) => self.number.0 = Some(n + 1),
            },
            CueMsg::Decrement => match self.number.0 {
                None => {}
                Some(n) if n > 1 => self.number.0 = Some(n - 1),
                Some(n) if n <= 1 => self.number.0 = None,
                Some(_) => {}
            },
        }
    }
}

struct Widgets {
    label: gtk::Label,
    label2: gtk::Entry,
    button: gtk::CheckButton,
}

impl Drop for Widgets {
    fn drop(&mut self) {}
}

impl RelmListItem for Cue {
    type Root = gtk::Box;
    type Widgets = Widgets;

    fn setup(_item: &gtk::ListItem) -> (gtk::Box, Widgets) {
        relm4::view! {
            my_box = gtk::Box {
                #[name = "label"]
                gtk::Label,

                #[name = "label2"]
                gtk::Entry,

                #[name = "button"]
                gtk::CheckButton,
            }
        }

        let widgets = Widgets {
            label,
            label2,
            button,
        };

        (my_box, widgets)
    }

    fn bind(&mut self, widgets: &mut Self::Widgets, _root: &mut Self::Root) {
        let Widgets {
            label,
            label2,
            button,
        } = widgets;
    }
}

struct App {
    created_widgets: u8,
    counters: FactoryVecDeque<Cue>,
}

#[derive(Debug)]
enum AppMsg {
    AddCounter,
    RemoveCounter,
    SendFront(DynamicIndex),
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = u8;
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Factory example"),
            set_default_size: (300, 100),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Add counter",
                    connect_clicked => AppMsg::AddCounter,
                },

                gtk::Button {
                    set_label: "Remove counter",
                    connect_clicked => AppMsg::RemoveCounter,
                },

                #[local_ref]
                counter_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                }
            }
        }
    }

    // Initialize the UI.
    fn init(
        counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let counters = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |output| match output {
                CueOutput::SendFront(index) => AppMsg::SendFront(index),
                CueOutput::MoveUp(index) => AppMsg::MoveUp(index),
                CueOutput::MoveDown(index) => AppMsg::MoveDown(index),
            });

        let model = App {
            created_widgets: counter,
            counters,
        };

        let counter_box = model.counters.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::AddCounter => {
                self.counters.guard().push_back(Uuid::new_v4());
                self.created_widgets = self.created_widgets.wrapping_add(1);
            }
            AppMsg::RemoveCounter => {
                self.counters.guard().pop_back();
            }
            AppMsg::SendFront(index) => {
                self.counters.guard().move_front(index.current_index());
            }
            AppMsg::MoveDown(index) => {
                let index = index.current_index();
                let new_index = index + 1;
                // Already at the end?
                if new_index < self.counters.len() {
                    self.counters.guard().move_to(index, new_index);
                }
            }
            AppMsg::MoveUp(index) => {
                let index = index.current_index();
                // Already at the start?
                if index != 0 {
                    self.counters.guard().move_to(index, index - 1);
                }
            }
        }
    }
}

fn main() {
    println!("Test");
    let app = RelmApp::new("test");
    println!("Test2");
    app.run::<App>(0);
}
