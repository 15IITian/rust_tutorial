// enum IpAddrKind{
//     v4(u8,u8,u8,u8),
//     v6(String),
// }

// struct IpAddr{
//     kind:IpAddrKind,
//     address:String,
// }


// struct Ipv4Addr{

// }

// struct Ipv6Addr{

// }


// enum IpAddr{
//     v4(Ipv4Addr),
//     v6(Ipv6Addr)
// }

// enum Message{
//     Quit,
//     Move{ x:i32, y:i32},
//     Write(String),
//     ChangeColor(i32,i32,i32),
// }

// impl Message{
//   fn call(&self){

//   }  

  
// }

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska
}
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main(){
    // let home= IpAddrKind::v4(127,0,9,8);
    // let loopback= IpAddrKind::v6(String::from("::8"));

    // let m= Message::Write(String::from("hello"));
    // m.call();

    //option<t><

    // let some_number= Some(5);
    // let some_char= Some('c');

    // let absent  = None;

    // let x:i8= 5;
    // let y:Option<i8> = Some(5);
    // let y= Some(5);
    // let sum=x + y;
//     let no = value_in_cents(Coin::Quarter(UsState::Alaska));
   

//    let five = Some(5);
//    let six= plus_one(five);
//    println!("six = {:?}", six);

//    let none_= plus_one(None);
//    pri,ntln!("nothing = {:?}",none_ );

   

//    let dice_roll= 9;
//    match dice_roll{
//     3 => add_fancy_hat(),
//     7 => remove_fancy_hat(),
//     // trial => move_player(trial),
//     // _ => reroll(),
//     _ => (),
//    }

let config_max= Some(3u8);
match config_max {
    Some(max)=> println!("The maximum is configured to be {}",max),
    None=> (),
}


if let Some(max) = config_max{
    println!("The maximum is configured to be {}", max);
}

let coin =Coin::Penny;
let mut non_quarter= 0;
// match coin {
//     Coin:: Quarter(state) => println!("State quarter form {:?}",state),
//     _ => non_quarter += 1,
// }


if let Coin::Quarter(state) = coin{
 println!("State quarter form {:?}",state);
}
else{
    non_quarter +=1;
}


}

// fn value_in_cents(coin:Coin) -> u8{
//     match coin{
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         },
//         Coin:: Nickel => 5,
//         Coin::Quarter(state)=> {
//             println!("the state is {:?}", state);
//             25
//         },
//         Coin::Dime => 25,
//     }
// }

// fn plus_one(x:Option<i32>) -> Option<i32>{
//     match x{
//         Some(i) => Some(i+1),
//         None => None,
//     }
// }

// fn add_fancy_hat(){}
// fn remove_fancy_hat(){}
// fn move_player(num_spaces:u8){}
// fn reroll(){}
