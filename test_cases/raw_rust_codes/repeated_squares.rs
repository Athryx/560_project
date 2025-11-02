fn pow(base: u64, exponent: u64, modulo: u64) -> u64 {
    if exponent == 0 {
        return 1;
    } else if exponent == 1 {
        return base;
    }

    let result = pow(base, exponent >> 1, modulo);
    if (exponent & 1) != 0 {
        return (result * result * base) % modulo;
    } else {
        return (result * result) % modulo;
    }
}

fn main() {
    println!("{}", pow(50, 28, 65537));
    println!("{}", pow(50, 3, 65537));
}
