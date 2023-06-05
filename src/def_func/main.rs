// ガウス・ルジャンドル法による円周率の計算
fn calculate_pi() -> f64
{
    let mut a = 1.0;
    let mut b = 1.0 / 2.0_f64.sqrt();
    let mut t = 1.0 / 4.0;
    let mut p = 1.0;

    for _ in 0..5 {
        let a_next = (a + b) / 2.0;
        let b_next = (a * b).sqrt();
        let t_next = t - p * (a - a_next).powi(2);
        let p_next = 2.0 * p;

        a = a_next;
        b = b_next;
        t = t_next;
        p = p_next;
    }

    let pi = (a + b).powi(2) / (4.0 * t);
    pi
}

// フィボナッチ数列の生成
fn generate_fibonacci(n: u32) -> Vec<u64>
{
    let mut fibonacci = vec![0, 1];

    for i in 2..=n {
        let next = fibonacci[(i - 1) as usize] + fibonacci[(i - 2) as usize];
        fibonacci.push(next);
    }

    fibonacci
}

fn main()
{
    // 円周率の計算と表示
    let pi = calculate_pi();
    println!("π ≈ {}", pi);

    // フィボナッチ数列の生成と表示
    let fibonacci = generate_fibonacci(10);
    println!("Fibonacci Sequence: {:?}", fibonacci);
}
