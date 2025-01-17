use gtk4::{glib, pango, Application, ApplicationWindow, DrawingArea};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, DrawingAreaExt, DrawingAreaExtManual, GtkWindowExt, WidgetExt};
use image::Rgba;
use parley::layout::{Alignment, Layout, PositionedLayoutItem};
use parley::style::{FontWeight, StyleProperty};
use parley::{InlineBox, LayoutContext};
use gosub_fontmanager::{FontInfo, FontManager, FontSourceType, FontStyle};

const RENDER_GLYPHS_PER_RUN : bool = false;

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

    let manager = FontManager::new();
    let font_info = manager.find(FontSourceType::Fontkit, &["comic sans ms"], FontStyle::Normal).unwrap();

    // let text = "Some text here. Let's make it a bit longer so that line wrapping kicks in 😊. And also some اللغة العربية arabic text.\nThis is underline and strikethrough text";
    // let text = "hello world. This is a test to see if it works! abcdefghhijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456790";
    let text = gosub_fontmanager::flatland::TEXT;

    let area = DrawingArea::default();
    area.set_hexpand(true);
    area.set_vexpand(true);
    area.set_draw_func(move |area, cr, width, _height| {
        // Red square to indicate stuff is being drawn on screen
        cr.set_source_rgba(1.0, 0.0, 0.0, 1.0);
        cr.rectangle(0.0, 0.0, 100.0, 100.0);
        let _ = cr.fill();

        let mut height = 100.0;

        for fs in [32.0, 16.0, 8.0, 4.0] {
            // Create layout with parley
            let layout = create_layout(&manager, &font_info, text, width as f64, fs);
            let h = layout.height();

            // Draw the layout with pango
            draw(&manager, &font_info, &cr, layout, 100.0, height);
            height += h + 50.0;
        }

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

fn draw(manager: &FontManager, font: &FontInfo, cr: &gtk4::cairo::Context, layout: Layout<ColorBrush>, offset_x: f32, offset_y: f32) {
    for line in layout.lines() {
        for item in line.items() {
            match item {
                PositionedLayoutItem::GlyphRun(glyph_run) => {
                    let run_x = offset_x + glyph_run.offset();
                    let run_y = offset_y + glyph_run.baseline();

                    // @Todo: how do we find the height of the font. We need this because the baseline
                    // does only tell us the start of the bottom (the actual baseline). But we do not
                    // have any (easy) way to find the height of the font. We need this to draw the
                    // rectangle around the text.

                    // let font_height = 48.0; // completely arbitrary value
                    // cr.rectangle(
                    //     run_x as f64,
                    //     run_y as f64 - font_height,
                    //     glyph_run.advance() as f64,
                    //     font_height,
                    // );
                    // cr.set_source_rgba(rng.random(), rng.random(), rng.random(), 0.5);
                    // let _ = cr.fill();


                    let pango = manager.find_pango();
                    let font = pango.load_font(&font).unwrap();

                    if RENDER_GLYPHS_PER_RUN {
                        // Render a whole glyph run at once. This does not work correctly

                        // convert glyph_run.glyphs() to a vector of GlyphInfo
                        let mut glyphs = Vec::new();
                        for g in glyph_run.positioned_glyphs() {
                            glyphs.push(g);
                        }
                        dbg!(&glyphs);

                        let mut gs = pango::GlyphString::new();
                        gs.set_size(glyphs.len() as i32);
                        for (i, glyph) in glyphs.iter().enumerate() {
                            let m = gs.glyph_info_mut();
                            m[i].set_glyph(glyph.id as u32);
                            m[i].geometry_mut().set_x_offset(glyph.x as i32);
                            m[i].geometry_mut().set_y_offset(glyph.y as i32);
                        }

                        cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
                        cr.move_to(run_x as f64, run_y as f64);
                        pangocairo::functions::show_glyph_string(
                            cr,
                            &font,
                            &mut gs,
                        );
                    } else {
                        // Render per glyph
                        let mut c_x = run_x;
                        for g in glyph_run.glyphs() {
                            let g_x = c_x + g.x;
                            let g_y = run_y + g.y;
                            c_x += g.advance;

                            let mut gs = pango::GlyphString::new();
                            gs.set_size(1);
                            let m = gs.glyph_info_mut();
                            m[0].set_glyph(g.id as u32);

                            cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
                            cr.move_to(g_x as f64, g_y as f64);
                            pangocairo::functions::show_glyph_string(
                                cr,
                                &font,
                                &mut gs,
                            );
                        }
                    }
                }
                PositionedLayoutItem::InlineBox(inline_box) => {
                    cr.rectangle(
                        (offset_x + inline_box.x) as f64,
                         (offset_y + inline_box.y) as f64,
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

fn create_layout(manager: &FontManager, font: &FontInfo, text: &str, width: f64, font_size: f64) -> Layout<ColorBrush> {
    let display_scale = 1.0_f32;
    let max_advance = Some(width as f32 * display_scale);

    let text_color = Rgba([0, 0, 0, 255]);

    let mut layout_cx = LayoutContext::new();

    let text_brush = ColorBrush { color: text_color };
    let brush_style = StyleProperty::Brush(text_brush);
    let bold_style = StyleProperty::FontWeight(FontWeight::EXTRA_BLACK);
    // let underline_style = StyleProperty::Underline(true);
    // let strikethrough_style = StyleProperty::Strikethrough(true);

    let parley = manager.find_parley();
    let font_stack = parley.get_font_stack(font.family.clone());
    let binding = parley.context();
    let mut font_context = binding.borrow_mut();

    let mut builder = layout_cx.ranged_builder(&mut font_context, &text, display_scale);
    builder.push_default(brush_style);
    builder.push_default(font_stack);
    builder.push_default(StyleProperty::LineHeight(1.3));
    builder.push_default(StyleProperty::FontSize(font_size as f32));
    builder.push_default(StyleProperty::LetterSpacing(5.0));

    builder.push(bold_style, 6..11);
    // builder.push(underline_style, 141..150);
    // builder.push(strikethrough_style, 155..168);

    builder.push_inline_box(InlineBox {
        id: 0,
        index: 5,
        width: 100.0,
        height: 100.0,
    });

    builder.push_inline_box(InlineBox {
        id: 1,
        index: 50,
        width: 100.0,
        height: 30.0,
    });

    let mut layout: Layout<ColorBrush> = builder.build(text);

    layout.break_all_lines(max_advance);
    layout.align(max_advance, Alignment::Start);

    layout
}
