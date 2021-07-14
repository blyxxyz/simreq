fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let url = std::env::args()
        .nth(1)
        .ok_or("Pass URL as first parameter")?;
    let resp = reqwest::blocking::get(url)?;
    println!("{:#?}", resp);
    Ok(())
}
