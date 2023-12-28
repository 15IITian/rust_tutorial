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




fn main(){
    // let home= IpAddrKind::v4(127,0,9,8);
    // let loopback= IpAddrKind::v6(String::from("::8"));

    // let m= Message::Write(String::from("hello"));
    // m.call();

    //option<t><

    let some_number= Some(5);
    let some_char= Some('c');

    let absent: Option<i32>  = None;

    let x:i8= 5;
    // let y:Option<i8> = Some(5);
    let y= Some(5);
    let sum=x + y;
}



