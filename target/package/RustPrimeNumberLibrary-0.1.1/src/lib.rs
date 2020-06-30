// define path of module 
pub use crate::Prime_Number_Checker::IS_prime_OR_notPrime;


// Prime Number Checker//
pub mod Prime_Number_Checker{
    pub fn IS_prime_OR_notPrime(num : u32){
    
           //intilize the loop for int value 2 beacause the cheker prime program and programm flow is every no check for required user input no is prime or not prime
  for j in 2..{

    if(j< num){ // check j is less user input value

       let result = num % j;  // reminder 
       if(result==0){      //checks reminder is equal to zero
           println!("{} is not prime ",num);
           
           break;
       }
   }
   else{ 
       println!("{} is prime", num);
       break;

      
   }
  
  

 }
 
    
   

}


}
