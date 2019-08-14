use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
#[my_custom_attribute = "blah_attrib"]
struct Blah {
    pub id: u32,
    pub blah: u16,
    pub desc: String,
}

pub mod blah_attrib {
    pub fn inside_func() {
        println!("{}", "Hello from inside blah_attrib module.");
    }
}

fn main() {
    Blah::hello_macro();
}
