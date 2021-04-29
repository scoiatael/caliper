use std::path::Path;
use svg::Document;

use crate::bezier;

pub fn run(curves: &bezier::Curves, filename: &Path) -> std::io::Result<()> {
    let mut document = Document::new().set("viewBox", (0, 0, 1024, 768));
    for curve in curves.iter() {
        document = document.add(curve.export());
    }
    svg::save(filename, &document)
}
