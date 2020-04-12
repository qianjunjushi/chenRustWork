use std::iter;
use std::ops;
use async_std::{task,prelude::*};

mod asyncchen;
use asyncchen::filechen;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
fn main()->Result<()>{
    task::block_on(filechen::writefile())
 
}
