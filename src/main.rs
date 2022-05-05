use gobble::traits::*;
use jparse::*;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;

    let v = JsonValue.parse_s(&buffer).map_err(|e| e.strung())?;
    println!("{:?}", &v);

    Ok(())
}
