// Animalトレイトの定義
trait Animal {
    // 名前を取得するメソッド
    fn get_name(&self) -> &String;

    // 性別を取得するメソッド
    fn get_gender(&self) -> u8;

    // 年齢を取得するメソッド
    fn get_age(&self) -> u8;

    // 鳴くメソッド
    fn make_sound(&self) {
        println!("{}が鳴きます。", self.get_name());
    }

    // 寝るメソッド
    fn sleep(&self) {
        println!("{}が寝ます。", self.get_name());
    }

    // 食べるメソッド
    fn eat(&self) {
        println!("{}が食事をします。", self.get_name());
    }
}

// Animalトレイトを実装する構造体
struct AnimalStruct {
    name: String,
    gender: u8,
    age: u8,
}

impl Animal for AnimalStruct {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_gender(&self) -> u8 {
        self.gender
    }

    fn get_age(&self) -> u8 {
        self.age
    }
}

fn main()
{
    // 犬と猫のインスタンスを生成
    let dog = AnimalStruct {
        name: String::from("ポチ"),
        gender: 0,
        age: 5,
    };

    let cat = AnimalStruct {
        name: String::from("タマ"),
        gender: 1,
        age: 3,
    };

    // 犬の動作をテスト
    println!("犬の名前: {}", dog.get_name());
    println!("性別: {}", if dog.get_gender() == 0 { "オス" } else { "メス" });
    println!("年齢: {}歳", dog.get_age());

    dog.make_sound();
    dog.sleep();
    dog.eat();

    // 猫の動作をテスト
    println!("猫の名前: {}", cat.get_name());
    println!("性別: {}", if cat.get_gender() == 0 { "オス" } else { "メス" });
    println!("年齢: {}歳", cat.get_age());

    cat.make_sound();
    cat.sleep();
    cat.eat();
}
