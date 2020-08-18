extern crate stack;
use stack::Stack;

fn main() {
    // Initializing the stack data structure
    let mut orders = Stack::new();

    // Adding some orders
    orders.push("Curry rice");
    orders.push("Steak and lobster tail");
    orders.push("McDonalds Happy Meal");
    orders.push("Omakase special");

    // Check to see what is at the top of the stack
    if orders.peek() != Some(&"McDonalds Happy Meal") {
        println!("We're not eating cheap tonight.")
    }

    // Remove the top order
    orders.pop();

    // Check to see if the stack is empty
    println!("Is the stack empty?    {}", orders.is_empty());

    // Removing all of the orders in the stack
    while !&orders.is_empty() {
        println!("{:?}", orders.pop());
    }

    if orders.is_empty() {
        println!("No more orders.");
    }
}
