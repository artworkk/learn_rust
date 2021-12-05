pub fn smallest<T: std::cmp::PartialOrd + Copy>(values: &[T]) -> T {
    let mut smallest = values[0];
    for &value in values {
        if value < smallest {
            smallest = value;
        }
    }
    return smallest;
}
