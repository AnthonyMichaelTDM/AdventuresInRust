//Objective: play rock paper scissors

//crates
use std::fs;
use std::io;
use rand::Rng;

fn main() {
    //DATA
    let mut user_move;
    let mut computer_move;
    let mut user_wins: u8 = 0;
    let mut computer_wins: u8 = 0;
    let score_filename = "./score.txt";


    //ask user whether they want to continue from their previous score
    //does the file exist already
    if let Ok(_c) = fs::read(score_filename) {
        println!("Do you want to continue from your previous session?\nyes (Y/y) or no (N)\n");
        //get and act on user input
        match get_char_from_user_input() {
            'Y' | 'y' => { //read the previous score
                //separate user and computer scores from the returned tuple
                let temp_scores  = read_score_from_file(score_filename);
                user_wins = temp_scores.0;
                computer_wins = temp_scores.1;
            },
            _ => (),
        };
    }
    else {
        //create a score file
        fs::write(score_filename, " ").expect("error creating score file");
    }
    //introduction
    println!("\n\n\nvalid inputs:\n\tR or r for rock\n\tP or p for paper\n\tS or s for scissors\n\tQ or q to quit");
    println!("Player: {} wins\nComputer: {} wins\n", user_wins, computer_wins);

    //game loop
    loop {
        //get user play
        user_move = user_play();
        if user_move=='q' {break;}
        //get computer play
        computer_move = computer_play();
        //determine winner and update score
        let tmp = determine_winner(user_move, computer_move); //store winner in a temporary variable
        match tmp {
            0=>{println!("you played {}, computer played {}\tyou win!", user_move, computer_move); user_wins+=1;},
            1=>{println!("you played {}, computer played {}\tcomputer wins", user_move, computer_move); computer_wins+=1;},
            _=>println!("you both played {}, it was a tie", user_move),
        }
        //tell user the score
        println!("Player: {} wins\nComputer: {} wins\n", user_wins, computer_wins);
    }

    //ending message
    println!("thanks for playing, your final score was {} wins and {} losses", user_wins, computer_wins);
    
    //write scores
    write_scores_to_file(score_filename, user_wins, computer_wins);
}

//functions

/**
 * gets and returns a character from user input
 */
fn get_char_from_user_input() -> char {
    //data
    let mut input = String::new();

    //get user input
    match io::stdin().read_line(&mut input){ //.expect("failed to read user input") {
        Ok(_c) => {
            //parse for first character in user input
            input.remove(0)
        },
        Err(_) => {
            //call this method again
            get_char_from_user_input()
        },
    }
}

/**
 * reads the score form the file, where the user score is on the first line, and computer score is on the second line
 */
fn read_score_from_file(filename:&str) -> (u8,u8) {
    //data
    let mut scores: (u8,u8) = (0,0); //return tuple
    let mut line: usize = 0;
    let mut curr_num: u8 = 0;

    //read file
    for token in fs::read(filename).expect("error reading score file").iter() {
        //scan through first line of file for numeric characters, break from loop upon reaching the EOF or newline characters
        
        //is current character a newline, if so increment line
        if *token as char == '\n' {
            //save number
            if line == 0 {scores.0 = curr_num;}
            else if line == 1 {scores.1 = curr_num;}
            else {break;}
            //reset for next line
            line+=1;
            println!("{}", curr_num);
            curr_num = 0;
        }
        else if (*token as char).is_numeric() {
            curr_num *= 10;
            curr_num += *token-48;//-48 re-alligns the numbers to the actual number rather than the ascii-representation of said number
        }
    }
    println!("scores: {:?}", scores);
    scores
}

/**
 * generates and returns a char, either R,P,or S
 */
fn computer_play() -> char {
    let computer_move = rand::thread_rng().gen_range(0..=2); 
    match computer_move {
        0 => 'r',
        1 => 'p',
        _ => 's',
    }
}

/**
 * gets and returns user input in the form of a char
 */
fn user_play() -> char {
    //data 
    let mut u_move;

    //input loop
    println!("Enter your move: ");
    loop { 
        //print!("\nEnter your move: ");
        //process first char in user inpit
        u_move = match get_char_from_user_input() {
            'R'|'r'=>Some('r'),
            //'r'=>'r',
            'P'|'p'=>Some('p'),
            //'p'=>'p',
            'S'|'s'=>Some('s'),
            //'s'=>'s',
            'Q'|'q'=>Some('q'),
            //'q'=>'q',
            _ => {
                //return
                None
            },
        };
        

        //break from loop if input is valid
        if let Some(_c) = u_move {break;}
    }

    //return
    u_move.unwrap_or('q')
}

/**
 * determine winner based on the user and computer move
 * return 0 if user wins, 1 if computer wins and 2 if a tie
 */
fn determine_winner(user_move: char, computer_move: char) -> u8 {
    //data
    let winner: u8; //return variable

    //logic to determine winner
    //tie
    if user_move==computer_move { 
        winner = 2;
    }
    //user win
    else if (user_move == 'r' && computer_move == 's') || (user_move == 'p' && computer_move == 'r') || (user_move == 's' && computer_move == 'p') {
        winner = 0;
    }
    else {
        winner = 1;
    }

    //return
    winner
}

/**
 * write the passed scores to scorefilename
 */
fn write_scores_to_file(filename:&str, user_wins:u8, computer_wins:u8) {
    //data
    
    //write to file
    fs::write(filename,format!("{}\n{}\n",user_wins,computer_wins)).expect("error writing to file");
}

