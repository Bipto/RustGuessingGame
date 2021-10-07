use rand;
use rand::Rng;

fn valid_value_entered(guess: i16, value: i16) -> bool{
    if guess > value{
         println!("Your guess was too high, you guessed: {}", guess);
    }
    else if guess < value{
        println!("Your guess was too low, you guessed: {}", guess);
    }
    else{
        println!("You guessed correctly. The number was {}!", value);
        return true;
    }
    return false;
}

fn invalid_value_entered(){
    println!("An incorrect value has been entered");
}

pub fn run(){

    let mut guess_count: i16 = 3;

    let value: i16 = rand::thread_rng().gen_range(0..10);

    let mut correct_value_guessed = false;

    while guess_count > 0 {

        let mut user_input = String::new();

        println!("Please enter your guess between 0 and 10:");

        std::io::stdin().read_line(&mut user_input).expect("An error has occured!");
        let result = user_input.trim_end().parse();

        match result{
            Ok(n) => correct_value_guessed = valid_value_entered(n, value),
            Err(_e) => invalid_value_entered()
        }

        if correct_value_guessed {
            break;
        }
        else{
            guess_count -= 1;
            println!("You have {} guesses remaining!", guess_count);
        }
    }
}