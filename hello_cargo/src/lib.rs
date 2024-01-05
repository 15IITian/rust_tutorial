// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width < other.width && self.height > other.height
//     }
// }



// pub fn add(a:i32) -> i32{
//     a+3
// }

// pub fn greeting(name: &str) -> String{
//     format!("hello")
// }

// pub struct Guess{
//     value:i32,
// }

// impl Guess{
//     pub fn new(value:i32) -> Guess{
//         if value < 1  {
//             panic!("value must be greater than 1 . value=  {}", value);
//         }
//         else if( value > 100){
//             panic!("value must be smaller than 100 . value=  {}", value);
//         }

//         Guess{
//             value
//         }
//     }
// }




#[cfg(test)]
mod tests_part {
    use super::*;


    
    fn it_works() -> Result<(),String>{
        if 2+3 ==4 {
            Ok(())
        }
        else{
            println!("test passes sfhsagxgsgvdxvjfdv");
            Err(String::from("two plus two does not equal four"))
        }
    }

    fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

    #[test]
    #[ignore]
    fn testing1(){
        let value= prints_and_returns_10(7);
        assert_eq!(10, value);
    }

    #[test]
    fn testing2(){
        let value= prints_and_returns_10(4);
        assert_eq!(10, value);
    }


    #[test]
    fn testing3(){
        let value= prints_and_returns_10(4);
        assert_eq!(10, value);
    }
    // #[test]
    // #[should_panic(expected = "smaller")]
    // fn greater(){
    //     Guess::new(0);
    // }

    // #[test]
    // fn test_greeting(){
    //     let result = greeting("Divyansh");

    //     assert!(result.contains("Divyansh"),
    //             "Greeting did not contain name , value was {}", result);
    // }


    // #[test]
    // fn it_add_two(){
    //     assert_eq!(4, add(2));
    // }

    // #[test]
    // fn can_hold_larger(){
    //     let larger= Rectangle{
    //         height:7,
    //         width:8
    //     };
    //     let smaller= Rectangle{
    //         height:1,
    //         width:5
    //     };

    //     assert!(larger.can_hold(&smaller));
    // }

    //  #[test]
    // fn smaller_cannot(){
    //     let larger= Rectangle{
    //         height:7,
    //         width:8
    //     };
    //     let smaller= Rectangle{
    //         height:1,
    //         width:5
    //     };

    //     assert!(!smaller.can_hold(&larger));

    // }
}
