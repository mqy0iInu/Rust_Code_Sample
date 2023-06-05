fn check_number(num: u32)
{
    if num % 2 == 0 {
        println!("Even Number!");
    } else {
        println!("Odd Number!");
    }
}

fn check_animal(animal: &str)
{
    match animal {
        "Dog" => println!("Dog"),
        "Cat" => println!("Cat"),
        "Bird" => println!("Bird!"),
        _ => println!("Unknown Animal"),
    }
}

fn main()
{
    check_number(2);
    check_number(3);

    check_animal("Dog");
    check_animal("Cat");
    check_animal("Cat");
}
