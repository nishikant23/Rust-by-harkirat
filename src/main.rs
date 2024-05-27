//use std::fmt::Debug;
//use std::result::Result; // Ensure that you are using the Result type from the standard library
use std::fs::{self};

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
    //Jargon-1 : Mutability MM
    let _temp = 10; //Rust bydefault makes each var(int,char,string,bool) = immutable
    //it makes multithread to access this var w/o worry/check if it is changed/updtaed or not, makes code faster.
    //Deliberately we have to specify the var is mutable/changable 'let mut temp'
    //other langs; javascript also has immutable feature of vars. but not bydefault they are.

    //Jargon-2 : Stack vs Heap MM
    //fixed-size-var(int,bool,arr), 1-over-other like recursive stack fun-call, STACKS-MM 
    //var(string, list/vector) dynamic-space on runtime req. 
    //Hence, STACK(ptr/ref, lenOfDynVar, CapacityOfHEAP) --->points to 1st elem.ofDynData-address & move 1step below it to access.nextDynData in HEAP.
    //On runtime Update of DynData.size().Heap.capacity, new HEAP.Capacity crted in another Adds. in RAM & STACK(ptr,len,cap) -->to new HEAP
    //Conclusion: STACK is Owns his DATA and access directly && HEAP asks OS for Data allocation each time Ownership of Memory heaps doesn't have.

    stcack_fn();
    heap_fn();
    update_fn();

    //Jargon-4: REFRENCES
    //is like pass by refrence a data, using & like:- ptr_of_c

    // Type_1: W/o passing ownership to any other variable, just letting any no. of vars can borrow our data,-
    //-for use not explicit oprtn. allowed to done on it  (borrow Rihana for only talk)
    fn new_main() {
        let my_str = String::from("I am rihana");
        let str2 = &my_str; //str2 borrowed data fro my_str
        let str3 = &my_str; //sAa
        let _str4 = &my_str; //sAa
        println!("str2 : {}",str2);
        println!("str3 : {}",str3);
        println!("owner data : {}",my_str); //Error-free, since ownership is still at my_str
        borrow_str(&my_str); // data:Pass by refrence
    }
    fn borrow_str(new_str: &String) {
        println!("new_str: {}", new_str);
       // new_str.push_str("will u update rihana"); //ERROR Prone, since not allowed to madeCHanges on Borrowed data
    }
    new_main();

    // TYPE-2: Can allowed to do made changes/oprtns on borrowed data also, now only 1 borrowing allowed this time
    fn new_main2() {
        let mut my_str = String::from("I am rihana");
        mutable_borrow_str(&mut my_str); // data:Pass by refrence
    }
    fn mutable_borrow_str(new_str: &mut String) {
        println!("new_str: {}", new_str);
        new_str.push_str(" will u update rihana"); //ERROR Free, since passed as '&mut' mutable refrence allowed to madeCHanges on Borrowed data.
        println!("Borrowed changes: {}", new_str);
    }
    new_main2();

    // TYPE - 3: Once, borrowed as MutablE, we can't borrow again RIHANA(data) as IMMUTABLE or MUTABLE
    //CAUSE-1: For Immut, as Mut-borrower changed data, then Immut-borrower thinks what happened how my holding data chnaged
    //CAUSE-2: Fo Mut,1_THREAD Mut-borrower changedData & 2_THREAD Mut-borrower changed-data Synchornize error ass same addres HEAp data accessed & try to change data.

    fn new_main3() {
        let mut my_str = String::from("I am Rihana");
        let mut_borrower = &mut my_str;
        mut_borrower.push_str(" i mutaed rihana");
        println!("Data: {}", mut_borrower);
        // let str2 = &mut my_str; //If str2_mutable borrower, not used the below either 1line can work, RUST smart that str2 used 
        //let str3 = &my_str; // Error prone: CAUSE-1, Immut
        //let str4 = &mut my_str; //Error prone: CAUSE-2, Mut 
    }
    new_main3();

    //Lifetime & Slice str Remains to done

    //STRUCTS
    // TYPE_01: struct = structure of our user-defined object
    struct Users {
        name: String,
        age: u32,
        isliving: bool,
    }

    fn struct_fun() {
        let new_user = Users {
            name: String::from("Vegita"), //String, name in Stack(name) points to HEAP(vegita)
            age: 2000, //only stroied in STACK
            isliving: true, //only stroied in STACK
        };
        println!("{} is {} yrs old & with goku {} fine.", new_user.name, new_user.age, new_user.isliving)
    }
    struct_fun();
    // TYPE_02: IMPL impl struct
    struct Rect {
        width: u32,
        height: u32,
    }

    impl Rect {
        fn area(&self) -> u32 {
            return self.width*self.height;
        }
        fn perimeter(&self) -> u32 {
            2*(self.width + self.height) // reruening stment, identify by: not wrote-->> "return & ;(semicolon)"
        }
    }

    fn find_area() {
        let value = Rect {
            width: 30,
            height: 50,
        };
        println!("the rectangle area is {} ", value.area());
        println!("the perimeter is {} ", value.perimeter());
    }
    find_area();

    //TYPE-03: Unit-Struct, struct which has no attributes
    struct NoShape;

    impl NoShape {
        fn area(&self) -> u8 {
            return 0;
        }
    }

    fn find_empty() {
        let value = NoShape;
        println!("the NoShape area is {} ", value.area());
    }
    find_empty();

    //ENUMS
    enum Directions { //when know limited varying varss req. instead using Strings and passing literal every where,
                      //we can make use of enum, makes the code more string and easy coding cleaner 
        East,
        West,
        North, 
        South,
    }

    fn move_player() {
        let player_direction = Directions::West;
        game_move(player_direction);
        //println!("this player moving on {} side", player_direction);
    }
    fn game_move(moving: Directions) {
        //Implement code here
    }
    move_player();

    //Type-02: Enums with values
    enum Shapes {
        Circle(f64),
        Square(f64),
        Rectangle(f64, f64),
    }

    fn area_calc(shape: Shapes) -> f64 {
        let ans = match  shape { //shape UD-datatype matches with ENUM-Attributes,
            Shapes::Circle(radius) => 3.14*radius*radius, //when matched do this opretions
            Shapes::Square(side) => {  //when matched do this opretions
                //println!("this square area: ");
                side*side
            },
            Shapes::Rectangle(width, height) => width*height,  //when matched do this opretions
        };
        ans //returning statement, w/o use of "rerturn & ;"
    }
    fn main_call() {
       let circle = Shapes::Circle(8.5);
       let square = Shapes::Square(5.3);
       let rect = Shapes::Rectangle(5.0, 6.0);

       let mut area  = area_calc(circle);
       println!("The circle result is {}\n.", area);

       area  = area_calc(square);
       println!("The square result is {}\n.", area);

       area  = area_calc(rect);
       println!("The rectangle result is {}\n.", area);
    }
    main_call();

    //ENUM ERROR HANDALING

    //Type-01: Generics:allows a variable to accept/store any dataTpye, w/o explicitly define dataType before store it there
    struct Anytype<T> {//x, y, z, can NOT store any diff-dataType w.r.t e/o at same time.
        x: T, //x can store any data type int/float/bool etc.
        y: T,
        z: T,
    }
    struct DiffTypes<A, B, C> { //a, b, c, can store diff-datatpyes w.r.t e/o at same time.
        a: A, //a can store any data type int/float/bool etc.
        b: B,
        c: C,
    }
    fn generic_datatype() {
        let int_type = Anytype{ x:5,y:6,z:10 };
        let float_type = Anytype{ x:5.67,y:60.00,z:1.005 };
        let bool_type = Anytype{ x:true,y:false,z:true };
        //let diff_type = Anytype{ x:5,y:1.006,z:true }; //can't store like this x,y,z can only store same DT of any type

        println!("this is int dtatatpye: {}, {}, {}",int_type.x,int_type.y, int_type.z);
        println!("this is float dtatatpye: {}, {}, {}",float_type.x,float_type.y, float_type.z);
        println!("this is boolean dtatatpye: {}, {}, {}\n",bool_type.x,bool_type.y, bool_type.z);
       // println!("this is int, float, bool simultaneously used: {}, {}, {}",diff_type.x,diff_type.y, diff_type.z);

       //Generic: Different data type stroe at vars at same type
       let diff_types = DiffTypes{ a:5,b:1.006,c:true }; 
       //can store like thisa,b,c diff DT of any type at same time
       println!("this is int, float, bool simultaneously used: {}, {}, {}",diff_types.a,diff_types.b, diff_types.c);
    }
    generic_datatype();

    //Type-02: Error handling enum
    enum Result<T, E> {
        Ok(T), //T can store any type of data
        Err(E)
    }
    fn catch_error() {
        let ans  = fs::read_to_string("example.txt"); //example.txt not present
        match ans { // match our ans with Ok if example.txt present OR match with ERR if not present
            Ok(content) => { 
                println!("Data inside exampleTxt is: {}", content);
            },
            Err(err) => {
                println!("ERROR: nothing inside exampleTxt: {}", err);
            }
        }
        println!("This is printed after Error also:"); //this line wouldn't get printed, if match ans {...} will not use,
        // Also if ERR found and  match ans{...} not present then the program wouldn't get compiled further & returned 'thread' panicked like error.
    }
    catch_error();

    //Type-03: Error handling 
    // UNCOMMENT THIS BELOW TILL 322line number
    // fn err_by_fun() {
    //     let _ans  = read_unsafe_file("nodata.txt".to_string());  //pass nodata.txt as arg to function

    //     println!("This is printed after Error also:"); //this line wouldn't get printed, if match ans {...} will not use,
    //     // Also if ERR found and  match ans{...} not present then the program wouldn't get compiled further & returned 'thread' panicked like error.
    // }
    // //Type--A: returning ERROR data as String here
    // fn read_unsafe_file(no_txt: String) -> String {  // returning ERROR data as String here --- "-> String" to return 
    //     let res = fs::read_to_string(no_txt); //res,try to read and store no_txt data
    //     return res.unwrap();  //.unwrap(), will unwrap data inside no_txt if present, 
    //     //else pass error in above call. & thread 'main' panicked at src\main.rs:318:20file
    // }
    // err_by_fun();


    //Type--B: retrun enum directly in terms of OK & ERR byreturning REsult<Data, Data> 
    //if, OK then also returns Ok-content data
    //else-if, ERR  then also return data as Err-as.to_string();
    //From line 329 to 354 UNCOMMENT BUT TYPE mismatched error comming in Result
    // fn enum_err() {
    //     let _ans  = return_enum("readme.txt".to_string());  //pass nodata.txt as arg to function
        
    //     // match ans {
    //     //     Ok(data) => println!("File content: {}", data),
    //     //     Err(err) => println!("Error: {}", err),
    //     // }
    //     println!("This is printed after Error also:"); //this line wouldn't get printed, if match ans {...} will not use,
    //     // Also if ERR found and  match ans{...} not present then the program wouldn't get compiled further & returned 'thread' panicked like error.
    // }
    // fn return_enum(data_content: String) -> Result<String, String> { //return is 'enum' itself, hence use "->Result<DT_Want_toReturn, DT_Want_toReturn>"
    //     let new_res = fs::read_to_string(data_content);
    //     // match new_res {
    //     //     Ok(data_present) => Ok(data_present),
    //     //     Err(_) => Err("Error reading file data not present".to_string()),
    //     // }

    //     if let Ok(content) = new_res {
    //         return println!("this is content"); //Ok(content);
    //     } else {
    //         return Err("Error reading file data not present".to_string());
    //     }
    // }
    // fn main() {
    //     enum_err();
    // }

    // OPTIONS Enum: To handle the NuLL-ptr error
    // pub enum Option<T> {
    //     Some(T),
    //     None,
    // }

    fn find_first_a(s:String) -> Option<i32> { //try to find first char  'a' in our String s
        for (index, character) in s.chars().enumerate() { //we will find index of 'a' found char in string
           if character == 'a' {  
            return Some(index as i32); // return Some(data) is OptionEnum is returning--enum as found Some-data
            //index as i32, for our option enum<i32> we defined i32 as index size, can also use Option<usize> then--return Some(index);
           }
        }
        return None;
    }
    fn main_option_enum() {
        let my_str = String::from("Raman");
        let answer = find_first_a(my_str);
        match  answer {
            Some(idx) => println!("the first 'a' is found at {} position", idx),
            None => println!("the letter 'a' is not found"),
        }
    }
    main_option_enum();


        
}
    



