fn double_vec_with_reverse(vec: &Vec<u8>) -> Vec<u8> {
    let mut doubled_vec = vec.clone(); // 引数のVecのクローンを作成

    doubled_vec.reserve(vec.len()); // 新しいVecの容量を確保

    // 元のVecを逆順に並べ替えて新しいVecに追加
    for &item in vec.iter().rev() {
        doubled_vec.push(item);
    }

    doubled_vec
}

fn main() {
    // 長さ5、[1, 2, 3, 4, 5]
    let input_vec: Vec<u8> = vec![1, 2, 3, 4, 5];
    println!("元のVecの長さ: {}", input_vec.len());
    println!("元のVecの中身: {:?}", input_vec);

    // 長さ10、[1, 2, 3, 4, 5, 5, 4, 3, 2, 1]
    let result_vec = double_vec_with_reverse(&input_vec);
    println!("返還されたVecの長さ: {}", result_vec.len());
    println!("返還されたVecの中身: {:?}", result_vec);
}