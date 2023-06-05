#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn create_optional_vec<'a, T: 'static, U>(value: U, count: usize) -> Option<Vec<T>>
where
    T: From<U> + Clone,
{
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<Person>() {
        let vec: Vec<T> = vec![value.into(); count];
        Some(vec)
    } else {
        None
    }
}

fn main() {
    let vec1: Option<Vec<i32>> = create_optional_vec(10, 5);
    println!("{:?}", vec1);

    let person = Person {
        name: "Alice".to_string(),
        age: 25,
    };
    let vec2: Option<Vec<Person>> = create_optional_vec(person.clone(), 3);
    println!("{:?}", vec2);
}
