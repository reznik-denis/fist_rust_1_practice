// Remove a line in the code to make it compile
#[test]
pub fn task_1() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let mut _x = x;
    x += 3;


    let y = 4;
    // Shadowing
    let _y = "I can also be bound to text!";

    println!("Success!");
}