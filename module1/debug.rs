// this struct is not printable with "fmt::display" or "fmt::debug"
struct UnPrintable(i32);

// 'derive' attribute automatically creates the implementation
#[derive(Debug)]
struct DebugPrintable(i32);

// basic structure
#[derive(Debug)]
struct Structure(i32);

// put a structure inside the structure 'Deep', makes it printable
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    // printing with '{:?}' is similar to with '{}'
    // all std library types are automatically printable with {:?}
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
            "Slater",
            "Christian",
            actor="actor's");

    // 'structure is printable'
    println!("Now {:?} will print!", Structure(3));

    // derive does not provide a way to control how the results look
    println!("Now {:?} will print!", Deep(Structure(7)));

    // person struct
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:#?}", peter);
}