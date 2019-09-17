pub fn run() {

    // Simple print
    println!("Hello from the print.rs file");

    // Basic formating
    println!("{} if from {}", "Brad", "Mass");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Named arguments
    println!("{name} likes to play {activity}", name="John", activity="Baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);

}