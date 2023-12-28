use std::fmt::Result as rf;
use std::io::Result as ri;
use::rand::Rng;

fn f1(){
    let secret_number= rand::thread_rng().gen_range(1..=100);
    let n= HashMap::new();
}
fn function1() -> rf{

}

fn function2() -> ri<()> {

}



mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
        
    }
}


mod back_of_house{


    // pub enum Appetizer{
    //     soup,
    //     Salad,
    // }
    
    // pub struct Breakfast
    // {
    //     pub  toast: String,
    //     seasonal_fruit:String,
    // }
    
    // impl Breakfast{
    //     // #[derive(Debug)]
    //    pub fn summer(toast: &str) -> Breakfast{
            
    //         Breakfast{
    //             toast: String::from(toast),
                // seasonal_fruit:String::from("papita"),
    //         }
    //     }
    // }


    // fn fix_incorrect_order(){
    //     cook_order();
    //     super::deliver_order();
    // }

    // fn cook_order(){}
}


mod customer {
       use crate::front_of_house::hosting;
     pub fn eat_at_restaurant(){
        hosting::add_to_waitlist();
    }
   }

    fn deliver_order(){}


//     mod serving{
//         fn take_order(){}

//         fn serve_order(){}

//         fn take_payments(){}
//     }
// }


// restaurant::front_of_house::hosting::add_to_waitlist();

// restaurant::hosting::add_to_waitlist();