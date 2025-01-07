use prettytable::{Attr, Cell, Row, Table};
use fontmanager::{FontManager, FontSourceType};

fn main() {
    colog::init();

    let arg = std::env::args().nth(1);
    let binding = arg.unwrap_or("".into());
    let pattern = binding.as_str();

    let manager = FontManager::new();

    for source_type in manager.sources() {
        render_table(source_type, &manager, pattern);
    }
}

#[allow(unused)]
fn render_table(source_type: FontSourceType, manager: &FontManager, family: &str) {
    println!("source_type: {:?}", source_type);

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

    for info in manager.available_fonts(source_type) {
        if !family.is_empty() {
            let fam = info.family.to_ascii_lowercase();
            if !fam.contains(&family.to_ascii_lowercase()) {
                continue;
            }
        }
        table.add_row(Row::new(vec![
            Cell::new(&info.family),
            Cell::new(&format!("{}", &info.style)),
            Cell::new(&info.weight.to_string()),
            Cell::new(&info.stretch.to_string()),
            Cell::new(&info.monospaced.to_string()),
            if info.path.is_some() {
                Cell::new(&info.path.unwrap().to_str().unwrap())
            } else {
                Cell::new("N/A")
            },
            Cell::new(&info.index.unwrap_or(0).to_string()),
        ]));
    }

    table.printstd();
    println!("\n\n\n");
}