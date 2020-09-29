pub mod hamming;
use hamming::{encode};
use num;
fn main() {
    let text:String="Hello World!".to_string();

    println!("{}",text);
    println!("{}",encode::to_binary_representation(&text));
    println!("{:?}",encode::encode_data(&text));
}
