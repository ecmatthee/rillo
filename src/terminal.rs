use crossterm::terminal;

pub fn term_size() -> (u16, u16) {
    match terminal::size() {
        Ok(i) => i,
        Err(_e) => (1, 1),
    }
}
