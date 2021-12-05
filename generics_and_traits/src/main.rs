mod smallest;
mod some_struct;
use smallest::smallest;

struct MyStruct {}

struct HisStruct {
    size: usize,
}

trait MyTrait {
    // Default implementation returns 10
    fn lol(&self) -> usize {
        10
    }
}

impl MyTrait for MyStruct {}
impl MyTrait for HisStruct {
    fn lol(&self) -> usize {
        self.size
    }
}

fn my_func(k: &impl MyTrait) {
    println!("lol: {}", k.lol())
}

fn main() {
    let x = MyStruct {};
    let y = HisStruct { size: 69 };

    my_func(&x);
    my_func(&y);

    let char_arr: [char; 3] = ['a', 'b', 'c'];
    let f64_slice = [69., 70., 80.];
    println!("{}", smallest(&char_arr));
    println!("{}", smallest(&f64_slice));

    let some_var = some_struct::SomeStruct::new(0.1, 0.2);
    println!("{}", some_var);
}
