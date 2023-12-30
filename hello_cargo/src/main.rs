use std::cmp::PartialOrd;

struct Point<X1, Y1> {
    x:X1,
    y:Y1,
}

impl<X1,Y1> Point<X1,Y1>{
    fn mixup<X2,Y2>(&self, other:Point<X2,Y2>)-> Point<X1,Y2>{
       Point { x:self.x , y:other.y }
    }
}


// impl<U> Point<f32> {
//     fn x(&self) -> &U{
//         &self.x
//     }
// }

// impl Point<f64> {
//     fn distnace_form_origin(&self) -> f64{
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

 
// enum Option<T>{
//     Some(T),
//     None
// }

// enum Result<T,E> {
//     Ok(T),
//     Err(E),
// }

enum Option{
    Some(f64),
    None
}


fn main(){


//     let number_list= vec![2,3,45,6];
//    let result= largest(&number_list);

//    let arr= [1,2,3,4];
// let result= largest(&arr);

//     println!("the largest no is = {}", result);

//     let number_list = vec!['e','r','t','u'];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

// let integer= Point{x:5, y:7};
// let float= Point{x:"edffdsfd",y:"xcdvn" };

// let mix= Point{x:6.0, y:7.0};
// let result= mix.distnace_form_origin();

let p1= Point{x:5, y:12.7};
let p2= Point{x:"Hello", y:'c'};

let p3= p1.mixup(p2);
 println!("p3.x = {}, p3.y = {}", p3.x, p3.y);




 let integer= Some(5);
 let float= Some(6.0);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T{
    let mut  large= list.get(0).unwrap();

    for number in list{
        if large < number{
            large = number;
        }
    };
    large
}