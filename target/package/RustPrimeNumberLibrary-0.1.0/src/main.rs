//use crate 
use RustPrimeNumberLibrary::IS_prime_OR_notPrime;
//importing library//
use std::io;



//main function 
fn main(){
    println!("--------------Please Enter a numer--------------");
    let mut num= String::new();

    io::stdin().read_line(& mut num).expect("failed to read a line");
    let int_num : u32 = num.trim().parse().expect("please enter correct number");
// Calling  Prime Number Checker //    
     IS_prime_OR_notPrime(int_num); //this is a funciton

}







