// use std::thread;
// use std::time::Duration;
// #[derive(Debug, Copy, Clone)]
// enum ShirtColor{
//     Red,
//     Blue
// }



// struct Inventory{
//     shirts: Vec<ShirtColor>
// }

// impl Inventory{

//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor{
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }

//     fn most_stocked(&self) -> ShirtColor{
//         let mut num_red=0;
//         let mut num_blue=0;

//         for color in &self.shirts{
  
//            match color{
//             ShirtColor:: Red => num_red+=1,
//             ShirtColor:: Blue => num_blue+=1
//            }
//         }

//         if num_red > num_blue {
//             ShirtColor::Red
//         }else{
//             ShirtColor::Blue
//         }
//     }


// }

// fn main(){

//     // let store = Inventory{
//     //     shirts: vec![ShirtColor::Blue,ShirtColor::Red, ShirtColor::Blue]
//     // };

//     // let user_pref1 = Some(ShirtColor::Red);

//     // let giveaway1 = store.giveaway(user_pref1);

//     // println!("the user1 with prefernece {:?} gets {:?}", user_pref1, giveaway1);

//     // let user_pref2= None;
//     // let giveaway2= store.giveaway(user_pref2);
    

//     // println!("the user2 with prefernece {:?} gets {:?}", user_pref2, giveaway2);


//     // let expensive_closure= |num:u32| -> u32 {
//     //     println!("calculating slowly ...");
//     //     thread::sleep(Duration::from_secs(2));
//     //     num 
//     // };

//     // let example= |x| x;
//     // let s= &example(String::from("hello"));

//     // let n= example(5);


//     // let mut list = vec![1,3,4];
//     // println!("before deining closure :{:?}", list);

//     // let mut only_borrows= || list.push(7);

//     // only_borrows();
//     // println!("before  calling closure: {:?}", list);

//     // println!("{:?}", list);

//     // println!("after calling closure: {:?}", list);

// let list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);

//     thread::spawn(move || println!("from thread : {:?}", list))
//                    .join()
//                    .unwrap();


// }



// Iterators ->
fn main(){
    // let mut v1= vec![1,3,4];
    // let v1_iter= v1.iter();
    // let v1_iter2= v1.iter();
    // for val in v1_iter{
    //     println!("Got : {}", val);
    // }

    // println!("iterator = {:?}", v1_iter);

//    let mut v2:Vec<_>= v1.into_iter().map(|x| x*5).collect();

//     let v2_iter2= v2.iter();
//     let v1_iter2= v1.iter();


//     for val in v2_iter2{
//         println!("G : {}", val);
//     }
// for val in v1_iter2{
//         println!("Back : {}", val);
//     }

    // for val in v1.iter().map(|x| x*5){
    //     println!("Back : {}", val);
    // }
let buffer:&mut [i32];
let coefficients: [i64;12];
let qlp_shift: i16;





}


// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }



// fn main(){

//     let arr= [1,3,4,5];

// let a = arr[0];

// let b= &arr[0];



//     let arr3= ["hello", "world"];

//     let s= "hello";
//     let x= s;
//     println!("s {}",s );



//     let mut s= String::from("hello");
//     let slice = &mut s[0..2];
//     let a= slice;
//     // println!("slice {}",slice );

// let arr2= [String::from("hello"),String::from("world")];
// // let a= arr2[0];

// let b=  &arr2[0];



//     // let c= arr3[0];

//     // let d=&arr3[0];
//     // let arr3_ref= &arr3;
//     // let array= &arr2;

//     // let a= arr3_ref[0];


// }

