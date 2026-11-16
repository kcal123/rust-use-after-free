fn main() {
    let value = Box::new(42);
    let reference = &*value;

    // The borrowed value is used before the owner is dropped.
    println!("{reference}");

    drop(value);
}