//STACK vs HEAP tpc
fn stcack_fn() {
    let t = 25;
    let p = 35;
    println!("Stack: t is {} and p is {}",t,p);
}

fn heap_fn() {
    let str1 = String::from("Hello");
    let str2 = String::from("World");

    let combine = format!("{} {}",str1, str2);
    println!("Heap: Combined str - {}", combine);
}

fn update_fn() {
    let mut s = String::from("Before update text");
    println!("Original: {}",s);
    println!("capacity: {}, length: {}, pointer: {:p}",s.capacity(), s.len(), s.as_ptr()); //shows, actualHeap-capacity, lengthOfS.data,STACK-ptr-point to which M.adds of HEAP

    s.push_str("and some additional text");
    println!("Updated: {}",s);
    println!("capacity: {}, length: {}, pointer: {:p}",s.capacity(), s.len(), s.as_ptr());

    //Jargon-3: Ownership 
    // int[] arr = new int[5]; STACKStroes=ref ref-points to HEAPStores=new.5.sizeArraySpace
    //H->int[5] it owner is S->ref, if 'ref' deleted == RUSt will cleare 'int[5]' from Heap

    //Direct or By-rerfrence Ownership transfer
    fn owner() {
        let s1 = String::from("hello"); //OwnerAtStack-.s1 of "hello"At-Heap

        let s2 = s1; //2-owners of 'hello" ,NO not allowed in Rust, this line, 
        //makes s2.newOwner of 'hello' & s1.owner deleted from Stack. 
        //No 2-owners at same time allowed to 1-heapData can lead "Danggaling ptr error" & "Double free error"
        //println!("{}",s1)  //give error
        println!("{}", s2)  //error free new owner of 'hello""

        //Danggaling ptr error, s1&s2 both pts to 'hello', if 'hello' deleted with s1 say, s2 still there pointing
        //that addresss on heap, but that addresss has anyNewData which will try to access s2 as think it is'hello'

        //double free eror, s1&s2 both pts to 'hello', if 'hello' deleted with s1 say, s2 still there pointing
        //that addresss on heap, but that addresss has anyNewData, if we now Delete s2-also this s2 try to delete
        //'heelo' from that locatn. which is not there and try to delete newData cause Double-free/delete error.
    }
    owner();
    //Indirect Using functions to transfer owner ship

    fn owner_2() {
        let mut bf1 = String::from("gf_rihana");
        bf1 = change_bf(bf1);
        //println!("bf1 died {}", bf1) if we haven't retransfer gf him from his bf2
        println!("bf1 got gf back {}", bf1); 
    }

    fn change_bf(bf2:String) -> String {
        println!(" bf2 got bf1 gf in trip:  {}", bf2);
        return bf2; //before dying bf2 return back gf_rihana to his original bf1
    }

    owner_2();

}

//Function Topic
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