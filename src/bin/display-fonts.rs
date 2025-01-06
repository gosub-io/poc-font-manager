use prettytable::{Attr, Cell, Row, Table};
use fontmanager::font_manager::FontManager;

fn main() {
    colog::init();

    let manager = FontManager::new();

    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(Row::new(vec![
        Cell::new("Family").with_style(Attr::Bold),
        Cell::new("Style").with_style(Attr::Bold),
        Cell::new("Weight").with_style(Attr::Bold),
        Cell::new("Stretch").with_style(Attr::Bold),
        Cell::new("Monospaced").with_style(Attr::Bold),
        Cell::new("Path").with_style(Attr::Bold),
        Cell::new("Index").with_style(Attr::Bold),
    ]));

    for info in manager.available_fonts() {
        table.add_row(Row::new(vec![
            Cell::new(&info.family),
            Cell::new(&format!("{}", &info.style)),
            Cell::new(&info.weight.to_string()),
            Cell::new(&info.stretch.to_string()),
            Cell::new(&info.monospaced.to_string()),
            Cell::new(&info.path.to_str().unwrap()),
            Cell::new(&info.index.unwrap_or(0).to_string()),
        ]));
    }

    table.printstd();
}