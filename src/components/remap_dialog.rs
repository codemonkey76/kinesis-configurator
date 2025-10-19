use adw::prelude::*;
use gtk4::prelude::*;
use libadwaita as adw;

#[derive(Debug, Clone)]
pub struct RemapDialogResult {
    pub source_key: String,
    pub target_key: Option<String>,
    pub remap_type: RemapType,
}

#[derive(Debug, Clone)]
pub struct RemapDialog {
    dialog: adw::AlertDialog,
    entry: gtk4::Entry,
    source_key: String,
    simple_radio: gtk4::CheckButton,
    macro_radio: gtk4::CheckButton,
}

#[derive(Debug, Clone)]
pub enum RemapType {
    Simple,
    Macro,
}

impl RemapDialog {
    pub fn new(source_key: &str, current_mapping: Option<&str>) -> Self {
        let dialog = adw::AlertDialog::builder()
            .heading(format!("Remap Key: {}", source_key))
            .body("Enter the target key (e.g., 'A', 'Enter', 'LShift')\nor leave empty to clear the mapping")
            .build();

        let content_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
        content_box.set_margin_top(12);
        content_box.set_margin_bottom(12);

        let type_label = gtk4::Label::new(Some("Mapping Type:"));
        type_label.set_halign(gtk4::Align::Start);
        content_box.append(&type_label);

        let radio_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);

        let simple_radio = gtk4::CheckButton::new();
        simple_radio.set_active(true);
        let simple_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        simple_box.append(&simple_radio);
        simple_box.append(&gtk4::Label::new(Some("Simple Remap")));

        let macro_radio = gtk4::CheckButton::new();
        macro_radio.set_group(Some(&simple_radio));
        let macro_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        macro_box.append(&macro_radio);
        macro_box.append(&gtk4::Label::new(Some("Macro")));

        radio_box.append(&simple_box);
        radio_box.append(&macro_box);
        content_box.append(&radio_box);

        content_box.append(&gtk4::Separator::new(gtk4::Orientation::Horizontal));

        let entry = gtk4::Entry::new();
        let entry_clone = entry.clone();
        let content_box_clone = content_box.clone();

        simple_radio.connect_toggled(move |radio| {
            if radio.is_active() {
                entry_clone.set_placeholder_text(Some("Target key (e.g., 'A', 'Enter', 'Shift')"));
            } else {
                entry_clone.set_placeholder_text(Some(
                    "Macro text or key sequence (e.g., 'hello' or '{ctrl}{c}')",
                ));
            }
            // Force the content box to repaint
            content_box_clone.queue_draw();
        });
        entry.set_placeholder_text(Some("Target key (e.g., 'A', 'Enter', 'Shift')"));

        if let Some(mapping) = current_mapping {
            entry.set_text(mapping);
        }

        content_box.append(&entry);

        if let Some(mapping) = current_mapping {
            let current_label =
                gtk4::Label::new(Some(&format!("Current: {} -> {}", source_key, mapping)));
            current_label.add_css_class("dim-label");
            content_box.append(&current_label);
        }

        dialog.set_extra_child(Some(&content_box));

        dialog.add_response("cancel", "Cancel");

        if current_mapping.is_some() {
            dialog.add_response("clear", "Clear Mapping");
            dialog.set_response_appearance("clear", adw::ResponseAppearance::Destructive);
        }

        dialog.add_response("apply", "Apply");
        dialog.set_response_appearance("apply", adw::ResponseAppearance::Suggested);
        dialog.set_default_response(Some("apply"));
        dialog.set_close_response("cancel");

        Self {
            dialog,
            entry,
            source_key: source_key.to_string(),
            simple_radio,
            macro_radio,
        }
    }

    pub async fn run(self, parent: &impl IsA<gtk4::Widget>) -> Option<RemapDialogResult> {
        let entry = self.entry.clone();
        let source_key = self.source_key.clone();
        let simple_radio = self.simple_radio.clone();

        let response = self.dialog.choose_future(parent).await;

        match response.as_str() {
            "apply" => {
                let target = entry.text().to_string();
                if target.is_empty() {
                    Some(RemapDialogResult {
                        source_key,
                        target_key: None,
                        remap_type: RemapType::Simple,
                    })
                } else {
                    let remap_type = if simple_radio.is_active() {
                        RemapType::Simple
                    } else {
                        RemapType::Macro
                    };

                    Some(RemapDialogResult {
                        source_key,
                        target_key: Some(target),
                        remap_type,
                    })
                }
            }
            "clear" => Some(RemapDialogResult {
                source_key,
                target_key: None,
                remap_type: RemapType::Simple,
            }),
            _ => None,
        }
    }
}
