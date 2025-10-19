use crate::app::AppMsg;
use crate::constants;
use gtk4::DrawingArea;
use gtk4::gio;
use gtk4::glib;
use gtk4::prelude::*;
use relm4::Sender;
use rsvg;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Key {
    pub label: String,
    pub remapped_label: Option<String>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub section: KeySection,
    pub svg_data: Option<&'static str>,
}

#[derive(Debug)]
pub struct KeyboardView {
    drawing_area: DrawingArea,
    keys: Rc<RefCell<Vec<Key>>>,
    remappings: Rc<RefCell<HashMap<String, String>>>,
    hovered_key: Rc<RefCell<Option<String>>>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeySection {
    LeftHand,
    RightHand,
    LeftThumb,
    RightThumb,
}
impl KeyboardView {
    pub fn new(sender: Sender<AppMsg>) -> Self {
        let drawing_area = DrawingArea::new();
        drawing_area.set_size_request(1200, 500);
        drawing_area.set_hexpand(true);
        drawing_area.set_vexpand(true);

        let keys = Rc::new(RefCell::new(Vec::new()));
        let remappings = Rc::new(RefCell::new(HashMap::new()));
        let hovered_key = Rc::new(RefCell::new(None));

        let gesture = gtk4::GestureClick::new();
        let keys_for_click = keys.clone();
        let sender_for_click = sender.clone();

        gesture.connect_pressed(move |gesture, _n, x, y| {
            if let Some(widget) = gesture.widget() {
                let width = widget.width();
                let height = widget.height();

                if let Some(key) = Self::find_key_at_position(&keys_for_click, x, y, width, height)
                {
                    println!("Clicked key: {}", key.label);
                    let _ = sender_for_click.send(AppMsg::KeyClicked(key.label.clone()));
                }
            }
        });
        drawing_area.add_controller(gesture);

        // Motion controller for hover
        let motion = gtk4::EventControllerMotion::new();
        let keys_for_motion = keys.clone();
        let hovered_key_for_motion = hovered_key.clone();
        let drawing_area_for_motion = drawing_area.clone();

        motion.connect_motion(move |_, x, y| {
            let width = drawing_area_for_motion.width();
            let height = drawing_area_for_motion.height();

            let new_hovered = Self::find_key_at_position(&keys_for_motion, x, y, width, height)
                .map(|k| k.label.clone());

            let mut hovered = hovered_key_for_motion.borrow_mut();
            if *hovered != new_hovered {
                *hovered = new_hovered;
                drawing_area_for_motion.queue_draw();
            }
        });

        let hovered_key_for_leave = hovered_key.clone();
        let drawing_area_for_leave = drawing_area.clone();
        motion.connect_leave(move |_| {
            let mut hovered = hovered_key_for_leave.borrow_mut();
            if hovered.is_some() {
                *hovered = None;
                drawing_area_for_leave.queue_draw();
            }
        });

        drawing_area.add_controller(motion);

        let mut view = Self {
            drawing_area,
            keys: keys.clone(),
            remappings: remappings.clone(),
            hovered_key: hovered_key.clone(),
        };

        view.initialize_keys();

        let keys_for_draw = view.keys.clone();
        let hovered_key_for_draw = view.hovered_key.clone();
        view.drawing_area
            .set_draw_func(move |_, cr, width, height| {
                let keys = keys_for_draw.borrow();
                let hovered = hovered_key_for_draw.borrow();
                Self::draw(cr, width, height, &keys, hovered.as_deref());
            });
        view
    }

    pub fn clear_all_remappings(&mut self) {
        self.remappings.borrow_mut().clear();

        let mut keys = self.keys.borrow_mut();
        for key in keys.iter_mut() {
            key.remapped_label = None;
        }

        self.drawing_area.queue_draw();
    }

