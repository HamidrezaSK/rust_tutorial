struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

fn main() {
    println!("Hello, world!");
}
