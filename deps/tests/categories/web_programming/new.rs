// ANCHOR: example
use url::ParseError;
use url::Url;

fn build_github_url(path: &str) -> Result<Url, ParseError> {
    const GITHUB: &str = "https://github.com";

    let base =
        Url::parse(GITHUB).expect("This hardcoded URL is known to be valid");
    let joined = base.join(path)?;

    Ok(joined)
}

fn main() -> Result<(), ParseError> {
    let path = "/rust-lang/cargo";

    let gh = build_github_url(path)?;

    assert_eq!(gh.as_str(), "https://github.com/rust-lang/cargo");
    println!("The joined URL is: {}", gh);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