    fn get_section_transform(section: KeySection) -> (f64, f64, f64) {
        let left_hand_offset_x = -constants::SPLIT_GAP - constants::HAND_WIDTH;
        let right_hand_offset_x = constants::SPLIT_GAP;

        match section {
            KeySection::LeftHand => (left_hand_offset_x, constants::HAND_VERTICAL_OFFSET, 0.0),
            KeySection::RightHand => (right_hand_offset_x, constants::HAND_VERTICAL_OFFSET, 0.0),
            KeySection::LeftThumb => (
                left_hand_offset_x + constants::LEFT_THUMB_HORIZONTAL_OFFSET,
                20.0,
                constants::THUMB_ROTATION_DEGREES.to_radians(),
            ),
            KeySection::RightThumb => (
                right_hand_offset_x + constants::RIGHT_THUMB_HORIZONTAL_OFFSET,
                45.0,
                -constants::THUMB_ROTATION_DEGREES.to_radians(),
            ),
        }
    }

    fn find_key_at_position(
        keys: &Rc<RefCell<Vec<Key>>>,
        click_x: f64,
        click_y: f64,
        width: i32,
        height: i32,
    ) -> Option<Key> {
        let keys = keys.borrow();

        // Reverse the transformations from draw()
        let keyboard_width = constants::KEYBOARD_WIDTH;
        let keyboard_height = constants::KEYBOARD_HEIGHT;

        let scale_x = (width as f64 * 0.95) / keyboard_width;
        let scale_y = (height as f64 * 0.95) / keyboard_height;
        let scale = scale_x.min(scale_y);

        let center_x = width as f64 / 2.0;
        let center_y = height as f64 / 2.0;

        // Transform click coordinates to keyboard space
        let x = (click_x - center_x) / scale;
        let y = (click_y - center_y) / scale;

        // Check each section with proper transformations
        for key in keys.iter() {
            let (offset_x, offset_y, rotation) = Self::get_section_transform(key.section);

            // Transform point to section's coordinate system
            let rel_x = x - offset_x;
            let rel_y = y - offset_y;

            // Apply inverse rotation if needed
            let (local_x, local_y) = if rotation != 0.0 {
                let cos = rotation.cos();
                let sin = rotation.sin();
                (rel_x * cos + rel_y * sin, -rel_x * sin + rel_y * cos)
            } else {
                (rel_x, rel_y)
            };

            // Check if point is inside key
            if local_x >= key.x
                && local_x <= key.x + key.width
                && local_y >= key.y
                && local_y <= key.y + key.height
            {
                return Some(key.clone());
            }
        }

        None
    }

    pub fn set_remapping(&mut self, original: &str, remapped: &str) {
        self.remappings
            .borrow_mut()
            .insert(original.to_string(), remapped.to_string());

        // Update the keys
        let mut keys = self.keys.borrow_mut();
        for key in keys.iter_mut() {
            if key.label == original {
                key.remapped_label = Some(remapped.to_string());
            }
        }

        self.drawing_area.queue_draw();
    }

    pub fn clear_remapping(&mut self, original: &str) {
        self.remappings.borrow_mut().remove(original);

        let mut keys = self.keys.borrow_mut();
        for key in keys.iter_mut() {
            if key.label == original {
                key.remapped_label = None;
            }
        }

        self.drawing_area.queue_draw();
    }

    fn initialize_keys(&mut self) {
        let mut keys = self.keys.borrow_mut();

        let mut add_key = |label: &str,
                           col: f64,
                           row: f64,
                           width_mult: f64,
                           height_mult: f64,
                           section: KeySection,
                           svg_data: Option<&'static str>| {
            keys.push(Key {
                label: label.to_string(),
                remapped_label: None,
                x: col * (constants::KEY_SIZE + constants::KEY_GAP),
                y: row * (constants::KEY_SIZE + constants::KEY_GAP),
                width: constants::KEY_SIZE * width_mult,
                height: constants::KEY_SIZE * height_mult,
                section,
                svg_data,
            });
        };

