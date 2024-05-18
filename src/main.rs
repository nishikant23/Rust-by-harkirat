fn main() {
    let x: i8 = -5;
    let y: u8 = 250;
    let z: f32 = 55.005;

    println!("x:{}, y:{}, z:{}",x,y,z);

    let mut a = 10; //mut = mutable can change same var values 
    for i in 0..100{
        a = a+i;
    }
    print!("a:{}", a);
     
    let mut is_male: bool = false;
    let is_above_18: bool = true;

    if is_male {
        println!("Your are male ");
    } else {
        println!("Your are not male");
    }
     
    is_male = true; //re-defined & value changed since is_male is mutable
    if is_male && is_above_18 {
        print!("you are legal to drive & Legal Male");
    } 

    //Strings
    // strings size vary in RUST as per user it increase its Size in RAM diff. to handle
    // unlike ints, we cant able to reserved size for string, 
    let greetings = String::from("Hello, everyone I am learning rust");
    println!("{}",greetings);  //way of printing any data in RUST curlyBraces{}

    //Character Dealing
    //JS greeting[2] -> [print 2nd index of str char]
    //RUST greeting[2] or greeting.chars().nth(2) -> find at nth =2th index char +> throw error
    //bcos, RUST is Compile time lang, it says I dont know if str's nth char u gave isCharPresnt or nothing there(str ended)
    // we as devloper cautious while coding there we use: match char{}+avoid error OR char.unwrap() we OK with compile error

    let char1 = greetings.chars().nth(3);

    match char1 {   //Error Catching syntax 
        Some(c)=> {println!("{}", c)},
        None => println!("No charater at nth index"),
    }
    
    //println!("{}", char1.unwrap());   Another way to avoid  but throw error   

    //Conditional loops//

    for i in 0..10 {  //this prints from 0 to 9 **not 10
        println!("{}", i);
    }

    for j in 0..5 { 
        println!("Hello World! {}", j) //value of j also prints
    }

    for _k in 0..5 {  // we use underscore in fornt of k =_k, do when u know u r not gonna use iterating variable
        println!("Hello World!") //other wise it will give error
    }

    //Charater loop
    let sentence = String::from(" Nishikant is my name");
    let first_word = get_last_word(sentence);
    println!("Last word: {}", first_word);  //"{}" imp to print data 

    //FUNCTIONS in RUST

    let a = 10;
    let b = 25;
    let sum: i32 = get_sum(a, b);
    println!("Sum of {} and  {} is : {}", a,b,sum);

    //MEMORY MANAGEMENT
    //Jargon-1 : Mutability
    let temp = 10; //Rust bydefault makes each var(int,char,string,bool) = immutable
    //it makes multithread to access this var w/o worry/check if it is changed/updtaed or not, makes code faster.
    //Deliberately we have to specify the var is mutable/changable 'let mut temp'
    //other langs; javascript also has immutable feature of vars. but not bydefault they are.


}

fn get_sum(a:i32 ,b:i32) -> i32{
    return a+b;
}
//u can not write fun-name as getLastWord ->error required in "snakeCase"
fn get_last_word(sentence: String) -> String {   //1st-String is Datatype of 'sentence', 2nd-String- return-type of a function
    let mut ans = String::from(""); //empty string initialised

    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;

//  char.to_string() converts the character to a String.
//  .as_str() converts the String to a string slice (&str). casue push_str accepts stringSlice=&str not char
//  ans.push_str(...) appends this string slice to the String ans.

//String is a heap-allocated, growable string type.
//It owns the string data and manages its memory.
//Example: let s = String::from("hello");

//&str is a string slice, which is a borrowed reference to a string.
//It can refer to part or all of a string.
//It is a view into a String or a string literal without taking ownership.
//Example: let s: &str = "hello"; or let slice: &str = &s[0..4];

//Immutability: &str is immutable, meaning you cannot modify the string data through a &str reference.
//Borrowed: A string slice borrows the string data it refers to, which means it does not own the data and is subject to Rust's borrowing rules.
//Efficient: String slices are efficient because they avoid copying the string data. They just point to a part of the string

}