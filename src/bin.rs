use cuelib::cue::CueNumber;
use gtk::prelude::*;

use relm4::{
    binding::{Binding, U8Binding},
    prelude::*,
    typed_view::list::{RelmListItem, TypedListView},
    RelmObjectExt,
};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cue {
    number: CueNumber,
    uuid: Uuid,
}

impl Cue {
    fn new() -> Self {
        Self {
            number: CueNumber::new(),
            uuid: Uuid::new_v4(),
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
    counter: u8,
    list_view_wrapper: TypedListView<Cue, gtk::SingleSelection>,
}

#[derive(Debug)]
enum Msg {
    Append,
    Remove,
    OnlyShowEven(bool),
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = u8;
    type Input = Msg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Actually idiomatic list view possible?"),
            set_default_size: (300, 100),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Append 10 items",
                    connect_clicked => Msg::Append,
                },

                gtk::Button {
                    set_label: "Remove second item",
                    connect_clicked => Msg::Remove,
                },

                gtk::ToggleButton {
                    set_label: "Only show even numbers",
                    connect_clicked[sender] => move |btn| {
                        sender.input(Msg::OnlyShowEven(btn.is_active()));
                    }
                },

                gtk::ScrolledWindow {
                    set_vexpand: true,

                    #[local_ref]
                    my_view -> gtk::ListView {}
                }
            }
        }
    }

    fn init(
        counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Initialize the ListView wrapper
        let mut list_view_wrapper: TypedListView<Cue, gtk::SingleSelection> =
            TypedListView::with_sorting();

        // Add a filter and disable it

        let model = App {
            counter,
            list_view_wrapper,
        };

        let my_view = &model.list_view_wrapper.view;

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::Append => {
                // Add 10 items
                for _ in 0..10 {
                    self.counter = self.counter.wrapping_add(1);
                    self.list_view_wrapper.append(Cue::new());
                }

                // Count up the first item
                let first_item = self.list_view_wrapper.get(0).unwrap();
                // let mut guard = first_binding.guard();
                // *guard += 1;
            }
            Msg::Remove => {
                // Remove the second item
                self.list_view_wrapper.remove(1);
            }
            Msg::OnlyShowEven(show_only_even) => {
                // Disable or enable the first filter
                self.list_view_wrapper.set_filter_status(0, show_only_even);
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
