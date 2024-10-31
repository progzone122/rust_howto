use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("I'm async!");
    Ok(())
}

#[test]
fn test_async_main() -> Result<()> {
    main()?;
    Ok(())
}
