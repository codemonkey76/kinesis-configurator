use gtk4::prelude::*;
use gtk4::{DrawingArea, glib};

#[derive(Debug)]
pub struct KeyboardView {
    drawing_area: DrawingArea,
}

impl KeyboardView {
    pub fn new() -> Self {
        let drawing_area = DrawingArea::new();
        drawing_area.set_content_width(1000);
        drawing_area.set_content_height(500);
        drawing_area.set_hexpand(true);
        drawing_area.set_vexpand(true);

        drawing_area.set_draw_func(|_, cr, width, height| {
            Self::draw(cr, width, height);
        });

        Self { drawing_area }
    }

    fn draw(cr: &cairo::Context, width: i32, height: i32) {
        // Clear background
        cr.set_source_rgb(0.95, 0.95, 0.95);
        let _ = cr.paint();

        // Calculate center and spacing
        let center_x = width as f64 / 2.0;
        let center_y = height as f64 / 2.0;

        let split_gap = 80.0;

        // Left hand main section
        Self::draw_section(
            cr,
            center_x - split_gap - 250.0,
            center_y - 100.0,
            250.0,
            200.0,
            "Left Hand",
            (0.8, 0.9, 1.0),
        );

        // Right hand main section
        Self::draw_section(
            cr,
            center_x + split_gap,
            center_y - 100.0,
            250.0,
            200.0,
            "Right Hand",
            (1.0, 0.9, 0.8),
        );

        // Left thumb cluster
        Self::draw_section(
            cr,
            center_x - split_gap - 150.0,
            center_y + 120.0,
            140.0,
            80.0,
            "Left Thumb",
            (0.9, 0.8, 1.0),
        );
        // Right thumb cluster
        Self::draw_section(
            cr,
            center_x + split_gap + 10.0,
            center_y + 120.0,
            140.0,
            80.0,
            "Left Thumb",
            (1.0, 1.0, 0.8),
        );

        // Draw center info
        cr.set_source_rgb(0.5, 0.5, 0.5);
        cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
        cr.set_font_size(12.0);
        cr.move_to(center_x - 60.0, center_y);
        let _ = cr.show_text("Kinesis Advantage 360");
    }

    fn draw_section(
        cr: &cairo::Context,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        label: &str,
        color: (f64, f64, f64),
    ) {
        // Draw filled rectangle with color
        cr.set_source_rgb(color.0, color.1, color.2);
        cr.rectangle(x, y, width, height);
        let _ = cr.fill_preserve();

        // Draw border
        cr.set_source_rgb(0.3, 0.3, 0.3);
        cr.set_line_width(2.0);
        let _ = cr.stroke();

        // Draw label
        cr.set_source_rgb(0.2, 0.2, 0.2);
        cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
        cr.set_font_size(16.0);

        let extents = cr.text_extents(label).unwrap();
        cr.move_to(
            x + (width - extents.width()) / 2.0,
            y + (height + extents.height()) / 2.0,
        );
        let _ = cr.show_text(label);
    }

    pub fn widget(&self) -> &DrawingArea {
        &self.drawing_area
    }
}

impl Default for KeyboardView {
    fn default() -> Self {
        Self::new()
    }
}
