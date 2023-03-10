//! https://theprimeagen.github.io/rust-for-typescript-devs

// fn main() {
//     println!("Hello, world!");
// }

// fn test(){
//     let a: Vec<_> = vec![1,2,3,4,5]
//         .iter()
//         .map(|x| x +1) 
//         .collect();
//     println!("a {:?}", a)
// }

//*  data persists with collect()
// fn collect(){
//     let data = vec![1,2,3,4,5]
//     let mut foo = data
//         .iter()
//         .map(|x| x +1) 
//     let mut new_vector = vec![];

//     while let Some(x) foo.next(){
//         new_vector.push(x);
//     }

//     println!("a {:?}", new_vector)
// }

// let foo: String = vec!["this", "is", "a", "test"]
//     .into_iter() // what the heck is this?  we will talk more about this
//     .collect();




//*  Collect into HashSet (this would be Set in JS)

// let foo: HashSet<isize> = vec![1, 2, 3]
//     .into_iter()
//     .collect();




//*  Collect into a HashMap

// let foo: HashMap<&str, usize> = vec!["this", "is", "a", "test"]
//     .into_iter()
//     .enumerate() // Adds the index to the iterator!
//                  // one of the glories of rust is that we work with iterators
//     .map(|(idx, item)| (item, idx)) // reverses the order
//     .collect();
// map(|(idx, item)| is an example of destructuring.



//* Iterators from other collections! */
// let map = HashMap::from([
//    ("foo", 1),
//    ("bar", 2),
//    ("baz", 3),
// ]);

// map
//     .iter()
//     .for_each(|(k, v)| println!("{}: {}", k, v));

// let set = HashSet::from([
//     "foo",
//     "bar",
//     "baz",
// ]);

// set
//     .iter()
//     .for_each(|v| println!("{}", v));


//* You can even create your own iterators! We will soon, but here is a basic example!*/

// let todos = Todo { ... values ... }

// for task in &todos { // requires trait implementations
//     println!("I need to do: {}", task); // require trait implementations
// }


//! Lines exercise

// fn main() {
//     let file = std::fs::read_to_string("lines").unwrap();

//     file
//         .lines()
//         .for_each(|line| println!("{}", line));
// }

//*Skipping exercises, filtering */
// fn main() {
//     let file = std::fs::read_to_string("lines").unwrap();

//     file
//         .lines()
//         .enumerate()
//         .filter(|(idx, _)| idx % 2 == 0)
//         .for_each(|line| println!("{}", line.1));
// }
//*One more do these steps IN ORDER.every other lineskip the first two linesprint two lines */

// fn main() {
//     let file = std::fs::read_to_string("lines").unwrap();

//     file
//         .lines()
//         .enumerate()
//         .filter(|(idx, _)| idx % 2 == 0)
//         .skip(2)
//         .take(2)
//         .for_each(|line| println!("{}", line.1));
// }


//     .lines()
//     .enumerate()
//     .filter(|(idx, _)| idx % 2 == 0)
//     .skip(2)
//     .take(2)
//     .for_each(|line| println!("{}", line.1));

//* */ Goes through every char
// let mut start = 0;
// let mut taken = 0;
// let mut skipped = 0;
// let mut lines_found = 0;
// for (idx, c) in lines.enumerate().chars() {
//     if c !== "\n" {
//         continue;
//     }

//* */     // doesn't copy, just a &str (ptr, len)
//     let slice = lines[start..idx];
//     start = idx + 1;

//     lines_found += 1
//     if lines_found % 2 == 0 {
//         continue
//     }

//     if skipped < 2 {
//         skipped += 1;
//         continue;
//     }

//     taken += 1;
//     println!("{}", slice);

//     if taken == 2 {
//         break;
//     }
// }


//* Zero Cost Adstractions */

//*Enums */
// enum Color {
//     Red,
//     Green,
//     Blue,
//     //Add yellow and in does not allow for the program to compile
//     Yellow
// }

// fn print_color(color: Color) {
//     match color {
//         Color::Red => println!("red"),
//         Color::Green => println!("green"),
//         Color::Blue => println!("blue"),
//         //need to create the function for yellow
//         Color ::Yellow => println!("yellow")
//     }
// }

// fn main() {
//     print_color(Color::Green);
// }

//* Next Level enums*/

// enum Color {
//     Red,
//     Yellow,
//     Green,
//     Blue,
// }

// impl Color {
//     fn is_green_parts(&self) -> bool {
//         match self {
//             Color::Yellow => true,
//             Color::Blue => true,
//             _ => false,
//         }
//     }

//     fn is_green(&self) -> bool {
//         if let Color::Green = self {
//             return true;
//         }
//         return false;
//     }
// }

