mod ch02;

fn main() {
    println!("Hello, world!");

    println!("ch2.2");
    ch2_2();
}

fn ch2_2() {
    println!(
        "{}, {}, {}, {}",
        13 % 5,
        i32::abs(-13),
        10_i32.pow(2),
        4.0_f64.sqrt()
    );

    println!("{:08b}", 25_u8);
    println!("{:08b}", -20_i8 & -70_i8);
    println!("{:08b}", -20_i8 | -70_i8);
    println!("{:08b}", -20_i8 ^ -70_i8);
    println!("{:08b}", 100i8 << 2);
    println!("{:08b}", 150_u8 >> 2);
    println!("{:08b}", -15_i8 >> 2);
}
