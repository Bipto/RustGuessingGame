mod guessing_game;

fn main() {
    let mut play = true;

    while play {
        guessing_game::run();
        println!("Would you like to play again (y/n)?");

        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).expect("An error has occured!");

        user_input = user_input.trim_end().to_lowercase();

        if user_input == "n" || user_input == "N" || user_input == "no"{
            play = false;
        }
    }
}