use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct TestStruct {
    a: String,
}

fn main() {
    TestStruct::hello_macro();
    println!("Hello, world!");
}
