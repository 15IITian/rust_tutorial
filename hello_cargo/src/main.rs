fn main(){
    let mut s= String::new();

    let data= "hello";
    s= data.to_string();
// println!("{}", data);
    s= "initial".to_string();

    s= String::from("initial");

    let hello = String::from("नमस्ते");

    let slice= &hello[0..3];
    s.push_str(slice);
    
    
    s.push('i');
    println!("{}", s);
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    // let s= s1+"-" +&s2 +"-" + &s3;
    let s= format!("{s1}-{s2}-{s3}");
    
    println!("{}", s);


    // let h= s[0];

    let hello = "Здравствуйте";
    let answer= &hello[0..4];


    for c in "Зд".chars(){
        println!("{c}");
    }

     for c in "Зд".bytes(){
        println!("{c}");
    }
}