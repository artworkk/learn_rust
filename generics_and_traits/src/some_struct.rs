use std::fmt::Display;

pub struct SomeStruct<T>
where
    T: Copy + std::ops::Add<Output = T>,
{
    x: T,
    y: T,
}

impl<T> SomeStruct<T>
where
    T: Copy + std::ops::Add<Output = T>,
{
    pub fn new(a: T, b: T) -> Self {
        Self { x: a, y: b }
    }
    pub fn sum(&self) -> T {
        self.x + self.y
    }
}

// Or you can also omit where clause
impl<T: Display + Copy + std::ops::Add<Output = T>> Display for SomeStruct<T> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "x: {}, y: {}, sum: {}", self.x, self.y, self.sum())
    }
}
