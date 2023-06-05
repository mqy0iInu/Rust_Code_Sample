fn main()
{
    // Loop example
    let mut cnt: u8 = 1;
    loop {
        println!("Loop Cnt: {}", cnt);
        cnt += 1;
        if cnt > 5 {
            break;
        }
    }

    // For loop example
    let numbers: [u8; 5] = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("Number: {}", number);
    }

    // While loop example
    let mut cntdown: u8 = 5;
    while cntdown > 0 {
        println!("Cntdown: {}", cntdown);
        cntdown -= 1;
    }
}