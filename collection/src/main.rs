#[derive(Debug)]

 enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
 }


fn main() {
//     let mut v: Vec<i32> = Vec::new();

//     let v2= vec![12,3];
    
//    v.push(4);
//    v.push(4);

//    let third= &v[0];
//    println!("the third element is {}", third);

//    let third= v.get(3);
//    match third{

//     Some(no) => println!("the element is {}", no),
//     None=> println!("there is no element")
//    }


//    let mut v= vec![100,32,45];
//    for i in &mut v{
//     *i = 50;
//     assert_eq!(*i,50);
//     println!("{}", i);
//    }


let row = vec![SpreadsheetCell::Int(3), 
SpreadsheetCell::Text(String::from("blue")),
 SpreadsheetCell::Float(10.12)];

{
    let v= vec![1,2,3,4];

    let slice= &v[0..2];
    println!("slice=  {:?}", slice);
}

}
