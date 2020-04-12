
use async_std::fs::File;
use async_std::prelude::*;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub async fn writefile()->Result<()>{

    let mut file = File::create("a.txt").await?;
    file.write_all(b"Hello, world!").await?;
    Ok(())
}