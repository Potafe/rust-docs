use unicode_segmentation::UnicodeSegmentation;

fn main() {
    //     // Strings are stored as a collection of UTF-8 encoded types.
    //     let mut s = String::new();
    //     let s2 = "init comment";
    //     let s3 = s2.to_string();
    //     let s4 = String::from("Hello");
    let s1 = String::from("Geel");
    let s2 = String::from("World");
    let s3 = s1 + &s2;

    for x in s3.bytes() {
        println!("{}", x);
    }

    println!("{}", s3);

    for x in s3.graphemes(true) {
        println!("{}", x);
    }
}
