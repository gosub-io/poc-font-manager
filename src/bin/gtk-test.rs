use gtk4::{glib, Application, ApplicationWindow, DrawingArea};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, DrawingAreaExt, DrawingAreaExtManual, GtkWindowExt, WidgetExt};
use pangocairo::functions::{create_layout, show_layout};
use pangocairo::pango;
use fontmanager::{FontManager, FontSourceType, FontStyle};

const APP_ID: &str = "io.gosub.font-manager.gtk-test";

fn main() -> glib::ExitCode {
    colog::init();

    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);

    app.connect_startup(|_app| {
        println!("Setting default icon");
        gtk4::Window::set_default_icon_name(APP_ID);
    });

    app.run()
}

fn build_ui(app: &Application) {
    let font_manager = FontManager::new();
    let _ = font_manager.find(FontSourceType::Pango,&["comic sans ms"], FontStyle::Normal).expect("Failed to find font Comic Sans MS");
    let _ = font_manager.find(FontSourceType::Pango, &["Arial"], FontStyle::Normal).expect("Failed to find font Arial");

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK Font Renderer")
        .build();

    let area = DrawingArea::default();
    area.set_hexpand(true);
    area.set_vexpand(true);
    area.set_draw_func(move |area, gtk_cr, width, _height| {
        let fi_comic = font_manager.find(FontSourceType::Pango,&["comic sans ms"], FontStyle::Normal).expect("Failed to find font");
        let fi_arial = font_manager.find(FontSourceType::Pango,&["arial"], FontStyle::Normal).expect("Failed to find font");

        // Red square to indicate stuff is being drawn on screen
        gtk_cr.set_source_rgba(1.0, 0.0, 0.0, 1.0);
        gtk_cr.rectangle(0.0, 0.0, 100.0, 100.0);
        let _ = gtk_cr.fill();


        // Layout works nicely with bounding boxes and alignment, but i can't seem to get the fontface to render
        let layout = create_layout(gtk_cr);

        let pango = font_manager.find_pango();
        let desc = pango.get_description(&fi_comic, 14.0);
        layout.set_font_description(Some(&desc));

        layout.set_text(fontmanager::flatland::TEXT);
        layout.set_width(width * pango::SCALE);
        layout.set_alignment(pango::Alignment::Center);

        let cur_y = 200;
        let mut max_y = cur_y;

        // Create layout
        gtk_cr.set_source_rgba(1.0, 0.0, 1.0, 1.0);
        gtk_cr.move_to(0.0, cur_y as f64);
        show_layout(&gtk_cr, &layout);
        max_y += layout.pixel_size().1;

        // Nice bounding rectangle around the text
        gtk_cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
        gtk_cr.set_line_width(1.0);
        gtk_cr.rectangle(0.0, cur_y as f64, width as f64, max_y as f64 - cur_y as f64);
        let _ = gtk_cr.stroke();

        // Add a little bit of padding
        max_y += 25;
        let cur_y = max_y;

        // Display the next text in a different font

        let pango = font_manager.find_pango();
        let desc = pango.get_description(&fi_arial, 14.0);
        layout.set_font_description(Some(&desc));
        gtk_cr.set_source_rgba(0.7, 0.2, 0.5, 1.0);
        gtk_cr.move_to(0.0, cur_y as f64);
        show_layout(&gtk_cr, &layout);
        max_y += layout.pixel_size().1;

        // Bounding box around the text again
        gtk_cr.set_source_rgba(0.0, 1.0, 1.0, 1.0);
        gtk_cr.set_line_width(3.0);
        gtk_cr.rectangle(0.0, cur_y as f64, width as f64, max_y as f64 - cur_y as f64);
        let _ = gtk_cr.stroke();

        // Get current position and add the layout height. This is the new height of the canvas in this drawing area so
        // we can scroll.
        area.set_content_height(max_y as i32 + 50);
    });

    // Of course, scrolling doesn't work... need to figure out why it doesn't work.
    let scroll = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Automatic)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .child(&area)
        .build();
    window.set_child(Some(&scroll));

    window.set_default_width(800);
    window.set_default_height(600);
    window.present();
}