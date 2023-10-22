fn main() {
    println!("Please Enter Your input Equation: ");
    let input: String = read_string();
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
    let mut operator:[i32;3]=[0,0,0];
    let mut counter = 0;

    for part in parse_input{
        match part {
             "*" => operator[1] = 1,
             "+" => operator[1] = 2,
             "/" => operator[1] = 3,
             "-" => operator[1] = 4,
             _ => if counter==0 {operator[0]=part.parse().unwrap();}else if counter==2 {operator[2]=part.parse().unwrap();}
        }
        counter +=1;
    }

    match operator[1] {
        1 => println!("Answer is: {:?}", operator[0] * operator[2]),
        2 => println!("Answer is: {:?}", operator[0] + operator[2]),
        3 => println!("Answer is: {:?}", f64::from(operator[0]) / f64::from(operator[2])),
        4 => println!("Answer is: {:?}", operator[0] - operator[2]),
        _ => println!("Wrong input bye!!!!")
        
    } 
}