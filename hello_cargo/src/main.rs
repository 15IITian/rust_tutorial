use std::collections::HashMap;

fn main(){

//     let mut scores:HashMap<String,u8>= HashMap::new();
//     scores.insert(String::from("Blue"), 10);
// scores.insert(String::from("Yellow"), 50);

// let team_name = String::from("Blue");
// let score= scores.get(&team_name).copied().unwrap_or(0);

// for (key,value) in &scores{
//     println!("{key}: {value}");
// }

// let field_name= String::from("Favorite color");
// let field_value = String::from("Blue");

// let mut map= HashMap::new();
// map.insert(&field_name,&field_value);

// println!("feild_name-> {}",field_name );

// let mut scores= HashMap::new();

// scores.insert(String::from("Blue"), 10);
// // scores.insert(String::from("Blue"), 20);


// // let score= scores.get("Blue").copied();
// // println!("{:#?}", score);

// // match score{
// //     Some(T) => (),
// //     None => {
// //         scores.insert(String::from("blue"),20);
// //     }
// // }
// println!("{:#?}", scores);


// let a= scores.entry(String::from("Yellow")).or_insert(70);
// println!("a={:?}",a );

// println!("{:#?}", scores);


let text= "hello world wonderful world";
let mut map= HashMap::new();
for word in text.split_whitespace(){

    let count= map.entry(word).or_insert(0);

    println!("word = {}", word);
    *count+=1;
}

println!("map = {:#?}", map);

}
