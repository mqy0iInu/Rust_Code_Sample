struct MyVec<T> {
    data: Vec<T>,
}

impl<T> MyVec<T> {
    pub fn new(data: Vec<T>) -> Self {
        MyVec { data }
    }

    pub fn find_value<P>(&self, target: P) -> Option<&T>
    where P: Fn(&T) -> bool,
    {
        for item in &self.data {
            if target(item) {
                return Some(item);
            }
        }
        None
    }
}

fn main()
{
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let my_struct = MyVec::new(numbers);
    let result = my_struct.find_value(|&x| x == 3);
    match result {
        Some(value) => println!("Found: {}", value),
        None => println!("Value not found"),
    }

    let words: Vec<&str> = vec!["apple", "banana", "cherry"];
    let my_struct = MyVec::new(words);
    let result = my_struct.find_value(|&x| x == "banana");
    match result {
        Some(value) => println!("Found: {}", value),
        None => println!("Value not found"),
    }
}