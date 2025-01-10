use std::ffi::c_ulong;
use gtk4::{glib, Application, ApplicationWindow, DrawingArea};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, DrawingAreaExt, DrawingAreaExtManual, GtkWindowExt, WidgetExt};
use image::Rgba;
use parley::layout::{Alignment, Layout, PositionedLayoutItem};
use parley::style::{FontStack, FontWeight, StyleProperty};
use parley::{FontContext, InlineBox, LayoutContext};

#[derive(Clone, Copy, Debug, PartialEq)]
struct ColorBrush {
    color: Rgba<u8>,
}

impl Default for ColorBrush {
    fn default() -> Self {
        Self {
            color: Rgba([0, 0, 0, 255]),
        }
    }
}


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
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK Font Renderer")
        .build();

    let area = DrawingArea::default();
    area.set_hexpand(true);
    area.set_vexpand(true);
    area.set_draw_func(move |area, cr, width, _height| {
        // Red square to indicate stuff is being drawn on screen
        cr.set_source_rgba(1.0, 0.0, 0.0, 1.0);
        cr.rectangle(0.0, 0.0, 100.0, 100.0);
        let _ = cr.fill();

        let layout = create_layout("Hello, world!", width as f64);
        let height = layout.height();

        draw(&cr, layout);

        // Get current position and add the layout height. This is the new height of the canvas in this drawing area so
        // we can scroll.
        area.set_content_height(height as i32 + 50);
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

fn draw(cr: &gtk4::cairo::Context, layout: Layout<ColorBrush>) {
    let mut x_off = 0.0;
    for line in layout.lines() {
        for item in line.items() {
            match item {
                PositionedLayoutItem::GlyphRun(glyph_run) => {
                    for g in glyph_run.glyphs() {
                        println!("GlyphID: {}", g.id);
                        let cg = gtk4::cairo::Glyph::new(
                            g.id as u32 as c_ulong,
                            x_off + g.x as f64,
                            g.y as f64,
                        );
                        let _ = cr.show_glyphs(&[cg]);

                        x_off += g.advance as f64;

                        // let x = g.x as f64;
                        // let y = g.y as f64;
                        // let width = g. as f64;
                        // let height = g.height as f64;
                        // cr.rectangle(x, y, g.advance as f64, g.advance as f64);
                        // cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
                        // let _ = cr.stroke();
                        // cr.set_source_rgba(0.0, 0.0, 1.0, 1.0);
                        // let _ = cr.fill();
                    }
                    // render_glyph_run(&mut scale_cx, &glyph_run, &mut img, padding);
                }
                PositionedLayoutItem::InlineBox(inline_box) => {
                    cr.rectangle(
                        inline_box.x as f64 + 20.0,
                        inline_box.y as f64 + 20.0,
                        inline_box.width as f64,
                        inline_box.height as f64,
                    );
                    cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
                    let _ = cr.stroke();
                    cr.set_source_rgba(0.0, 0.0, 1.0, 1.0);
                    let _ = cr.fill();
                }
            };
        }
    }
}

fn create_layout(text: &str, width: f64) -> Layout<ColorBrush> {
    let display_scale = 1.0_f32;
    let max_advance = Some(width as f32 * display_scale);

    let text_color = Rgba([0, 0, 0, 255]);

    let mut font_cx = FontContext::new();
    let font_stack = FontStack::from("comic sans ms");

    // let fontmanager = fontmanager::font_manager::FontManager::new();
    // let Some(font_info) = fontmanager.find(vec!["arial"], FontStyle::Normal) else {
    //     panic!("Font not found");
    // };

    let mut layout_cx = LayoutContext::new();

    let text_brush = ColorBrush { color: text_color };
    let brush_style = StyleProperty::Brush(text_brush);
    let bold_style = StyleProperty::FontWeight(FontWeight::EXTRA_BLACK);
    let underline_style = StyleProperty::Underline(true);
    let strikethrough_style = StyleProperty::Strikethrough(true);

    let mut builder = layout_cx.ranged_builder(&mut font_cx, &text, display_scale as f32);
    builder.push_default(brush_style);
    builder.push_default(font_stack);
    builder.push_default(StyleProperty::LineHeight(1.3));
    builder.push_default(StyleProperty::FontSize(16.0));

    builder.push(bold_style, 4..8);
    builder.push(underline_style, 141..150);
    builder.push(strikethrough_style, 155..168);

    builder.push_inline_box(InlineBox {
        id: 0,
        index: 0,
        width: 50.0,
        height: 50.0,
    });

    builder.push_inline_box(InlineBox {
        id: 1,
        index: 50,
        width: 50.0,
        height: 30.0,
    });

    let mut layout: Layout<ColorBrush> = builder.build(text);

    layout.break_all_lines(max_advance);
    layout.align(max_advance, Alignment::Start);

    layout
}
