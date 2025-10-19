fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // another_main();

    another_another_main();

    another_another_another_main();
}

fn another_main() {
    loop {
        println!("again!");
    }
}

fn another_another_main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn another_another_another_main() {
    let a = [1, 2, 3, 4, 5];
    let mut i = 0;

    while i < 5 {
        println!("The value at {i} is {}", a[i]);
        i += 1;
    }

    for el in a {
        println!("The value at is {el}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    
    println!("LIFTOFF!!!");
}