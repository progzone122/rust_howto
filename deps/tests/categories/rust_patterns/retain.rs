// ANCHOR: example

fn parse_response(
    response: reqwest::blocking::Response,
) -> anyhow::Result<u32> {
    let body = response.text()?;
    let body = body.trim();
    // println!("Body: {body}");
    let b = body.parse::<u32>()?;
    Ok(b)
}

fn main() -> anyhow::Result<()> {
    let url = "https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain".to_string();
    let response = reqwest::blocking::get(url)?;
    let random_value: u32 = parse_response(response)?;
    println!("A random number between 0 and 10: {}", random_value);
    Ok(())
}
// ANCHOR_END: example

// TODO P1 flaky test
#[ignore]
#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
