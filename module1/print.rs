fn main() {
    // the {} will be automatically replaced with nay arguments
    println!("{} days", 31);

    // position arguments inside {} can be used
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // arguments can be named
    println!("{subject} {verb} {adjective}",
            subject="the dog",
            verb="runs",
            adjective="fast");

    // formatting can be invoked by a format character
    println!("Base 10:                  {}", 69420);   // 69420
    println!("Base 2 (binary):          {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):           {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal)     {:x}", 69420); // 10f2c

    // right justify text, >5 is a total width of 5
    println!("{number:>5}", number=1);

    // pad numbers with zeroes
    println!("{number:0>5}", number=1); // 00001
    // left adjust by flipping the sign
    println!("{number:0<5}", number=1); // 10000

    // can check if correct arguments are given, if "james" is removed, it gives error
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // printing pi with decimals
    let pi = 3.141592;
    let printitem = format!("{0:.3}", pi);
    println!("{}", printitem);
}