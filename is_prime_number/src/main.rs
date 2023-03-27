fn main() {
    let inputed_number: i32 = get_number();
    check_number(inputed_number);
}


fn get_number() -> i32 {
    return 4;
}


fn check_number(to_check: i32) {
    // create en array named checking_history
    for i in 2..to_check{
        if to_check%i == 0{
            // add false in the checking_history
        } else {
            // add true to the checking_history
        }
    }
    is_prime(checking_history, to_check)
}


fn is_prime(history: i32, checked_number: i32){
    for false in history{
        println!("\n\t - {checked_number} is a prime number.");
        break;
    }
    for true in history{
        println!("\n\t - {checked_number} is not a prime number.");
        break;
    }
}