// fn print_color(color: Color) {
//     match color {
//         Color::Red => println!("red"),
//         Color::Green => println!("green"),
//         Color::Blue => println!("blue"),
//         Color::Yellow => println!("yellow"),
//     }
// }

// fn main() {
//     print_color(Color::Red);
//     //allows for checking of methods on enums
//     Color::Green.is_green();
// }


//* More Enums Excersise - with structs */
//* Our enums can have subtypes, much more operant that TS */

// struct Custom {
//     age:usize,
//     name: String
// }

// enum Item {
//     Number(usize),
//     String(String),
//     MyCustom(Custom),
// }

// fn append(items: &mut Vec<Item>) {
//     items.push(Items::String("hello, Fem".into())) //into changes the data into the correct type
// }


// fn main(){
//     let mut items: Vec<Item> = vec![];
//     append(&mut items)
// }

//! This means no more

// if (typeof x === "number") {
//     ...
// }
// or

// if ("bar" in x) {
//     ...
// }
// So no more "magic" checking for types, you get named types and this works very well with non type discriminated unions (what we made). This is because the discrimination exists at a language level, not a type: string level

// its not all magic
// Sometimes code can become a bit more verbose because of this, and that isn't as nice to write. But at the same time, it prevents easy errors where you forgot to handle cases.


//? This allows for matches outside of runtime, very efficient

// struct Custom {
//     name: String,
//     age: usize,
// }

// enum Item {
//     Number(usize),
//     Custom(Custom),
//     String(String),
// }

// fn main() {
//     let foo = Item::Number(5);

//     match &foo {
//         Item::Number(num) => println!("i am a number: {}", num),
//         Item::String(str) => println!("i am a string: {}", str),
//         Item::Custom(custom) =>
//             println!("name: {}, age: {}", custom.name, custom.age),
//     }

//     match &foo {
//         Item::Custom(custom) =>
//             println!("name: {}, age: {}", custom.name, custom.age),
//         _ => {}
//     }

//     match &foo {
//         Item::Custom(Custom {
//             age,
//             ..
//         }) => println!("age: {}", age),
//         _ => {}
//     }

//     match &foo {
//         Item::Custom(custom) if custom.name == "Ricky" =>
//             println!("Hi, Ricky"),
//         Item::Custom(custom) if custom.age > 33 =>
//             println!("N64 was the best console"),
//         Item::Custom(custom) if custom.age < 30  =>
//             println!("Xbox was the best console"),
//         _ => {}
//     }
// }


//* Options */

//? Options are enums that allow for null and undefined

// enum Option<T> { // yes, generics can be used in enums, again, cool
//     None
//     Some(T),
// }

// let foo = Some(5);
// let foo = Some("different type");
// let foo = Some(Custom { age: 69, name: "ThePrimeagen" });

// ?Why do we need Options in rust? The answer is memory. If you might or might not return an item from a function, rust needs to be able to allocate that memory on the stack. (we will talk about this more shortly)
// This happens in JS too, it is just behind the scenes in the engine
//? Because they are enums, can use match/if

// fn multiply(num: Option<usize>) -> usize {
//     if num.is_some() {
//         return num.unwrap() * 5; // unwrap a None causes a panic
//     }
//     return 0;
// };
// //Newer:
// fn multiply(num: Option<usize>) -> usize {
//     return num.unwrap_or(0) * 5;
// }

// //? instead of returing 0, if undefined is provided, return undefined else multiply by 5
// fn multiply(num: Option<usize>) -> Option<usize> {
//     return num.map(|x| x * 5); // remains as an option
// };

// fn multiply(num: Option<usize>) -> Option<usize> {
//     return Some(num? * 5);
// }

//*Errors */

// if let Ok(value) = a_function_that_can_error() {
//     // something with the value
// }

// match a_function_that_can_error() {
//     Ok(value) => println!("oh yeah, value! {}", value);
//     Err(e) => eprintln!("ohh no... {}", e);
// }

// // you don't care about the error
// _ = a_function_that_can_error();

// // yolo
// let foo = a_function_that_can_error().unwrap();

// // respectful yolo
// let foo = a_function_that_can_error().expect("should never fail");

// // defaults
// let foo = a_function_that_can_error().unwrap_or(0);

// convert to option
// Ok(V) => Some(V)
// Err(E) => None
// bai felicia
// let foo = a_function_that_can_error().ok();

// let foo = a_function_that_can_error()
//     .map(|value| value + 1);

// let foo = a_function_that_can_error()
//     .and_then(|value| another_possible_error(value))
//     .and_then(|value| again(value));

// // If your function returns an error, you can do this!
// let foo = a_function_that_can_error()?;

//? Side Note
// there are two crates (rust package) that work very well with errors

// thiserror - great for creating your own errors. should be used in libraries
// anyhow - great for applications.

// fn main() {
//     let arg = std::env::args().nth(1)
//         .expect("please provide a file name as an argument");

