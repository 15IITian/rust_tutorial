// // // // [derive(Debug)]
// // // // use::hello_cargo
// // // use hello_cargo::{Summary, Tweet, NewsArticle};
// // // use core::fmt::Display;
// // // // use::hello_cargo::Tweet;
// // // // use::hello_cargo::Summary;
// // // fn main(){

// // //     let tweet= Tweet{
// // //         username: String::from("Divyansh"),
// // //         content:String::from("We will win the jokerrace contest"),
// // //         reply:false,
// // //         retweet:true

// // //     };


// // //     let article= NewsArticle{
// // //         headline:String::from("Penguins win the Stanley Cup Championship!"),
// // //         location:String::from("Pittsburgh, PA, USA"),
// // //         content:String::from("The Pittsburgh Penguins once again are the best \
// // //              hockey team in the NHL."),
// // //         author:String::from("Iceburgh")  ,   
// // //     };

// // //     // println!("new article is {}", article.summarize());

// // //     println!("the tweet is :  {}", tweet.summarize());

// // //     // pub fn notify(item: &(impl Summary + Display) ){
// // //     //     println!("Breaking news {}", item.summarize());
// // //     // }


// // //     // pub fn notify<T:Summary+ Display , U: Clone+ Display>(item1: &T, item2: &U){
// // //     //     println!("Breaking news {}", item1.summarize());
// // //     // }

// // // //     pub fn notify<T,U>(t:&T, u:&U) -> i32
// // // //     where 
// // // //     T:Display+ Clone,
// // // //     U:Clone+ Summary
// // // // {
    
// // // // } 


// // // fn returns_summarizable(switch: bool) -> impl Summary{

// // //     if switch{
// // //  Tweet{
// // //            username: String::from("divyansh"),
// // //            content:String::from("of course, as you probably already know, people"),
// // //            reply:false,
// // //            retweet:false
// // //     }
// // //     }
// // //     else{
// // //         NewsArticle {
// // //             headline: String::from(
// // //                 "Penguins win the Stanley Cup Championship!",
// // //             ),
// // //             location: String::from("Pittsburgh, PA, USA"),
// // //             author: String::from("Iceburgh"),
// // //             content: String::from(
// // //                 "The Pittsburgh Penguins once again are the best \
// // //                  hockey team in the NHL.",
// // //             ),
// // //         }
// // //     }
   
// // // }

// // // }


// // use std::fmt::Display;

// // // struct Pair<T>{
// // //     x:T,
// // //     y:T
// // // } 

// // // // impl<T> Pair<T>{
// // // //     fn new(x:T, y:T) -> self{
        
// // // //     }
// // // // }


// // // impl<T:Display + PartialOrd> Pair<T>{
// // //     fn cmP_display(&self){
// // //         if self.x >= self.y{
// // //             println!("the largest member is x = {}", self.x);
// // //         }
// // //         else{
// // //             println!("The largest member is y = {}",self.y) 
// // //         }
// // //     }
// // // }


// // struct ImportantExcerpt<'a> {
// // part: & 'a str,
// // }


// // fn main(){
   
// // //    let s1= String::from("abcd");
// // //    let result;
// // //    {
// // //        let s2= String::from("xyz");
// // //        result= longest(&s1, &s2);
// // //     }
// // //     println!("The longest string is {}", result);
    

// //     let novel= String::from("call . ne");
// //     let first_sentence= novel.split('.').next().expect("Could not find a .");
// //     let i= ImportantExcerpt{
// //         part:&novel,
// //     };

// // }


// // fn longest<'a>(x:& 'a str, y:&str)-> String{

// //      let result= String::from("really long");
// //      result 
// //     }


// impl <'a> Imp<'a>{
//     // fn level(&self) -> i32{
//     //     3
//     // }

//     fn ann(&self, announcement:&str) -> &str{
//         println!("Attention please: {}", announcement);
        //  self.part
//     }
// }
use std::fmt::Display;

fn main(){


    let s: & 'static str = "hello";
}


// // fn first_word<'a>(s:& 'a str) -> &'a str {}

// // fn longest<'a , 'b>(x: & 'a str , y:& 'b str) -> &str{}

fn longest_final< 'a, T>(
    x:&'a str , 
    y : &'a str, 
    ann: T
)-> &'a str
where 
T: Display, 
{
   println!("Announcement {}", ann);

   if x.len() > y.len() {
    x
   }
   else{
    y
   }
} 

