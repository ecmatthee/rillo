use std::io;

pub fn get_string() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(())
}
