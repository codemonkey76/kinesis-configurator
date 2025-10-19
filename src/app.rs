use adw::prelude::*;
use gtk4::prelude::*;
use libadwaita as adw;
use relm4::{ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

use crate::{
    components::{
        KeyboardView,
        remap_dialog::{RemapDialog, RemapType},
    },
    models::{KeyAction, KinesisLayout},
};

#[derive(Debug)]
pub struct App {
    layouts: [KinesisLayout; 9],
    current_layout: usize,
    keyboard_view: KeyboardView,
    main_window: adw::ApplicationWindow,
}

#[derive(Debug)]
pub enum AppMsg {
    SwitchLayout(usize),
    LoadConfig,
    SaveConfig,
    DetectKeyboard,
    KeyClicked(String),
    ApplyRemap {
        source: String,
        target: Option<String>,
        remap_type: RemapType,
    },
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
            set_hexpand: true,
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
            gtk4::Frame {
            set_halign: gtk4::Align::Center,
            set_valign: gtk4::Align::Center,
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
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {
            layouts: std::array::from_fn(|_| KinesisLayout::new()),
            current_layout: 0,
            keyboard_view: KeyboardView::new(sender.input_sender().clone()),
            main_window: root.clone(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::SwitchLayout(idx) => {
                if idx < 9 {
                    self.current_layout = idx;
                    self.load_layout_into_view();
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
            AppMsg::KeyClicked(key_label) => {
                let current_mapping = self.get_current_mapping(&key_label);
                let key_label_clone = key_label.clone();
                let input = sender.input_sender().clone();

                let window = self.main_window.clone();

                relm4::spawn_local(async move {
                    let dialog = RemapDialog::new(&key_label_clone, current_mapping.as_deref());

                    if let Some(result) = dialog.run(&window).await {
                        let _ = input.send(AppMsg::ApplyRemap {
                            source: result.source_key,
                            target: result.target_key,
                            remap_type: result.remap_type,
                        });
                    }
                });
            }
            AppMsg::ApplyRemap {
                source,
                target,
                remap_type,
            } => {
                let layout = &mut self.layouts[self.current_layout];
                layout.remove_by_source(&source);

                if let Some(target_key) = &target {
                    match remap_type {
                        RemapType::Simple => {
                            if !Self::is_valid_key(target_key) {
                                let error_dialog = adw::AlertDialog::new(
                                    Some("Invalid Key"),
                                    Some(&format!(
                                        "'{}' is not a valid key name. Please enter a single key or a recognized key name like 'Enter', 'Shift', etc.",
                                        target_key
                                    )),
                                );
                                error_dialog.add_response("ok", "OK");
                                error_dialog.set_default_response(Some("ok"));

                                let window = self.main_window.clone();
                                relm4::spawn_local(async move {
                                    error_dialog.choose_future(&window).await;
                                });
                                return;
                            }
                            // Normalize and get full label for the target BEFORE mutable borrow
                            let full_target_label = self.get_full_key_label(target_key);
                            let layout = &mut self.layouts[self.current_layout];
                            layout.remove_by_source(&source);
                            layout.add_remap(source.clone(), full_target_label.clone());
                            self.keyboard_view
                                .set_remapping(&source, &full_target_label);
                            println!("Remapped {} -> {}", source, full_target_label);
                        }
                        RemapType::Macro => {
                            layout.add_macro(source.clone(), target_key.clone());
                            self.keyboard_view
                                .set_remapping(&source, &format!("Macro: {}", target_key));
                            println!("Created macro {} -> {}", source, target_key);
                        }
                    }
                } else {
                    let layout = &mut self.layouts[self.current_layout];
                    layout.remove_by_source(&source);

                    self.keyboard_view.clear_remapping(&source);
                    println!("Cleared mapping for {}", source);
                }
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

    fn load_layout_into_view(&mut self) {
        let layout = &self.layouts[self.current_layout];

        self.keyboard_view.clear_all_remappings();

        for mapping in &layout.mappings {
            if let KeyAction::SimpleRemap { source, target } = mapping {
                self.keyboard_view.set_remapping(source, target);
            }
        }
    }

    fn get_current_mapping(&self, key: &str) -> Option<String> {
        let layout = &self.layouts[self.current_layout];
        layout.find_by_source(key).first().and_then(|action| {
            if let KeyAction::SimpleRemap { target, .. } = action {
                Some(target.clone())
            } else {
                None
            }
        })
    }

    fn is_valid_key(key_str: &str) -> bool {
        let normalized = key_str.to_uppercase();

        if key_str.len() == 1 {
            return key_str.chars().next().unwrap().is_ascii_graphic();
        }

        matches!(
            normalized.as_str(),
            "TAB"
                | "ESC"
                | "ESCAPE"
                | "SPACE"
                | "SPC"
                | "ENTER"
                | "RETURN"
                | "BACKSPACE"
                | "BKSP"
                | "DELETE"
                | "DEL"
                | "SHIFT"
                | "LSHIFT"
                | "RSHIFT"
                | "CTRL"
                | "CONTROL"
                | "LCTRL"
                | "RCTRL"
                | "ALT"
                | "LALT"
                | "RALT"
                | "HOME"
                | "END"
                | "PGUP"
                | "PAGEUP"
                | "PGDN"
                | "PAGEDOWN"
                | "PGDOWN"
                | "UP"
                | "DOWN"
                | "LEFT"
                | "RIGHT"
                | "CAPS"
                | "CAPSLOCK"
                | "WIN"
                | "WINDOWS"
                | "SUPER"
                | "META"
                | "FN"
                | "LFN"
                | "RFN"
                | "F1"
                | "F2"
                | "F3"
                | "F4"
                | "F5"
                | "F6"
                | "F7"
                | "F8"
                | "F9"
                | "F10"
                | "F11"
                | "F12"
                | "SS"
                | "SMARTSET"
                | "KP"
                | "KEYPAD"
                | "HK1"
                | "HK2"
                | "HK3"
                | "HK4"
        )
    }

    fn get_full_key_label(&self, key_str: &str) -> String {
        // Normalize to uppercase first
        let normalized = key_str.to_uppercase();

        // Map common number/symbol keys to their full labels
        match normalized.as_str() {
            "1" => "!\n1".to_string(),
            "2" => "@\n2".to_string(),
            "3" => "#\n3".to_string(),
            "4" => "$\n4".to_string(),
            "5" => "%\n5".to_string(),
            "6" => "^\n6".to_string(),
            "7" => "&\n7".to_string(),
            "8" => "*\n8".to_string(),
            "9" => "(\n9".to_string(),
            "0" => ")\n0".to_string(),
            "-" => "_\n-".to_string(),
            "=" => "+\n=".to_string(),
            "[" => "{\n[".to_string(),
            "]" => "}\n]".to_string(),
            ";" => ":\n;".to_string(),
            "'" => "\"\n'".to_string(),
            "," => "<\n,".to_string(),
            "." => ">\n.".to_string(),
            "/" => "?\n/".to_string(),
            "\\" => "|\n\\".to_string(),
            "`" => "~\n`".to_string(),
            // Special keys remain as-is
            "TAB" => "Tab".to_string(),
            "ESC" | "ESCAPE" => "Esc".to_string(),
            "SPACE" | "SPC" => "Space".to_string(),
            "ENTER" | "RETURN" => "Enter".to_string(),
            "BACKSPACE" | "BKSP" => "Back\nSpace".to_string(),
            "DELETE" | "DEL" => "Delete".to_string(),
            "SHIFT" | "LSHIFT" | "RSHIFT" => "Shift".to_string(),
            "CTRL" | "CONTROL" | "LCTRL" | "RCTRL" => "Ctrl".to_string(),
            "ALT" | "LALT" | "RALT" => "Alt".to_string(),
            "HOME" => "Home".to_string(),
            "END" => "End".to_string(),
            "PGUP" | "PAGEUP" => "Pg\nUp".to_string(),
            "PGDN" | "PAGEDOWN" | "PGDOWN" => "Pg\nDown".to_string(),
            "UP" => "↑".to_string(),
            "DOWN" => "↓".to_string(),
            "LEFT" => "←".to_string(),
            "RIGHT" => "→".to_string(),
            // Single letters stay uppercase
            other => other.to_string(),
        }
    }
}
