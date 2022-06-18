fn main() {
    let x = 5;
    println!("Hello, world!: {} ", x);
    { //Block for local variable
        let x = x + 1;

        println!("Hello, world!: {} ", x);
        
    }

    let x = x + 6;

    println!("Hello, world!: {} ", x);


    let spaces = "   ";
    println!("SPACES: {} ", spaces);
    let spaces = spaces.len();

    println!("SPACES: {} ", spaces);
}
