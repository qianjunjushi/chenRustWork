#[derive(PartialEq)]
pub enum BYTEORDER {
    BigEndian,
    LittleEndian,
    UNKNOW,
}

/*
 let x=byteorder::get_byte_order();
    if x==byteorder::BYTEORDER::BigEndian{
        println!("大端");
    }else  if x==byteorder::BYTEORDER::LittleEndian{
        println!("小端");
    }else{ println!("未知");}

*/
pub fn get_byte_order() -> BYTEORDER {
    let test:u32 = 0x12345678;
    if u32::from_be(test)== test {
        BYTEORDER::BigEndian
    } else if u32::from_le(test)== test {
        BYTEORDER::LittleEndian
    } else {
        BYTEORDER::UNKNOW

    }
}


