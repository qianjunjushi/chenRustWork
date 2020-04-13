
use async_std::fs::{File,DirBuilder};
use async_std::prelude::*;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub async fn test()->Result<()>{
    writefile().await?;
    create_path().await?;
    Ok(())
}


pub async fn writefile()->Result<()>{

    let mut file = File::create("a.txt").await?;
    file.write_all(b"Hello, world!").await?;
    Ok(())
}


pub async fn create_path()->Result<()>{
    
    /*
    * 如果在win下面 没有. 而是以/开头 那么会创建
    *在程序执行的盘符的根目录下 G:/tmp/
    */
    let path = "./tmp/foo/bar/baz";
    DirBuilder::new()
        .recursive(true)
        .create(path).await?;
    Ok(())    
    
   // assert!(fs::metadata(path).unwrap().is_dir());
}   