// use::hello_cargo::kinds::PrimaryColor;
// use::hello_cargo::utils::mix;

use hello_cargo_divyansh::mix;
use hello_cargo_divyansh::PrimaryColor;
fn main(){
    let red= PrimaryColor::Red;
    let yellow= PrimaryColor::Yellow;
    mix(red, yellow);
}