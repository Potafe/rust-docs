fn main() {
    let _a = [1, 2, 3];
    
    let mut v:Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    
    match v.get(20) {
        Some(_third) => println!("The third element is {}", _third),
        None => println!("Index Out of Bounds")
    }

    for i in &v {
        print!("\n{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec! [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    let mut i = 1;

    match &row[i] {
        SpreadsheetCell::Int(i) => println!("Welcome Integer {}", i),
        _ => println!("Not an integer")
    }
}

