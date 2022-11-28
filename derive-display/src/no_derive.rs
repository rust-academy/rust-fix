use std::fmt;

pub enum MyEnum {
    VarA,
    VarB,
}

impl MyEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            MyEnum::VarA => "VarA",
            MyEnum::VarB => "VarB",
        }
    }
}

impl fmt::Display for MyEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub struct MyStruct {
    a: u32,
    b: u32,
}

impl MyStruct {
    pub fn new(a: u32, b: u32) -> MyStruct { MyStruct { a, b } }
}

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.a, self.b)
    }
}