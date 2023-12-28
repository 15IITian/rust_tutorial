mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
        
    }
}


mod back_of_house{


    pub enum Appetizer{
        soup,
        Salad,
    }
    
    pub struct Breakfast
    {
        pub  toast: String,
        seasonal_fruit:String,
    }
    
    impl Breakfast{
        // #[derive(Debug)]
       pub fn summer(toast: &str) -> Breakfast{
            
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit:String::from("papita"),
            }
        }
    }


    // fn fix_incorrect_order(){
    //     cook_order();
    //     super::deliver_order();
    // }

    // fn cook_order(){}
}

    pub fn eat_at_restaurant(){

    //     crate::front_of_house::hosting::add_to_waitlist();
       
    //    front_of_house::hosting::add_to_waitlist();

    let meal= back_of_house::Breakfast::summer("Rye");
      println!("my meal = {:?}", meal.toast);

      meal.toast = String::from("Wheat");
      println!("I'd like {} toast please",meal.toast);



      meal.seasonal_fruit= String::from("Apple")
   
   
      let order1= back_of_house::Appetizer::soup;
      let order2= back_of_house::Appetizer::Salad;
    }

    fn deliver_order(){}


//     mod serving{
//         fn take_order(){}

//         fn serve_order(){}

//         fn take_payments(){}
//     }
// }