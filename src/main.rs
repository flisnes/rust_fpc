fn main() {
    println!("Main function start");

    let pi: f64 = 3.141592653589793;

    let pi_q16_16: i32 = float_to_fixed(pi, 16);

    let pi_from_q: f64 = fixed_to_float(pi_q16_16, 16);
    println!("fixed: {}, float: {}", pi_q16_16, pi_from_q);
    println!("Conversion error: {}", pi-pi_from_q);

}

fn float_to_fixed(num: f64, frac_bits: i8) -> i32 {
    return (num * (1 << frac_bits) as f64 ) as i32;
}

fn fixed_to_float(num: i32, frac_bits: i8) -> f64 {
    let mut c: i32 = num.abs();
    let mut sign: f64 = 1.0;
    let mut f: f64;

    if num < 0 {
        c = num - 1;
        c = !c;
        sign = -1.0;
    }

    f = (1.0 * c as f64) / (1 << frac_bits) as f64;
    f = f * sign;

    return f;
}