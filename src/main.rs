fn main() {
    println!("What is your name?");
    let input: String = read_string();
    // println!("Your name is: {}", input);
    calculate(&input);
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input
}

fn calculate(input: &str){
    let parse_input: Vec<&str> = input.split_whitespace().collect();
    
    for part in parse_input{
        let operator = 0;
        match part {
             "*" => println!("{}", part),
             _ => print!("")
        }
    }

}