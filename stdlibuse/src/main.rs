

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;



/*
use async_std::{task,prelude::*};

mod asyncchen;
use asyncchen::filechen;
fn main()->Result<()>{
    task::block_on(filechen::test())
 
}
*/

mod stdchen;
use stdchen::filechen;
use stdchen::byteorder;

fn main()->Result<()>{
    filechen::create_path()?;
    let x=byteorder::get_byte_order();
    if x==byteorder::BYTEORDER::BigEndian{
        println!("大端");
    }else  if x==byteorder::BYTEORDER::LittleEndian{
        println!("小端");
    }else{ println!("未知");}

    Ok(())


}




