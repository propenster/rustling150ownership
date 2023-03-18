fn main() {
    
    // {
    //     let a = 10;

    // }
    // println!("{}", a);
    let s = String::from("Jayesh");

    main1(&s);
    //REFERENCES and BORROWING

    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10{
            break
            counter * 2;
        }
    };
    println!("The result is {result}");




}

fn main1(x:&String){
    println!("{} ", &x);
}
