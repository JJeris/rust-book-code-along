use std::io;

fn main() {
    loop {
        println!("Input the temperature in Fahrenheit.");
        // User inputs *F.
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");

        // Input conversion to a float.
        let input_f: f64 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        // Float conversion to *C using this: C = (F-32)*5/9.
        let output_c = (input_f - 32.0) * (5.0/9.0);
        
        // Print out the conversion result.
        println!("{input_f}F is equivalent to {output_c}C.");
    }
}
