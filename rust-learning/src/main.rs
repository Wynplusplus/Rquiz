use std::io;
use std::env;

struct question {
    question: String,
    correct_answer: String,
}

fn main() {
    let mut input = env::args().nth(1).unwrap_or("0.5".to_string());
    let input: f32 = match input.trim().parse(){
        Ok(intp) => intp,
        Err(err) => {
            println!("pleaseStartOverAndInputANumberAndIUseArchBTW");
            std::process::exit(1);
        }
        
    };

    let mut correct = 0.0;

    let questions = [
        question {
            question: "test".to_string(),
            correct_answer: "test".to_string(),
        },
        question {
            question: "test2".to_string(),
            correct_answer: "test2".to_string(),
        },
    ];

    for question in &questions {
        if input_and_return(question) {
            correct += 1.0;
        }
    }
    let x = questions.len() as f32;

    //let x: f32 = qam.parse();

    if correct >= x * input {
        println!("youHaveWon");
    } else {
        println!("youHaveLost");
    }
}

fn input_and_return(q: &question) -> bool {
    println!("{}", q.question);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failedToReadLine");

    if input.trim() == q.correct_answer {
        println!("answerCorrect");
        true
    } else {
        println!("answerIncorect");
        println!("CorrectanswerIS: {}", q.correct_answer);
        false
    }
}
