#[allow(dead_code)]
#[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         match self {
//             Message::Quit => println!("Quit message received"),
//             Message::Move { x, y } => println!("Move to position ({}, {})", x, y),
//             Message::Write(content) => println!("Write message: {}", content),
//             Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
//         }
//     }
// }

// enum Option<T> {
//     None,
//     Some(T),
// }

enum IndiaState {
    Delhi,
    UP,
}

enum Coin {
    OneRupee(IndiaState),
}

fn main() {
    let coin = Coin::OneRupee(IndiaState::Delhi);
    print_coin(coin);
}

fn print_coin(coin: Coin) -> u8 {
    match coin {
        Coin::OneRupee(state) => {
            println!("State Coin from {state:?}");
            25
        },
    }
}

// fn main() {
//     // let m = Message::ChangeColor(1, 2, 3);
//     // m.call();
//     println!("Hello, world!");
// }
