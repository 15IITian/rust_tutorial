#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle{
    fn area(&self) -> u32{
self.width * self.height
    }

    


    fn can_hold(&self , other:&Rectangle) -> bool{
           if self.width > other.width   && self.height > other.height
           {
            true
           }
           else{
            false 
           }
    }


    fn square(size:u32) -> Self{
        Self { width: (size), height: (size) }
    }
}

impl Rectangle{
    fn width(&self) -> bool{
        self.width >0
    }
}
fn main(){
    let scale=4;
    let rect1= Rectangle{
        width:dbg!(30*scale),
        height:50

    };
    // if rect1.width(){
    //     println!("the rectangle has a nonzero width {}", rect1.width);
    // }
    // println!("area = {} sqaure metres", rect1.area());

    // // println!("rect1= {:#?}", rect1)d
    // dbg!(&rect1);

    let rect2= Rectangle{
        width:dbg!(10*scale),
        height:40
    };

        println!("Rect1 can hold rect2 -> {}", rect1.can_hold(&rect2));
        println!("Rec2 can hold rect1 -> {}", rect2.can_hold(&rect1));


        let sq= Rectangle::square(5);

}


