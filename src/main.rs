use std::io;

fn main() {
    let mut choice: String = String::new();
    println!("What operation do you want to apply to these numbers?\n 1. Addition\n 2. Subtraction\n 3. Multiplication\n 4. Division\n");
    io::stdin().read_line(&mut choice).expect("msg");

    
    

    let anotate: i32 = choice.trim().parse().expect("msg");

    if anotate == 1 {
        let mut a = String::new();
        println!("Input first number: ");
        io::stdin().read_line(&mut a).expect("msg");

        let mut b = String::new();
        println!("Input second number: ");
        io::stdin().read_line(&mut b).expect("msg");


        let ans = addition(a.trim().parse().expect("msg"), b.trim().parse().expect("msg"));

        println!("Your answer is: {}", ans);
    }else if anotate == 2 {
        let mut a = String::new();
        println!("Input first number: ");
        io::stdin().read_line(&mut a).expect("msg");

        let mut b = String::new();
        println!("Input second number: ");
        io::stdin().read_line(&mut b).expect("msg");


        let ans = subtraction(a.trim().parse().expect("msg"), b.trim().parse().expect("msg"));

        println!("Your answer is: {}", ans);

    }else if anotate == 3 {
        let mut a = String::new();
        println!("Input first number: ");
        io::stdin().read_line(&mut a).expect("msg");

        let mut b = String::new();
        println!("Input second number: ");
        io::stdin().read_line(&mut b).expect("msg");


        let ans = multiplication(a.trim().parse().expect("msg"), b.trim().parse().expect("msg"));

        println!("Your answer is: {}", ans);

    }else if anotate == 4 {
        let mut a = String::new();
        println!("Input first number: ");
        io::stdin().read_line(&mut a).expect("msg");

        let mut b = String::new();
        println!("Input second number: ");
        io::stdin().read_line(&mut b).expect("msg");


        let ans = division(a.trim().parse().expect("msg"), b.trim().parse().expect("msg"));

        println!("Your answer is: {}", ans);

    } else {
        println!("unknown error");
    }

    
}

fn addition(a: i32, b: i32) -> i32 {
    a+b
}

fn subtraction(a: i32, b: i32) -> i32 {
    a-b
}

fn multiplication(a: i32, b: i32) -> i32 {
    a*b
}

fn division(a: i32, b: i32) -> i32 {
    a/b
}



