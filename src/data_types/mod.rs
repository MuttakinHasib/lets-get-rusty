pub fn data_types() {
    let mut x = 5;

    println!("The value of x is {x}");

    x = 6;

    let x = x + 1;

    println!("The value of x is {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}");

    // Scaler data types

    // Integer types
    let a = 98_222; // decimal
    let b = 0xff; // hexadecimal
    let c = 0o77; // octal
    let d = 0b1111_0000; // binary
    let e = 1_000_000; // underscores for readability
    let f = 1_000_000_000; // underscores for readability
    let g = b'A'; // character type byte

    // Floating point types
    let h = 2.0;
    let i: f32 = 3.0;
    // Boolean types
    let j = true;
    let k: bool = false;
    // Character types
    let l = 'L';
    let m: char = 'M';
    // Compound types
    // Tuple types
    let n = ("Hasib", 20);
    let (name, age) = n;
    let his_age = n.1;
    // Array types

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    // let x = error_codes[3]; // This will cause a runtime error
    let byte = [0; 8];
}
