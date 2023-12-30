use std::fs:: File;
// use std::io::ErrorKind;
use std::fs;
use std::io;



// use std::io::{self,Read};
fn main(){
    // panic!("crash and burn");


    // fn another();

    // let greeting_file_result= File::open("./hello.txt");
    
//     let greeting_file = match greeting_file_result{
//        Result::Ok(file) => file,
//         Err(error) => match error.kind(){
//                    ErrorKind::NotFound => match File::create("hello.txt"){
//                     Ok(f) => f,
//                     Err(e)=> panic!("problem creating the file ={}", e),
//                    },
//                    other_error => {
//                     panic!("Problem opening the file={}", other_error);
//                    }
//         }
// };

//  let greeting_file= File::open("hello.txt").unwrap_or_else(|error|{
//     if error.kind() == ErrorKind::NotFound {
//         File::create("hello.txt").unwrap_or_else(|error|{
//             panic!("Problem creating the file: {:?}",error);
//         })
//     }
//     else{
//         panic!("Problem openign the file");
//     }
//  });


// let greeting_file= File::open("hello.txt").unwrap();



// let greeting_file= File::open("hello.txt")
                                            //  .expect("hello.txt should be included in this project");


    let greeting_file= File::open("hello.txt")?;                                        


}


// fn read_username_from_file() -> Result< String, io::Error>{  // Result<t,e>

//     let username_file_result= File::open("hello.txt");

//     let mut username_file= match username_file_result{
//      Ok(file) =>file,
//       Err(e) => return Err(e),
//     };

//     let mut username= String::new();

//     match username_file.read_to_string(&mut username){
//         Ok(_)=> Ok(username),
//         Err(e) => Err(e),
//     }

// } 


// fn read_username_from_file() -> Result< String, io::Error>{  // Result<t,e>

//     let mut  username_file= File::open("hello.txt")?;

//     // let mut username_file= match username_file_result{
//     //  Ok(file) =>file,
//     //   Err(e) => return Err(e),
//     // };

//     let mut username= String::new();

//     username_file.read_to_string(&mut username)?;
//     // match username_file.read_to_string(&mut username){
//     //     Ok(_)=> Ok(username),
//     //     Err(e) => Err(e),
//     // }
//     Ok(username)
    

// } 






