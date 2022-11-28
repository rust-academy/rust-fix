mod no_derive;
mod derived_display;

fn main() {
    no_derive();
}

fn no_derive() {
    println!("Implemented Display method");
    let a = no_derive::MyEnum::VarA;
    let b = no_derive::MyEnum::VarB;
    println!("{}", a);
    println!("{}", b);

    let s = no_derive::MyStruct::new(10, 20);
    println!("{}", s);
    println!();
}


fn derive() {
    // cargo add parse-display-derive
    // https://crates.io/crates/parse-display-derive
    println!("Derived Display method");

    let s = derived_display::MyStruct::new(10, 20);
    println!("{}", s);

    let a = derived_display::MyEnum::VarA;
    let b = derived_display::MyEnum::VarB;
    println!("{}", a);
    println!("{}", b);
    println!();
}