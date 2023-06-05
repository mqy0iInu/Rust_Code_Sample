const EPSILON: f64 = 1e-15;

fn gauss_legendre_pi() -> f64 {
    let mut a: f64 = 1.0;
    let mut b: f64 = 1.0 / f64::sqrt(2.0);
    let mut t: f64 = 0.25;
    let mut p: f64 = 1.0;

    loop {
        let a_next = (a + b) / 2.0;
        let b_next = f64::sqrt(a * b);
        let t_next = t - p * (a - a_next).powi(2);
        let p_next = 2.0 * p;

        if (a - a_next).abs() < EPSILON {
            break;
        }

        a = a_next;
        b = b_next;
        t = t_next;
        p = p_next;
    }

    (a + b).powi(2) / (4.0 * t)
}

fn main() {
    let pi = gauss_legendre_pi();
    println!("Calculated value of pi: {}", pi);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss_legendre_pi() {
        let calculated_pi = gauss_legendre_pi();
        let expected_pi = 3.141592653589793;

        assert!(
            (calculated_pi - expected_pi).abs() < EPSILON,
            "Calculated pi: {}, Expected pi: {}",
            calculated_pi,
            expected_pi
        );
    }
}
