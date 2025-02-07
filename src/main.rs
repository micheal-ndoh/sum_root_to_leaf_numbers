mod sum;

use std::io;

fn main() {
    println!("Enter the value of the root node:");
    let mut root_value = String::new();
    io::stdin().read_line(&mut root_value).expect("Failed to read line");
    let root_value: f64 = root_value.trim().parse().expect("Enter a number");

    let mut root = sum::TreeNode::new(1.0);

    loop {
        println!("Do you want to add a new node enter l for left and r for right (l/r/ok)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        match input {
            "l" => {
                println!("Enter the value of the left node:");
                let mut left_value = String::new();
                io::stdin().read_line(&mut left_value).expect("Failed to read line");
                let left_value: f64 = left_value.trim().parse().expect("Enter a number");

                let new_left = sum::TreeNode::new(left_value);
                root.left = Some(Box::new(new_left));
            },
            "r" => {
                println!("Enter the value of the right node:");
                let mut right_value = String::new();
                io::stdin().read_line(&mut right_value).expect("Failed to read line");
                let right_value: f64 = right_value.trim().parse().expect("Please type a number!");

                let new_right = sum::TreeNode::new(right_value);
                root.right = Some(Box::new(new_right));
            },
            "ok" => break,
            _ => println!("Invalid inpu Please try again."),
        }
    }

    let sum = sum::Stress::sum_numbers(Some(Box::new(root)));
   println!("The sum is : {}", sum)
}



