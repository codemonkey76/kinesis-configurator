use gtk4::DrawingArea;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// Key positions for the Kinesis Advantage 360 layout
// (x, y, width, height, label)
type KeyDef = (f64, f64, f64, f64, &'static str);

const KEY_SIZE: f64 = 50.0;
const KEY_SPACING: f64 = 4.0;
const SPLIT_GAP: f64 = 100.0;

pub struct KeyboardView {
    drawing_area: DrawingArea,
    selected_key: Rc<RefCell<Option<String>>>,
    remaps: Rc<RefCell<HashMap<String, String>>>,
}

impl KeyboardView {
    pub fn new() -> Self {
        let drawing_area = DrawingArea::new();
        let selected_key = Rc::new(RefCell::new(None));
        let remaps = Rc::new(RefCell::new(HashMap::new()));

        let selected_key_clone = selected_key.clone();
        let remaps_clone = remaps.clone();

        drawing_area.set_draw_func(move |_, cr, width, height| {
            Self::draw_keyboard(cr, width, height, &selected_key_clone, &remaps_clone);
        });

        // Add click handler
        let gesture = gtk4::GestureClick::new();
        let selected_key_clone = selected_key.clone();
        let drawing_area_clone = drawing_area.clone();

        gesture.connect_pressed(move |_, _, x, y| {
            if let Some(key) = Self::get_key_at_position(x, y) {
                *selected_key_clone.borrow_mut() = Some(key);
                drawing_area_clone.queue_draw();
            }
        });

        drawing_area.add_controller(gesture);

        KeyboardView {
            drawing_area,
            selected_key,
            remaps,
        }
    }

    pub fn set_remaps(&self, new_remaps: HashMap<String, String>) {
        *self.remaps.borrow_mut() = new_remaps;
        self.drawing_area.queue_draw();
    }

    fn draw_keyboard(
        cr: &cairo::Context,
        width: i32,
        height: i32,
        selected_key: &Rc<RefCell<Option<String>>>,
        remaps: &Rc<RefCell<HashMap<String, String>>>,
    ) {
        // Background
        cr.set_source_rgb(0.95, 0.95, 0.95);
        let _ = cr.paint();

        // Calculate total keyboard width
        let left_hand_width = 7.0 * (KEY_SIZE + KEY_SPACING);
        let right_hand_width = 7.0 * (KEY_SIZE + KEY_SPACING);
        let total_width = left_hand_width + SPLIT_GAP + right_hand_width;

        // Calculate total keyboard height (approximate)
        let total_height = 8.0 * (KEY_SIZE + KEY_SPACING);

        // Center horizontally and vertically
        let start_x = (width as f64 - total_width) / 2.0;
        let start_y = (height as f64 - total_height) / 2.0;

        // Draw left hand
        Self::draw_hand(cr, start_x, start_y, true, selected_key, remaps);

        // Draw right hand
        Self::draw_hand(
            cr,
            start_x + left_hand_width + SPLIT_GAP,
            start_y,
            false,
            selected_key,
            remaps,
        );
    }

    fn draw_hand(
        cr: &cairo::Context,
        start_x: f64,
        start_y: f64,
        is_left: bool,
        selected_key: &Rc<RefCell<Option<String>>>,
        remaps: &Rc<RefCell<HashMap<String, String>>>,
    ) {
        let keys = if is_left {
            Self::get_left_hand_keys()
        } else {
            Self::get_right_hand_keys()
        };

        let selected = selected_key.borrow();
        let remap_map = remaps.borrow();

        for (x, y, w, h, label) in keys {
            let key_x = start_x + x * (KEY_SIZE + KEY_SPACING);
            let key_y = start_y + y * (KEY_SIZE + KEY_SPACING);

            // Use remapped value if it exists
            let display_label = remap_map.get(label).map(|s| s.as_str()).unwrap_or(label);

            // Check if this key is selected or remapped
            let is_selected = selected.as_ref().map(|s| s == label).unwrap_or(false);
            let is_remapped = remap_map.contains_key(label);

            // Draw key background
            if is_selected {
                cr.set_source_rgb(0.3, 0.5, 0.8);
            } else if is_remapped {
                cr.set_source_rgb(0.9, 0.95, 1.0); // Light blue for remapped keys
            } else {
                cr.set_source_rgb(1.0, 1.0, 1.0);
            }

            cr.rectangle(
                key_x,
                key_y,
                w * KEY_SIZE + (w - 1.0) * KEY_SPACING,
                h * KEY_SIZE + (h - 1.0) * KEY_SPACING,
            );
            let _ = cr.fill();

            // Draw key border
            cr.set_source_rgb(0.3, 0.3, 0.3);
            cr.set_line_width(1.5);
            cr.rectangle(
                key_x,
                key_y,
                w * KEY_SIZE + (w - 1.0) * KEY_SPACING,
                h * KEY_SIZE + (h - 1.0) * KEY_SPACING,
            );
            let _ = cr.stroke();

            // Draw key label
            if is_selected {
                cr.set_source_rgb(1.0, 1.0, 1.0);
            } else {
                cr.set_source_rgb(0.2, 0.2, 0.2);
            }
            cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
            cr.set_font_size(10.0);

            // Handle multi-line labels
            let lines: Vec<&str> = display_label.split('\n').collect();
            let line_height = 14.0;
            let total_height = lines.len() as f64 * line_height;
            let start_text_y =
                key_y + (h * KEY_SIZE + (h - 1.0) * KEY_SPACING - total_height) / 2.0 + line_height;

            for (i, line) in lines.iter().enumerate() {
                let extents = cr.text_extents(line).unwrap();
                let text_x =
                    key_x + (w * KEY_SIZE + (w - 1.0) * KEY_SPACING - extents.width()) / 2.0;
                let text_y = start_text_y + i as f64 * line_height;

                let _ = cr.move_to(text_x, text_y);
                let _ = cr.show_text(line);
            }
        }
    }

    fn get_left_hand_keys() -> Vec<KeyDef> {
        vec![
            // Number row
            (0.0, 0.2, 1.0, 1.0, "+\n="),
            (1.0, 0.3, 1.0, 1.0, "!\n1"),
            (2.0, 0.0, 1.0, 1.0, "@\n2"),
            (3.0, 0.0, 1.0, 1.0, "#\n3"),
            (4.0, 0.0, 1.0, 1.0, "$\n4"),
            (5.0, 0.0, 1.0, 1.0, "%\n5"),
            (6.0, 0.0, 1.0, 1.0, "Kp"),
            // Top row
            (0.0, 1.2, 1.0, 1.0, "Tab"),
            (1.0, 1.3, 1.0, 1.0, "Q"),
            (2.0, 1.0, 1.0, 1.0, "W"),
            (3.0, 1.0, 1.0, 1.0, "E"),
            (4.0, 1.0, 1.0, 1.0, "R"),
            (5.0, 1.0, 1.0, 1.0, "T"),
            (6.0, 1.0, 1.0, 1.0, "Hk1"),
            // Home row
            (0.0, 2.2, 1.0, 1.0, "Esc"),
            (1.0, 2.3, 1.0, 1.0, "A"),
            (2.0, 2.0, 1.0, 1.0, "S"),
            (3.0, 2.0, 1.0, 1.0, "D"),
            (4.0, 2.0, 1.0, 1.0, "F"),
            (5.0, 2.0, 1.0, 1.0, "G"),
            (6.0, 2.0, 1.0, 1.0, "Hk2"),
            // Lower alpha row
            (0.0, 3.2, 1.0, 1.0, "Shift"),
            (1.0, 3.3, 1.0, 1.0, "Z"),
            (2.0, 3.0, 1.0, 1.0, "X"),
            (3.0, 3.0, 1.0, 1.0, "C"),
            (4.0, 3.0, 1.0, 1.0, "V"),
            (5.0, 3.0, 1.0, 1.0, "B"),
            // Bottom row
            (0.0, 4.2, 1.0, 1.0, "Fn"),
            (1.0, 4.3, 1.0, 1.0, "~\n`"),
            (2.0, 4.0, 1.0, 1.0, "Caps"),
            (3.0, 4.0, 1.0, 1.0, "Left"),
            (4.0, 4.0, 1.0, 1.0, "Right"),
            // Thumb cluster
            (6.5, 3.5, 1.0, 1.0, "Ctrl"),
            (7.5, 3.5, 1.0, 1.0, "Alt"),
            (5.5, 4.5, 1.0, 2.0, "Back\nSpace"),
            (6.5, 4.5, 1.0, 2.0, "Del"),
            (7.5, 4.5, 1.0, 1.0, "Home"),
            (7.5, 5.5, 1.0, 1.0, "End"),
        ]
    }

    fn get_right_hand_keys() -> Vec<KeyDef> {
        vec![
            // Number row
            (1.5, 0.0, 1.0, 1.0, "Ss"),
            (2.5, 0.0, 1.0, 1.0, "6"),
            (3.5, 0.0, 1.0, 1.0, "7"),
            (4.5, 0.0, 1.0, 1.0, "8"),
            (5.5, 0.0, 1.0, 1.0, "9"),
            (6.5, 0.0, 1.0, 1.0, "0"),
            (7.5, 0.0, 1.5, 1.0, "-\n_"),
            // Top row
            (1.5, 1.0, 1.0, 1.0, "Hk3"),
            (2.5, 1.0, 1.0, 1.0, "Y"),
            (3.5, 1.0, 1.0, 1.0, "U"),
            (4.5, 1.0, 1.0, 1.0, "I"),
            (5.5, 1.0, 1.0, 1.0, "O"),
            (6.5, 1.0, 1.0, 1.0, "P"),
            (7.5, 1.0, 1.5, 1.0, "|\n\\"),
            // Home row
            (1.5, 2.0, 1.0, 1.0, "Hk4"),
            (2.5, 2.0, 1.0, 1.0, "H"),
            (3.5, 2.0, 1.0, 1.0, "J"),
            (4.5, 2.0, 1.0, 1.0, "K"),
            (5.5, 2.0, 1.0, 1.0, "L"),
            (6.5, 2.0, 1.0, 1.0, ";"),
            (7.5, 2.0, 1.5, 1.0, "\"\n'"),
            // Lower alpha row
            (2.5, 3.0, 1.0, 1.0, "N"),
            (3.5, 3.0, 1.0, 1.0, "M"),
            (4.5, 3.0, 1.0, 1.0, ","),
            (5.5, 3.0, 1.0, 1.0, "."),
            (6.5, 3.0, 1.0, 1.0, "/"),
            (7.5, 3.0, 1.5, 1.0, "Shift"),
            // Bottom row
            (3.5, 4.0, 1.0, 1.0, "Up"),
            (4.5, 4.0, 1.0, 1.0, "Down"),
            (5.5, 4.0, 1.0, 1.0, "{\n["),
            (6.5, 4.0, 1.0, 1.0, "}\n]"),
            (7.5, 4.0, 1.5, 1.0, "Fn"),
            // Thumb cluster
            (0.0, 3.5, 1.0, 1.0, "Win"),
            (1.0, 3.5, 1.0, 1.0, "Ctrl"),
            (0.0, 4.5, 1.0, 1.0, "Page\nUp"),
            (1.0, 4.5, 1.0, 2.0, "Enter"),
            (0.0, 5.5, 1.0, 1.0, "Page\nDown"),
            (2.0, 4.5, 1.0, 2.0, "Space"),
        ]
    }

    fn get_key_at_position(x: f64, y: f64) -> Option<String> {
        // Check left hand
        for (kx, ky, w, h, label) in Self::get_left_hand_keys() {
            let key_x = 20.0 + kx * (KEY_SIZE + KEY_SPACING);
            let key_y = 50.0 + ky * (KEY_SIZE + KEY_SPACING);
            let key_w = w * KEY_SIZE + (w - 1.0) * KEY_SPACING;
            let key_h = h * KEY_SIZE;

            if x >= key_x && x <= key_x + key_w && y >= key_y && y <= key_y + key_h {
                return Some(label.to_string());
            }
        }

        // Check right hand
        let right_start = 20.0 + 7.0 * (KEY_SIZE + KEY_SPACING) + SPLIT_GAP;
        for (kx, ky, w, h, label) in Self::get_right_hand_keys() {
            let key_x = right_start + kx * (KEY_SIZE + KEY_SPACING);
            let key_y = 50.0 + ky * (KEY_SIZE + KEY_SPACING);
            let key_w = w * KEY_SIZE + (w - 1.0) * KEY_SPACING;
            let key_h = h * KEY_SIZE;

            if x >= key_x && x <= key_x + key_w && y >= key_y && y <= key_y + key_h {
                return Some(label.to_string());
            }
        }

        None
    }

    pub fn widget(&self) -> &DrawingArea {
        &self.drawing_area
    }

    pub fn get_selected_key(&self) -> Option<String> {
        self.selected_key.borrow().clone()
    }
}
