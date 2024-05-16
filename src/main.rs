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
}
