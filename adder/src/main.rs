// use add_one;

// use std::ops::Deref;

// use crate::List::{Cons, Nil};
// enum List<T>{
//     Cons(T,Box<List<T>>),
//     Nil,
// }


// enum Message{
//     Quit, Move
// }

// #[derive(Debug)]
// struct MyBox<T>(T);

// impl<T> MyBox <T>{
//     fn new(x:T) ->MyBox<T>{
//         MyBox(x)
//     }
// }
// impl<T> Deref for MyBox<T>{
      
//       type Target= T;
//       fn deref(&self) -> &Self::Target{
//         &self.0
//       }
// }

// fn main(){
//     // let num=10;
//     // println!("Hello, world! {num} plus one is {}", add_one::add_one_fn(num));


//     let b= Box::new(5);
//     println!("b={}", b);

//     let list= Cons(1,Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));


//     let x= 5;
//     // let y = &mut x;
//     // let z= *y;
//     // // assert_eq!(*y+1, x);
//     // assert_eq!(x, 5);

//     // let y= Box::new(x);

//     // assert_eq!(5, x);
//     // assert_eq!(5, *y);
//     // println!("{}", x);

//     let y= MyBox::new(x);
//     println!("{:?}", y);
// assert_eq!(5, *y);
// // let z= y.deref();


// let m= MyBox::new(String::from("RUsr"));


// let a= m.deref().deref();
// // let a= *(&m);
// let b= &m.deref().deref();
// // hello(m);
// let c= &(*m)[..];


// // hello(m);
// }

// fn hello(name:String){
//     println!("hello, {name}");
// }



// // (1, (2,(3,Nil)))


#[derive(Debug)]
struct custom{
    data:String,
}

impl Drop for custom{
    fn drop(&mut self){
        println!("Dropping custom {:?}",self);
    }
}
fn main(){

   let c= custom{
        data:String::from("one stuff")
    };


    let d = custom {
        data: String::from("other stuff"),
    };
    // c.drop();
    drop(c);
    
       println!("CustomSmartPointers dropped before {:?}.", d);
   
}