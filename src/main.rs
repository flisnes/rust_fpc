fn main() {
    println!("Main function start");

    let pi: f64 = 3.141592653589793;
    let sqrttwo: f64 = 1.414213562373095;

    let pi_q16_16: i32 = float_to_fixed(pi, 16);
    let sqrttwo_q16_16: i32 = float_to_fixed(sqrttwo, 16);

    let pi_from_q: f64 = fixed_to_float(pi_q16_16, 16);
    println!("fixed: {}, float: {}", pi_q16_16, pi_from_q);
    println!("Conversion error: {}", pi-pi_from_q);

    println!("q_add: {} -> as float: {}", q_add(pi_q16_16, sqrttwo_q16_16), fixed_to_float(q_add(pi_q16_16, sqrttwo_q16_16), 16));

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

fn q_add(a: i32, b: i32) -> i32 {
    let result: i32;
    let mut tmp: i64;
    tmp = a as i64 + b as i64;

    // Saturate
    if tmp > 0x7FFFFFFF {
        tmp = 0x7FFFFFFF;
    }
    if tmp < -1 * 0x80000000 {
        tmp = 0x80000000;
    }

    result = tmp as i32;
    return result;
}