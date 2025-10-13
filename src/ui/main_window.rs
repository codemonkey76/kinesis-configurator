use gtk::prelude::*;
use libadwaita as adw;
use relm4::gtk;
use relm4::prelude::*;

use crate::config::KeyboardConfig;
use crate::ui::keyboard_view::KeyboardView;
use crate::vdrive::VDrive;

#[derive(Debug)]
pub enum AppMsg {
    LoadConfig,
    SaveConfig,
    DetectKeyboard,
    UpdateStatus(String),
    SwitchLayout(usize),
    CopyLayout,
}

pub struct AppModel {
    config: Option<KeyboardConfig>,
    vdrive: VDrive,
    status_message: String,
    current_layout: usize,
    copy_from_layout: Option<usize>,
    keyboard_view: KeyboardView,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        #[root]
        adw::ApplicationWindow {
            set_title: Some("Kinesis 360 Configurator"),
            set_default_size: (1000, 700),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    pack_start = &gtk::Button {
                        set_label: "Load Config",
                        set_icon_name: "document-open-symbolic",
                        connect_clicked => AppMsg::LoadConfig,
                    },
                    pack_start = &gtk::Button {
                        set_label: "Save Config",
                        set_icon_name: "document-save-symbolic",
                        connect_clicked => AppMsg::SaveConfig,
                    },
                    pack_end = &gtk::Button {
                        set_tooltip_text: Some("Search for the keyboard V-Drive mount"),
                        connect_clicked => AppMsg::DetectKeyboard,

                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,

                            gtk::Image {
                                set_icon_name: Some("input-keyboard-symbolic"),
                            },

                            gtk::Label {
                                set_label: "Detect Keyboard",
                            },
                        }
                    },
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 12,
                    set_margin_all: 12,
                    set_vexpand: true,

                    // Layout selector
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 6,
                        set_halign: gtk::Align::Center,

                        gtk::Label {
                            set_label: "Layout:",
                            add_css_class: "title-4",
                        },

                        gtk::Button {
                            set_label: "1",
                            connect_clicked => AppMsg::SwitchLayout(0),
                        },
                        gtk::Button {
                            set_label: "2",
                            connect_clicked => AppMsg::SwitchLayout(1),
                        },
                        gtk::Button {
                            set_label: "3",
                            connect_clicked => AppMsg::SwitchLayout(2),
                        },
                        gtk::Button {
                            set_label: "4",
                            connect_clicked => AppMsg::SwitchLayout(3),
                        },
                        gtk::Button {
                            set_label: "5",
                            connect_clicked => AppMsg::SwitchLayout(4),
                        },
                        gtk::Button {
                            set_label: "6",
                            connect_clicked => AppMsg::SwitchLayout(5),
                        },
                        gtk::Button {
                            set_label: "7",
                            connect_clicked => AppMsg::SwitchLayout(6),
                        },
                        gtk::Button {
                            set_label: "8",
                            connect_clicked => AppMsg::SwitchLayout(7),
                        },
                        gtk::Button {
                            set_label: "9",
                            connect_clicked => AppMsg::SwitchLayout(8),
                        },

                        gtk::Separator {
                            set_orientation: gtk::Orientation::Vertical,
                        },

                        gtk::Button {
                            set_label: "Copy Layout",
                            set_icon_name: "edit-copy-symbolic",
                            connect_clicked => AppMsg::CopyLayout,
                        },
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 6,

                        gtk::Label {
                            set_label: "Keyboard Layout",
                            add_css_class: "title-2",
                            set_halign: gtk::Align::Start,
                        },

                        gtk::Frame {
                            set_vexpand: true,

                            gtk::ScrolledWindow {
                                #[wrap(Some)]
                                set_child = model.keyboard_view.widget(),
                            },
                        },
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 6,

                        gtk::Label {
                            set_label: "Configuration Details",
                            add_css_class: "title-3",
                            set_halign: gtk::Align::Start,
                        },

                        gtk::Frame {
                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 6,
                                set_margin_all: 12,

                                gtk::Label {
                                    #[watch]
                                    set_label: &model.status_message,
                                    set_wrap: true,
                                    set_halign: gtk::Align::Start,
                                },
                            },
                        },
                    },
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {
            config: None,
            vdrive: VDrive::new(),
            status_message: "Welcome! To begin:\n1. Press SmartSet + Hk3 on your keyboard to enable V-Drive mode\n2. Wait for the drive to mount\n3. Click 'Detect Keyboard' to find your keyboard".to_string(),
            current_layout: 0,
            copy_from_layout: None,
            keyboard_view: KeyboardView::new(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::LoadConfig => {
                self.status_message = "Loading configuration...".to_string();
                match self.vdrive.load_config() {
                    Ok(config) => {
                        self.config = Some(config);
                        self.status_message = "Configuration loaded successfully!".to_string();
                    }
                    Err(e) => {
                        self.status_message = format!("Error loading config: {}", e);
                    }
                }
            }
            AppMsg::SaveConfig => {
                self.status_message = "Saving configuration...".to_string();
                if let Some(ref config) = self.config {
                    match self.vdrive.save_config(config) {
                        Ok(_) => {
                            self.status_message = "Configuration saved successfully!".to_string();
                        }
                        Err(e) => {
                            self.status_message = format!("Error saving config: {}", e);
                        }
                    }
                } else {
                    self.status_message = "No configuration to save!".to_string();
                }
            }
            AppMsg::DetectKeyboard => {
                self.status_message = "Detecting keyboard V-Drive...".to_string();
                match self.vdrive.detect() {
                    Ok(path) => {
                        self.status_message = format!(
                            "✓ Keyboard found at: {}\nClick 'Load Config' to load your current configuration.",
                            path
                        );
                    }
                    Err(e) => {
                        self.status_message = format!(
                            "⚠ Keyboard not found: {}\n\nMake sure:\n• Your keyboard is connected via USB\n• V-Drive mode is enabled (press SmartSet + Hk3)\n• The drive has mounted (check your file manager)",
                            e
                        );
                    }
                }
            }
            AppMsg::UpdateStatus(msg) => {
                self.status_message = msg;
            }
            AppMsg::SwitchLayout(layout_idx) => {
                if layout_idx < 9 {
                    self.current_layout = layout_idx;
                    self.status_message = format!("Switched to Layout {}", layout_idx + 1);
                }
            }
            AppMsg::CopyLayout => {
                if self.copy_from_layout.is_none() {
                    // First click - mark source layout
                    self.copy_from_layout = Some(self.current_layout);
                    self.status_message = format!(
                        "Layout {} selected. Switch to target layout and click 'Copy Layout' again.",
                        self.current_layout + 1
                    );
                } else {
                    // Second click - perform copy
                    let from = self.copy_from_layout.unwrap();
                    let to = self.current_layout;

                    if let Some(ref mut config) = self.config {
                        match config.copy_layout(from, to) {
                            Ok(_) => {
                                self.status_message =
                                    format!("Copied Layout {} to Layout {}", from + 1, to + 1);
                            }
                            Err(e) => {
                                self.status_message = format!("Error copying layout: {}", e);
                            }
                        }
                    } else {
                        self.status_message = "No configuration loaded!".to_string();
                    }

                    self.copy_from_layout = None;
                }
            }
        }
    }
}
