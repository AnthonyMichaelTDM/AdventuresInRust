fn main() {
    //vectors are basically array lists
    {
        let _v: Vec<i32> = Vec::new(); //create empty vec
    } // like other structs, vectors are dropped when they go out of scope
    let mut v = vec![1,2,3,4,5]; //use the vec! macro
    println!("{:?}", v);

    //add data to a vector with .push
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);
    v.push(10);
    println!("{:?}",v);

    //remove data with .pop
    v.pop();
    println!("{:?}",v);

    //reading from a vector
    let third: &i32 = &v[2]; //array notation
    println!("The third element is {}", third);

    match v.get(2) { //.get(index)
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    //iterating over a vector
    for i in &v {
        println!("{}", i);
    } //read only

    for i in &mut v {
        *i = *i * 2; //have to deferenec the i
        println!("{}",i);
    }//read and write

    //using an enum to store multiple types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);


}

