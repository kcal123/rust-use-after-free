fn main() {
    let value = Box::new(42);
    let reference = &*value;

    // Rust rejects destroying the owner while the reference is still needed.
    drop(value);

    println!("{reference}");
}
