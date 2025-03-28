use std::time::Instant;

fn modulo(a: i128, b: i128) -> i128 {
    a % b
}

fn modulo_with_power_dp(a: i128, power: i128, b: i128) -> i128 {
    if power == 0 {
        return 1 % b;
    }
    let mut result = 1;
    let mut base = modulo(a, b);
    let mut exp = power;

    while exp > 0 {
        if exp % 2 == 1 { // analyze the binary number from the rtl
            println!("triggered {:?}", exp);
            result = modulo(result * base, b);
        }
        base = modulo(result * base, b);
        println!("{:?}", base);
        exp /= 2;
    }

    result
}

fn modulo_with_power_unoptimized(a: i128, power: i128, b: i128) -> i128 {
    let mut result = 1;
    for _ in 0..power {
        result = (result * a) % b;
    }
    result
}

fn modulo_addition_chain_with_power(a: i128, power: i128, b: i128) -> i128 {
    // convert power to biner and store in array
    // 32 16 8 4 2 1
    // 23 -> 1 0 1 1 1
    let mut binary_power = Vec::new();
    let mut temp_power = power;
    while temp_power > 0 {
        binary_power.push(temp_power % 2);
        temp_power /= 2;
    }
    binary_power.reverse();

    // starting the logic
    let mut h: i128 = 1;
    for i in &binary_power {
        h = h.pow(2) % b;
        if i == &1 {
            h = (h * a) % b;
        }
    }

    h
}

fn main() {
    // view how many memory taken if large number
    let start = Instant::now();
    println!("{:?}", modulo_addition_chain_with_power(12, 8, 5));
    let duration = start.elapsed();
    println!("modulo_addition_chain_with_power execution time: {:32} seconds", duration.as_secs_f32());

    let start = Instant::now();
    println!("{:?}", modulo_with_power_unoptimized(12, 8, 5));
    let duration = start.elapsed();
    println!("modulo_with_power_unoptimized execution time: {:32} seconds", duration.as_secs_f32());

    let start = Instant::now();
    println!("{:?}", modulo_with_power_dp(12, 8, 5));
    let duration = start.elapsed();
    println!("modulo_with_power_dp execution time: {:32} seconds", duration.as_secs_f32());

    let start = Instant::now();
    println!("{:?}", reverse_cipher("hello"));
    let duration = start.elapsed();
    println!("reverse_cipher execution time: {:32} seconds", duration.as_secs_f32());
}
