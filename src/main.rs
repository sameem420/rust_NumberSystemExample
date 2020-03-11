fn main() {
    let binary_number = 0b100101;
    let decimal_number = 135;
    let octal_number = 0o74;
    let hexa_number = 0x77f;

    // rust will auto convert them to the decimal for human understanding
    println!("Binary to Decimal Is : {}", binary_number);
    println!("Decimal Is : {}", decimal_number);
    println!("Octal to Decimal Is : {}", octal_number);
    println!("HexDecimal to Decimal Is : {}", hexa_number);    
}
