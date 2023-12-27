use std::io;
fn main(){
let my_string= String::from("hello world");


let word= first_word(&my_string[0..6]);
let word= first_word(&my_string[..]);
let word= first_word(&my_string);

let string_literal= "hello_world";

let word= first_word(&string_literal[..]);
let word= first_word(&string_literal[0..8]);
let word= first_word(string_literal);




let a= [1,3,3,4,5,5];
let slice= &a[1..6];

assert_eq!(slice , &[2,3]);




}

fn first_word(s:&str) -> &str{
    let bytes = s.as_bytes();
    for(i , &item) in bytes.iter().enumerate(){
        if(item == b' '){
            return &s[0..i];
        }
    }
    &s[..]
}