//     std::fs::read_to_string(arg)
//         .expect("unable to read the file provided")
//         .lines()
//         .for_each(|line| println!("{}", line));
// }

//?lets only print out lines that are numbers and lines that are not, lets print out Line not a number

//there is a singular parse method! 

// fn main() {
//     let arg = std::env::args().nth(1)
//         .expect("please provide a file name as an argument");

//     std::fs::read_to_string(arg)
//         .expect("unable to read the file provided")
//         .lines()
//         .for_each(|line| {
//             if let Ok(value) = line.parse::<usize>() {
//                 println!("{}", value);
//             } else {
//                 println!("Line not a number");
//             }
//         });
// }

//*The Borrow Checker */

// Derives Debug allows for printing of the struct.

// #[derive(Debug)]
// struct Item {
//     count: isize,
// }

// fn add_one(mut item: Item) {
//     item.count += 1;
// }

// fn main() {
//     let item = Item { count: 0 };
//     println!("item {:?}", item);
//     add_one(item);
//     println!("item {:?}", item); // <--- why does this error
// }
// // Let me ask you this question, who owns Item on this line?

// let item = Item { count: 0 };
// the var needs to be borrowed
// fn add_one(item: &mut Item)
// \/
// add_one(&mut item)
// \/
// let mut item = Item {count: 1}

//?Think about what owns the value, how long does it persists

// #[derive(Debug)]
// struct Item {
//     count: isize,
// }

// fn add_one(item: &mut Item) {
//     item.count += 1;
// }

// fn print_all(items: &Vec<Item>) {
//     for item in items {
//         println!("print {:?}", item);
//     }
// }

// fn main() {

//     let mut items = vec![
//         Item { count: 0 },
//         Item { count: 0 },
//         Item { count: 0 },
//     ];

//     let item = items.get_mut(0).unwrap();
//     add_one(item);
//     print_all(&items);

//     println!("help {:?}", item);
// }

//! Quick Recap: The big three rules
//? There can only be one value owner
// let item = Item { age: 10 };
// let other = item; // value moved here

// println!("{:?}", item.age); // borrow of moved value (item moved to other)




// //? There can be 0 mutable borrows when there are 1 or more immutable borrows
// let mut items = vec![Item { age: 1 }, Item { age: 2 }];

// let items2: &Vec<Item> = &items; // immutable borrow occurs here
// let items3: &mut Vec<Item> = &mut items; // cannot borrow mutably with
//                                          // immutable references out

// items2.get(0); // item3 is mutably borrowed




//? There can only be 1 mutable borrow
// let mut items = vec![Item { age: 1 }, Item { age: 2 }];

// let items2: &mut Vec<Item> = &mut items; // First mutable borrow
// let items3: &mut Vec<Item> = &mut items; // Error occurs here

// items2.push(Item { age: 3 }); // nope!

//? Applications of the rules
// // There is a "flow" to references
// // Since items2 was not used when items3 mutable borrow out, this is ok

// let mut items = vec![Item { age: 1 }, Item { age: 2 }];

// let items2: &mut Vec<Item> = &mut items; // First mutable borrow
// items2.push(Item { age: 3 }); // ok!

// let items3: &mut Vec<Item> = &mut items; // Second mutable borrow
// items3.push(Item { age: 3 }); // still ok!

//? References cannot outlive their associated values
// let y: &usize;
// {
//     let x: usize = 5;
//     y = &x;
// }

// println!("ooh no! {}", y);

//*         Traits                  
// You can define most of rust's behavior, from addition, equality checks, hashing, parsing strings, to being displayed via traits.
//? Traits are effectively Interfaces but how they are used are a bit different, and how the language lets you specify them is different.
// What traits allow you to do cannot be directly done in JS. The language doesn't have the ability to do the same thing.

// struct Rect {
//     x: f64,
//     y: f64,
//     width: f64,
//     height: f64,
// }

// struct Circle {
//     x: f64,
//     y: f64,
//     radius: f64,
// }
mod shapes;

use std::f64:consts::PI;

use crate::shapes::{rect::Rect, circle::Circle, area::Area};


trait Area {
    fn area(&self) -> f64;
}

impl Area for Rect {
    fn area(&self) -> f64;
        retrun self.width * self.height; 
}
impl Area for Circle {
    fn area(&self) -> f64;
        retrun self.radius * self.radius * PI; 
}

fn main() {
    let circle = Circle {
        x: 0f64, y: 0f64,
        radius: 4f64,
    };
    let circle = Circle {
        x: 10f64, y: 10f64,
        radius: 7f64,
    };
    let rect1 = Rectangle::default();
    let rect2 = Rectangle::default();



    println!("{}", rect);


    println!("area: {}", rect.area());
    println!("area: {}", circle.area());
}