        // Left hand - Row 0
        add_key("+\n=", 0.0, 0.2, 1.5, 1.0, KeySection::LeftHand, None); // Wide + offset down
        add_key("!\n1", 1.5, 0.2, 1.0, 1.0, KeySection::LeftHand, None); // Offset down
        add_key("@\n2", 2.5, 0.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("#\n3", 3.5, 0.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("$\n4", 4.5, 0.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("%\n5", 5.5, 0.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key(
            "Kp",
            6.5,
            0.0,
            1.0,
            1.0,
            KeySection::LeftHand,
            Some(constants::KP_ICON),
        );

        // Left hand - Row 1
        add_key("Tab", 0.0, 1.2, 1.5, 1.0, KeySection::LeftHand, None);
        add_key("Q", 1.5, 1.2, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("W", 2.5, 1.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("E", 3.5, 1.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("R", 4.5, 1.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("T", 5.5, 1.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key(
            "Hk1",
            6.5,
            1.0,
            1.0,
            1.0,
            KeySection::LeftHand,
            Some(constants::HK1_ICON),
        );

        // Left hand - Row 2
        add_key("Esc", 0.0, 2.2, 1.5, 1.0, KeySection::LeftHand, None);
        add_key("A", 1.5, 2.2, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("S", 2.5, 2.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("D", 3.5, 2.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("F", 4.5, 2.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("G", 5.5, 2.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key(
            "Hk2",
            6.5,
            2.0,
            1.0,
            1.0,
            KeySection::LeftHand,
            Some(constants::HK2_ICON),
        );

        // Left hand - Row 3
        add_key("LShift", 0.0, 3.2, 1.5, 1.0, KeySection::LeftHand, None);
        add_key("Z", 1.5, 3.2, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("X", 2.5, 3.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("C", 3.5, 3.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("V", 4.5, 3.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("B", 5.5, 3.0, 1.0, 1.0, KeySection::LeftHand, None);

        // Left hand - Row 4 (bottom, KeySection::LeftHand)
        add_key(
            "LFn",
            0.0,
            4.2,
            1.5,
            1.0,
            KeySection::LeftHand,
            Some(constants::FN_ICON),
        );
        add_key("~\n`", 1.5, 4.2, 1.0, 1.0, KeySection::LeftHand, None);
        add_key("Caps", 2.5, 4.0, 1.0, 1.0, KeySection::LeftHand, None);
        add_key(
            "←",
            3.5,
            4.0,
            1.0,
            1.0,
            KeySection::LeftHand,
            Some(constants::LEFT_ICON),
        );
        add_key(
            "→",
            4.5,
            4.0,
            1.0,
            1.0,
            KeySection::LeftHand,
            Some(constants::RIGHT_ICON),
        );

        // Right hand - Row 0
        add_key(
            "Ss",
            0.0,
            0.0,
            1.0,
            1.0,
            KeySection::RightHand,
            Some(constants::SMARTSET_ICON),
        );
        add_key("^\n6", 1.0, 0.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key("&\n7", 2.0, 0.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key("*\n8", 3.0, 0.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key("(\n9", 4.0, 0.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key(")\n0", 5.0, 0.2, 1.0, 1.0, KeySection::RightHand, None); // Offset down
        add_key("_\n-", 6.0, 0.2, 1.5, 1.0, KeySection::RightHand, None); // Wide + offset down

        // Right hand - Row 1
        add_key(
            "Hk3",
            0.0,
            1.0,
            1.0,
            1.0,
            KeySection::RightHand,
            Some(constants::HK3_ICON),
        );
        add_key("Y", 1.0, 1.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key("U", 2.0, 1.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key("I", 3.0, 1.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key("O", 4.0, 1.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key("P", 5.0, 1.2, 1.0, 1.0, KeySection::RightHand, None);
        add_key("|\n\\", 6.0, 1.2, 1.5, 1.0, KeySection::RightHand, None);

        // Right hand - Row 2
        add_key(
            "Hk4",
            0.0,
            2.0,
            1.0,
            1.0,
            KeySection::RightHand,
            Some(constants::HK4_ICON),
        );
        add_key("H", 1.0, 2.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key("J", 2.0, 2.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key("K", 3.0, 2.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key("L", 4.0, 2.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key(":\n;", 5.0, 2.2, 1.0, 1.0, KeySection::RightHand, None);
        add_key("\"\n'", 6.0, 2.2, 1.5, 1.0, KeySection::RightHand, None);

        // Right hand - Row 3
        add_key("N", 1.0, 3.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key("M", 2.0, 3.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key("<\n,", 3.0, 3.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key(">\n.", 4.0, 3.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key("?\n/", 5.0, 3.2, 1.0, 1.0, KeySection::RightHand, None);
        add_key("RShift", 6.0, 3.2, 1.5, 1.0, KeySection::RightHand, None);

        // Right hand - Row 4 (bottom, KeySection::RightHand)
        add_key(
            "↑",
            2.0,
            4.0,
            1.0,
            1.0,
            KeySection::RightHand,
            Some(constants::UP_ICON),
        );
        add_key(
            "↓",
            3.0,
            4.0,
            1.0,
            1.0,
            KeySection::RightHand,
            Some(constants::DOWN_ICON),
        );
        add_key("{\n[", 4.0, 4.0, 1.0, 1.0, KeySection::RightHand, None);
        add_key("}\n]", 5.0, 4.2, 1.0, 1.0, KeySection::RightHand, None);
        add_key(
            "RFn",
            6.0,
            4.2,
            1.5,
            1.0,
            KeySection::RightHand,
            Some(constants::FN_ICON),
        );

        // Left thumb cluster - reorganized with consistent rows
        add_key("LCtrl", 1.0, 0.0, 1.0, 1.0, KeySection::LeftThumb, None);
        add_key("Alt", 2.0, 0.0, 1.0, 1.0, KeySection::LeftThumb, None);
        add_key(
            "Back\nSpace",
            0.0,
            1.0,
            1.0,
            2.0,
            KeySection::LeftThumb,
            None,
        ); // Double height
        add_key("Delete", 1.0, 1.0, 1.0, 2.0, KeySection::LeftThumb, None); // Double height
        add_key("Home", 2.0, 1.0, 1.0, 1.0, KeySection::LeftThumb, None);
        add_key("End", 2.0, 2.0, 1.0, 1.0, KeySection::LeftThumb, None);

        // Right thumb cluster - reorganized with consistent rows
        add_key(
            "Win",
            0.0,
            0.0,
            1.0,
            1.0,
            KeySection::RightThumb,
            Some(constants::WIN_ICON),
        );
        add_key("RCtrl", 1.0, 0.0, 1.0, 1.0, KeySection::RightThumb, None);
        add_key("Pg\nUp", 0.0, 1.0, 1.0, 1.0, KeySection::RightThumb, None);
        add_key("Pg\nDown", 0.0, 2.0, 1.0, 1.0, KeySection::RightThumb, None);
        add_key("Enter", 1.0, 1.0, 1.0, 2.0, KeySection::RightThumb, None); // Double height
        add_key("Space", 2.0, 1.0, 1.0, 2.0, KeySection::RightThumb, None); // Double height
    }

    fn draw(
        cr: &cairo::Context,
        width: i32,
        height: i32,
        keys: &[Key],
        hovered_label: Option<&str>,
    ) {
        // Clear background
        Self::set_color(cr, constants::BACKGROUND);
        let _ = cr.paint();

        // Calculate scale to fit, maintaining aspect ratio
        let scale_x = (width as f64 * 0.95) / constants::KEYBOARD_WIDTH;
        let scale_y = (height as f64 * 0.95) / constants::KEYBOARD_HEIGHT;
        let scale = scale_x.min(scale_y);

        // Center the keyboard
        let center_x = width as f64 / 2.0;
        let center_y = height as f64 / 2.0;

        // Apply transformations
        cr.save().unwrap();
        cr.translate(center_x, center_y);
        cr.scale(scale, scale);

        // Draw all sections
        for section in [
            KeySection::LeftHand,
            KeySection::RightHand,
            KeySection::LeftThumb,
            KeySection::RightThumb,
        ] {
            Self::draw_section(cr, keys, section, hovered_label);
        }

        cr.restore().unwrap();
    }

    fn draw_key(cr: &cairo::Context, key: &Key, hovered_label: Option<&str>) {
        // Determine if this is a home row key
        let is_home_row = matches!(
            key.label.as_str(),
            "A" | "S" | "D" | "F" | "G" | "H" | "J" | "K" | "L" | ":" | ";\n:"
        );

        let is_hovered = hovered_label == Some(&key.label);

        // Set key background color - dark slate gray
        if is_hovered {
            Self::set_color(cr, constants::KEY_BACKGROUND_HOVER);
        } else if key.remapped_label.is_some() {
            Self::set_color(cr, constants::KEY_BACKGROUND_REMAPPED);
        } else if is_home_row {
            Self::set_color(cr, constants::KEY_BACKGROUND_HOME_ROW);
        } else {
            Self::set_color(cr, constants::KEY_BACKGROUND_DEFAULT);
        }

        // Draw rounded rectangle
        Self::draw_rounded_rectangle(
            cr,
            key.x,
            key.y,
            key.width,
            key.height,
            constants::KEY_CORNER_RADIUS,
        );
        let _ = cr.fill_preserve();

        // Draw key border with slightly lighter gray
        Self::set_color(cr, constants::KEY_BORDER);
        cr.set_line_width(constants::KEY_BORDER_WIDTH);
        let _ = cr.stroke();

        if key.remapped_label.is_none() && key.svg_data.is_some() {
            Self::draw_svg_on_key(cr, key);
        } else {
            Self::draw_text_on_key(cr, key);
        }
    }

    fn draw_section(
        cr: &cairo::Context,
        keys: &[Key],
        section: KeySection,
        hovered_label: Option<&str>,
    ) {
        let (offset_x, offset_y, rotation) = Self::get_section_transform(section);

        cr.save().unwrap();
        cr.translate(offset_x, offset_y);

        // Apply rotation for thumb clusters
        if matches!(section, KeySection::LeftThumb | KeySection::RightThumb) {
            let center_x = 1.5 * (constants::KEY_SIZE + constants::KEY_GAP);
            let center_y = 1.5 * (constants::KEY_SIZE + constants::KEY_GAP);
            cr.translate(center_x, center_y);
            cr.rotate(rotation);
            cr.translate(-center_x, -center_y);
        }

        // Draw all keys in this section
        for key in keys.iter().filter(|k| k.section == section) {
            Self::draw_key(cr, key, hovered_label);
        }

        cr.restore().unwrap();
    }

    fn rgb_to_hex(color: constants::Color) -> String {
        format!(
            "#{:02x}{:02x}{:02x}",
            (color.0 * 255.0) as u8,
            (color.1 * 255.0) as u8,
            (color.2 * 255.0) as u8
        )
    }

    fn replace_svg_colors(svg: &str, new_color: &str) -> String {
        use regex::Regex;

        let hex_color_regex = Regex::new(r#"#[0-9a-fA-F]{3,8}"#).unwrap();

        let result = hex_color_regex.replace_all(svg, new_color);

        result
            .replace("black", new_color)
            .replace("white", new_color)
            .to_string()
    }
    fn draw_svg_on_key(cr: &cairo::Context, key: &Key) {
        if let Some(svg_data) = key.svg_data {
            let text_color_hex = Self::rgb_to_hex(constants::TEXT_PRIMARY);
            let modified_svg = Self::replace_svg_colors(svg_data, &text_color_hex);

            match rsvg::Loader::new().read_stream::<_, gio::File, gio::Cancellable>(
                &gio::MemoryInputStream::from_bytes(&glib::Bytes::from(modified_svg.as_bytes())),
                None,
                None,
            ) {
                Ok(handle) => {
                    let renderer = rsvg::CairoRenderer::new(&handle);

                    cr.save().unwrap();

                    let available_width = key.width - (constants::SVG_PADDING * 2.0);
                    let available_height = key.height - (constants::SVG_PADDING * 2.0);

                    match renderer.intrinsic_size_in_pixels() {
                        Some((svg_width, svg_height)) => {
                            let scale_x = available_width / svg_width;
                            let scale_y = available_height / svg_height;
                            let scale = scale_x.min(scale_y);

                            let x_offset = key.x
                                + constants::SVG_PADDING
                                + (available_width - svg_width * scale) / 2.0;
                            let y_offset = key.y
                                + constants::SVG_PADDING
                                + (available_height - svg_height * scale) / 2.0;

                            cr.translate(x_offset, y_offset);
                            cr.scale(scale, scale);

                            let viewport = cairo::Rectangle::new(0.0, 0.0, svg_width, svg_height);
                            renderer.render_document(cr, &viewport).ok();
                        }
                        None => {
                            // Try with a fixed viewport
                            let viewport =
                                cairo::Rectangle::new(0.0, 0.0, available_width, available_height);
                            renderer.render_document(cr, &viewport).ok();
                        }
                    }

                    cr.restore().unwrap();
                }
                Err(e) => {
                    eprintln!("Failed to load SVG for key {}: {:?}", key.label, e);
                }
            }
        }
    }

    fn set_color(cr: &cairo::Context, color: constants::Color) {
        cr.set_source_rgb(color.0, color.1, color.2);
    }

    fn draw_text_on_key(cr: &cairo::Context, key: &Key) {
        let display_text = if let Some(ref remapped) = key.remapped_label {
            remapped.as_str()
        } else {
            key.label.as_str()
        };

        let display_text = match display_text {
            "LShift" => "Shift",
            "RShift" => "Shift",
            "LCtrl" => "Ctrl",
            "RCtrl" => "Ctrl",
            _ => display_text,
        };

        if display_text.contains('\n') {
            let parts: Vec<&str> = display_text.split('\n').collect();

            Self::set_color(cr, constants::TEXT_PRIMARY);
            // Draw main label
            cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);

            // Top line (shifted symbol) - adjust font size based on length
            let top_font_size = if parts[0].len() > 1 {
                constants::MULTI_DIGIT_FONT_SIZE
            } else {
                constants::SINGLE_DIGIT_FONT_SIZE
            };
            cr.set_font_size(top_font_size);
            let top_extents = cr.text_extents(parts[0]).unwrap();
            cr.move_to(
                key.x + (key.width - top_extents.width()) / 2.0,
                key.y + key.height * constants::TEXT_TOP_POSITION,
            );
            let _ = cr.show_text(parts[0]);

            // Bottom line (main character) - adjust font size based on length
            let bottom_font_size = if parts[1].len() > 1 {
                constants::MULTI_DIGIT_FONT_SIZE
            } else {
                constants::SINGLE_DIGIT_FONT_SIZE
            };
            cr.set_font_size(bottom_font_size);
            let bottom_extents = cr.text_extents(parts[1]).unwrap();
            cr.move_to(
                key.x + (key.width - bottom_extents.width()) / 2.0,
                key.y + key.height * constants::TEXT_BOTTOM_POSITION,
            );
            let _ = cr.show_text(parts[1]);
        } else {
            // Draw main label
            Self::set_color(cr, constants::TEXT_PRIMARY);
            cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);

            // Adjust font size based on label length
            let font_size = if display_text.len() > 1 {
                constants::MULTI_DIGIT_FONT_SIZE
            } else {
                constants::SINGLE_DIGIT_FONT_SIZE
            };
            cr.set_font_size(font_size);

            let extents = cr.text_extents(display_text).unwrap();
            cr.move_to(
                key.x + (key.width - extents.width()) / 2.0,
                key.y + (key.height + extents.height()) / 2.0,
            );
            let _ = cr.show_text(display_text);
        }
    }
    fn draw_rounded_rectangle(
        cr: &cairo::Context,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
    ) {
        let degrees = std::f64::consts::PI / 180.0;

        cr.new_path();
        cr.arc(
            x + width - radius,
            y + radius,
            radius,
            -90.0 * degrees,
            0.0 * degrees,
        );
        cr.arc(
            x + width - radius,
            y + height - radius,
            radius,
            0.0 * degrees,
            90.0 * degrees,
        );
        cr.arc(
            x + radius,
            y + height - radius,
            radius,
            90.0 * degrees,
            180.0 * degrees,
        );
        cr.arc(
            x + radius,
            y + radius,
            radius,
            180.0 * degrees,
            270.0 * degrees,
        );
        cr.close_path();
    }

    pub fn widget(&self) -> &DrawingArea {
        &self.drawing_area
    }
}
