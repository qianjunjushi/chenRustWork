
use std::fs::{self, DirBuilder};
use std::io::Result;



pub fn create_path()->Result<()>{
    
    /*
    * 如果在win下面 没有. 而是以/开头 那么会创建
    *在程序执行的盘符的根目录下 G:/tmp/
    */
    let path = "./tmp/foo/bar/baz";
    DirBuilder::new()
        .recursive(true)
        .create(path)?;
    Ok(())    
    
   // assert!(fs::metadata(path).unwrap().is_dir());
}    