use tree::Conding;
mod tree;

fn main() {
    let buffer = "sssaaawwwjjjjwjfwufinlkwfuhb3wfuiv3v";

    let coding: Conding = buffer.parse().unwrap();
}
