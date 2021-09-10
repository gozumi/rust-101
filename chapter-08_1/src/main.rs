enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    //let does_not_exist = &v2[100];
    let does_not_exist = v2.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    //v.push(6);
    println!("The first element is {}", first);

    let v = vec![100, 32, 57];
    for mut i in v {
        println!("before {}", i);
        i += 50;
        println!("after {}", i);
    }

    let mut v = vec![23, 53, 244, 77];
    for i in &mut v {
        println!("before {}", i);
        *i += 50;
        println!("after {}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for item in row {
        match item {
            SpreadsheetCell::Int(num) => println!("item is the integer {}", num),
            SpreadsheetCell::Text(text) => println!("item is the string {}", text),
            SpreadsheetCell::Float(num) => println!("item is the integer {}", num),
        }
    }
}
