fn main() {
    println!("Hello, world!");

    another_function();

    another_another_function(5);

    another_another_another_function();

    let x = another_another_another_another_function();

    println!("Value returned is {x}");
}

fn another_function() {
    println!("Another function.");
}

fn another_another_function(value:u8) {
    println!("The value is {value}")
}

fn another_another_another_function() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn another_another_another_another_function() -> i32 {
    5
}