use adw::prelude::*;
use gtk4::prelude::*;
use libadwaita as adw;
use relm4::{ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

use crate::{components::KeyboardView, models::KinesisLayout};

#[derive(Debug)]
pub struct App {
    layouts: [KinesisLayout; 9],
    current_layout: usize,
    keyboard_view: KeyboardView,
}

#[derive(Debug)]
pub enum AppMsg {
    SwitchLayout(usize),
    LoadConfig,
    SaveConfig,
    DetectKeyboard,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
            #[root]
            main_window = adw::ApplicationWindow {
                set_title: Some("Kinesis Advantage 360 Configurator"),
                set_default_width: 1200,
                set_default_height: 800,

                #[wrap(Some)]
                set_content = &gtk4::Box {
                    set_orientation: gtk4::Orientation::Vertical,

                    adw::HeaderBar {
                        pack_start = &gtk4::Box {
                            set_spacing: 6,

                            gtk4::Button {
                                set_icon_name: "document-open-symbolic",
                                set_tooltip_text: Some("Load Config from Keyboard"),
                                connect_clicked => AppMsg::LoadConfig
                            },

                            gtk4::Button {
                                set_icon_name: "document-save-symbolic",
                                set_tooltip_text: Some("Save Config to Keyboard"),
                                connect_clicked => AppMsg::SaveConfig
                            },
                        },

                    pack_end = &gtk4::Button {
                    set_tooltip_text: Some("Search for the keyboard V-Drive mount"),
                    connect_clicked => AppMsg::DetectKeyboard,

                    gtk4::Box {
                        set_spacing: 6,

                        gtk4::Image {
                            set_icon_name: Some("input-keyboard-symbolic"),
                        },

                        gtk4::Label {
                            set_label: "Detect Keyboard",
                        },
                    }
                },
        },
        gtk4::Box {
            set_orientation: gtk4::Orientation::Horizontal,
            set_spacing: 12,
            set_margin_all: 12,
            set_halign: gtk4::Align::Center,

            gtk4::Label {
                set_label: "Layout:",
                add_css_class: "title-3",
            },

            gtk4::Box {
                set_spacing: 0,
                add_css_class: "linked",

                gtk4::Button {
                    set_label: "1",
                    #[watch]
                    set_css_classes: if model.current_layout == 0 { &["suggested-action"] } else { &[] },
                    connect_clicked => AppMsg::SwitchLayout(0),
                },
                gtk4::Button {
                    set_label: "2",
                    #[watch]
                    set_css_classes: if model.current_layout == 1 { &["suggested-action"] } else { &[] },
                    connect_clicked => AppMsg::SwitchLayout(1),
                },
                gtk4::Button {
                    set_label: "3",
                    #[watch]
                    set_css_classes: if model.current_layout == 2 { &["suggested-action"] } else { &[] },
                    connect_clicked => AppMsg::SwitchLayout(2),
                },
                gtk4::Button {
                    set_label: "4",
                    #[watch]
                    set_css_classes: if model.current_layout == 3 { &["suggested-action"] } else { &[] },
                    connect_clicked => AppMsg::SwitchLayout(3),
                },
                gtk4::Button {
                    set_label: "5",
                    #[watch]
                    set_css_classes: if model.current_layout == 4 { &["suggested-action"] } else { &[] },
                    connect_clicked => AppMsg::SwitchLayout(4),
                },
                gtk4::Button {
                    set_label: "6",
                    #[watch]
                    set_css_classes: if model.current_layout == 5 { &["suggested-action"] } else { &[] },
                    connect_clicked => AppMsg::SwitchLayout(5),
                },
                gtk4::Button {
                    set_label: "7",
                    #[watch]
                    set_css_classes: if model.current_layout == 6 { &["suggested-action"] } else { &[] },
                    connect_clicked => AppMsg::SwitchLayout(6),
                },
                gtk4::Button {
                    set_label: "8",
                    #[watch]
                    set_css_classes: if model.current_layout == 7 { &["suggested-action"] } else { &[] },
                    connect_clicked => AppMsg::SwitchLayout(7),
                },
                gtk4::Button {
                    set_label: "9",
                    #[watch]
                    set_css_classes: if model.current_layout == 8 { &["suggested-action"] } else { &[] },
                    connect_clicked => AppMsg::SwitchLayout(8),
                },
            }
        },
    gtk4::Separator {
        set_orientation: gtk4::Orientation::Horizontal,
    },

    gtk4::Box {
        set_orientation: gtk4::Orientation::Vertical,
        set_spacing: 12,
        set_margin_all: 24,
    set_vexpand: true,
    set_halign: gtk4::Align::Center,
     gtk4::Label {
                        set_label: "Keyboard Layout Editor",
                        add_css_class: "title-1",
                    },

                    gtk4::Label {
                        #[watch]
                        set_label: &format!(
                            "Layout {} - {} mappings",
                            model.current_layout + 1,
                            model.layouts[model.current_layout].mappings.len()
                        ),
                        add_css_class: "dim-label",
                    },
    // Placeholder for keyboard view
                    gtk4::Frame {
                        set_vexpand: true,
                        #[wrap(Some)]
                        set_child = model.keyboard_view.widget(),
                    },

    }
    }
    },
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {
            layouts: std::array::from_fn(|_| KinesisLayout::new()),
            current_layout: 0,
            keyboard_view: KeyboardView::new(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::SwitchLayout(idx) => {
                if idx < 9 {
                    self.current_layout = idx;
                    println!("Switched to layout {}", idx + 1);
                }
            }
            AppMsg::LoadConfig => {
                println!("Load config - not implemented yet");
            }
            AppMsg::SaveConfig => {
                println!("Save config - not implemented yet");
            }
            AppMsg::DetectKeyboard => {
                println!("Detect keyboard - not implemented yet");
            }
        }
    }
}
impl App {
    pub fn run() {
        adw::init().expect("Failed to ");
        let app = RelmApp::new("com.kinesis-configurator");
        app.run::<App>(());
    }
}
