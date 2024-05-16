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
    
}
