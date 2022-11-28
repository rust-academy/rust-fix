use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{a}-{b}")]
pub struct MyStruct {
    a: u32, b: u32,
}

impl MyStruct {
    pub fn new(a: u32, b: u32) -> MyStruct { MyStruct { a, b } }
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style = "snake_case")]
pub enum MyEnum {
    VarA, VarB,
}