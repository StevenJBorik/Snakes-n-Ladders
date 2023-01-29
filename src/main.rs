extern crate text_io; 

use snakenladder::Snl; 
use std::process::exit;

// main function

fn main() {
    println!("{}", "~~~Time to play Sneks n Ladrs~~~"); 
    // Display board
    let mut snl = Snl::new();
    snl.gamestart(); 
    exit(0); 
}