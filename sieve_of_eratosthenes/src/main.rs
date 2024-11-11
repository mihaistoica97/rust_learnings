// MAKE THE SIEVE OF ERATOSTHENES, FIND PRIME NUMBERS BETWEEN 2 AND X WHERE X IS 
// A USER INPUT
fn get_primes(max_number: u128) {
    print!("2");
    for x in (3..max_number).step_by(2){
        let mut is_prime = true;
        for y in 2..=(x as f64).sqrt() as u128{            
            if x % y == 0 {               
                is_prime = false;
            } 
        }
        if is_prime == true{
            print!(" {}", x)
        }
    }
} 

fn main() {
    let mut user_number = String::new();
    let mut is_number = false;
    while is_number == false {        
        println!("GIVE ME A NUMBER");
        std::io::stdin().read_line(&mut user_number).unwrap();
        println!("Number is: {}", user_number);
        for x in user_number.chars(){
            if x == '\r' || x == '\n' {
                is_number = true;                
            }
            else if x.is_numeric() == false {
                user_number = String::new();
                break;
            }
        }
    }
    user_number.remove(user_number.len() - 1);
    user_number.remove(user_number.len() - 1);
    let max_number = user_number.parse::<u128>().unwrap();
    get_primes(max_number)
}
