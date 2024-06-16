use hello_macro_internal::HelloMacro;

#[derive(HelloMacro)]
struct TestStruct {
    a: String,
}

fn main() {
    TestStruct::hello_macro();
    println!("Hello, world!");
}
