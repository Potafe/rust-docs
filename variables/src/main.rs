// fn main() {
//     let x = 5;
//     println!("The value of x is {x}");

//     x = 6;
//     println!("The value of x is {x}");

//     // Gets me Error -> error[E0384]: cannot assign twice to immutable variable `x`
// }

// fn main() {
//     let mut x = 5;
//     println!("The value of x is {x}");

//     x = 6;
//     println!("The value of x is {x}");

//     // No error
// }

// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");

//     // Shows shadowing of a var.
// }


fn main() {
    // let tup: (i32, f64, u8) = (500, 5.5, 1); // -> this is a tuple

    // let (_x, _y, _z) = tup;

    // println!("The val of _y is {_y}")

    let a = [1, 2, 3, 4, 5]; // -> array

    let element = a[9];
    
    println!("The val of el at index 9 is {element}");
}