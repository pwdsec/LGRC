use std::io::Write;

pub fn clear_screen() {
    let mut stdout = std::io::stdout();
    stdout.write_all(b"\x1b[2J").unwrap();
    stdout.write_all(b"\x1b[1;1H").unwrap();
}