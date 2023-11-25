// mod helper;

// use helper::starter::multiply;

// fn main() {
//     let result = multiply(5, 6);
//     println!("The result is: {}", result);
// }

//variable
// fn main (){
//     let x: i32 = 9;
//     let y: i32 = 5;
//     assert_eq!(x,y);
//     println!("success")
// }

//mut
// fn main (){
//     let mut x:i32 =5;
//     x+=5;
//     println!("x is {}",x);
// }
//unused variable
// #[allow(unused_variables)]
// fn main (){
//     let x:i32 =5;

// }
//types necessary for summations
// fn  main (){
//    assert_eq!( 0.1_f32+0.2_f32,0.3_f32);
//    println!("success")
// }
//range
// fn main (){
//    let mut  sum : i32 = 0;
//   for i in -3 .. 5{//this for loop does not inclued 5
//    sum += i;

//   }
//   assert!(sum==4);
//   println!(" the sum is {}",sum);

// }

//make println output 97-122
// fn main(){
//    for c in 'a' ..='z'{
//       println!("{}",c as u8 );
//    }

// }

//  fn main (){
//    let x:i32=5;
//    let y: i32={
//       let a:i32= 9 * x;
//       let b:i32 = 10 * x;
//       a + b
//    };
//    println!("{}",y)

//  }
//  // unit
//  fn main (){
//    // let unit:()=();
//    // assert_eq!(unit,());
//    // println!("success");
//    let v ={
//       let mut m:i32=1;
//         m+=1;
//         m
//    };
//    assert_eq!(v,2_i32);
//    println!("success")

//  }

// fn main (){
//    let (x,y)=(5,8);
//    let s=sum(x,y);
//    println!("sum is {}",s)
// }
// fn sum(x:i32,y:i32)-> i32{
//    x+y

// }
// fn main (){
//     let s1=String :: from ("hello");
//     let s3:String="hello world ".to_string();
//     let s2=s1;//after reasinging the string s1 to s2 ,si pointer is copied to the heap ,but its values are droped since they can only be one owner at a time
//     println!("{}",s2);
//     println!("{}",s3);

// }
//  fn main (){
//     let s1:String="my name is king paul".to_string();
//  take_owner_ship(s1);
// //  println!("{}",s1); this code wont work cause s1 onwership was moved to the function take_owner_ship  therefore in this main scope s1 ios droped
// let x=9+3;
// println!("answer is {}",copy(x));
// println!("{}",x);

//  }
//  fn take_owner_ship(some_strings:String){
//     println!("this functions takes ownerShip of the variable and prints out the following sting '{}' ",some_strings);

//  }
//  fn copy( mut x:i32)-> i32{
//     x += 78*45;
//     x+5

//  }

// fn main (){
//     let x: String = String::from("hello,world");
//     let y: String = x.clone();
//     println!("{},{}",x,y);

// }
// fn main(){
//     let s1: String =String::from("hello,world hjdbjhvjhsdgyjjas");
//     let s2: String = take_owner_ship(s1);
//     println!("{}",s2);
// }
// fn take_owner_ship(s:String)-> String{
//     s

// }
// fn  main (){
//     let s: Vec<u8> =give_ownership();
//  println!("{:?}",s);
// }
// fn give_ownership()->Vec<u8>{
//     let s:String = String::from("hellow world");
//     //convert String to vec
//     let _s: Vec<u8> =s.into_bytes();
//    _s
// }
// fn main (){
//     let s = give_owner_ship();
//     println!("{}",s);

// }
// fn give_owner_ship()-> String{
//     let s:String= String::from("hello world");
//     let _s: &[u8] = s.as_bytes();
//      s

// }
//  fn main (){
//     let x: (i32, i32, (), &str)= (1,2,(),"hello world");
//     let y: (i32, i32, (), &str)=x.clone();
//     println!("{:?},{:?}",x,y);

//  }
//mutabilty can be changed when ownership is transfered

// fn main (){
//    let s1:String=String::from ("hello,");
//  let mut s2:String=s1;
//  s2.push_str("world");
//  s2.push_str(" glen");
//  println!("{}",s2);

// }
//asigning values as to the heap memory directly
// fn main(){
//   let x:Box<i32>=Box::new(5);
//   let mut y:Box <i32>=Box::new(1);
//     *y=4;
//     assert!(*x==5);
//     println!("success {},{},{}",y,*x,*y-*x);

// }
// //ownership in struct
// fn main() {
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }
//     let person: Person = Person {
//         name: String::from("Glen Ayienda"),
//         age: Box::new(20),
//     };
//     let Person { name, ref age } = person;
//      println!("My name is {},I am {} years old",name,age);
//      println!("My name is {},I am {} years old",name,person.age);
// }
// fn main (){
//   let t:(String,String)=(String::from("hello"),String::from("world"));
//   let _s=t.0;
//   println!("{}",t.1);
// }
//Borrowing
// fn main (){
//   let s1: String="hello".to_string();
//   let length: usize=caluculate_length(&s1);
//   println!("the length of {} is {}",s1,length)
// }git commit -m Merge branch 'master' of https://github.com/glennin-codes/rust"
// fn caluculate_length(s:&String)->usize{
//   s.len()
// }
// //mutable reference 
// fn main (){
//   let mut s: String=String::from("hello");
//   change(  &mut s);
// println!("{}",s);
// }
// fn change(some_string:&mut String)->(){
//   some_string.push_str(",world")

// }
//dangling function reference 
// fn main (){
//   let reference= dangle();
//   println!("{}",reference);

// }
// fn dangle()-> &String {
//   let s1: String=String::from("hellow world");
//   &s1

// }
 //function no dangle
//  fn no_dangle()-> String{
//   let s1:String=String::from("hello  world");
//   s1
//  }
// fn main (){
//   let reference: String= no_dangle();
//   println!("{}",reference);

// }
//Reference
// 
// fn main (){
//     let mut s:String=String::from("hello");
//     s.push(',');
//     s.push_str(" world");
//     s += "!";
//     println!("{}",s);
// }// mod helper;

// use helper::starter::multiply;

// fn main() {
//     let result = multiply(5, 6);
//     println!("The result is: {}", result);
// }

//variable
// fn main (){
//     let x: i32 = 9;
//     let y: i32 = 5;
//     assert_eq!(x,y);
//     println!("success")
// }

//mut
// fn main (){
//     let mut x:i32 =5;
//     x+=5;
//     println!("x is {}",x);
// }
//unused variable
// #[allow(unused_variables)]
// fn main (){
//     let x:i32 =5;

// }
//types necessary for summations
// fn  main (){
//    assert_eq!( 0.1_f32+0.2_f32,0.3_f32);
//    println!("success")
// }
//range
// fn main (){
//    let mut  sum : i32 = 0;
//   for i in -3 .. 5{//this for loop does not inclued 5
//    sum += i;

//   }
//   assert!(sum==4);
//   println!(" the sum is {}",sum);

// }

//make println output 97-122
// fn main(){
//    for c in 'a' ..='z'{
//       println!("{}",c as u8 );
//    }

// }

//  fn main (){
//    let x:i32=5;
//    let y: i32={
//       let a:i32= 9 * x;
//       let b:i32 = 10 * x;
//       a + b
//    };
//    println!("{}",y)

//  }
//  // unit
//  fn main (){
//    // let unit:()=();
//    // assert_eq!(unit,());
//    // println!("success");
//    let v ={
//       let mut m:i32=1;
//         m+=1;
//         m
//    };
//    assert_eq!(v,2_i32);
//    println!("success")

//  }

// fn main (){
//    let (x,y)=(5,8);
//    let s=sum(x,y);
//    println!("sum is {}",s)
// }
// fn sum(x:i32,y:i32)-> i32{
//    x+y

// }
// fn main (){
//     let s1=String :: from ("hello");
//     let s3:String="hello world ".to_string();
//     let s2=s1;//after reasinging the string s1 to s2 ,si pointer is copied to the heap ,but its values are droped since they can only be one owner at a time
//     println!("{}",s2);
//     println!("{}",s3);

// }
//  fn main (){
//     let s1:String="my name is king paul".to_string();
//  take_owner_ship(s1);
// //  println!("{}",s1); this code wont work cause s1 onwership was moved to the function take_owner_ship  therefore in this main scope s1 ios droped
// let x=9+3;
// println!("answer is {}",copy(x));
// println!("{}",x);

//  }
//  fn take_owner_ship(some_strings:String){
//     println!("this functions takes ownerShip of the variable and prints out the following sting '{}' ",some_strings);

//  }
//  fn copy( mut x:i32)-> i32{
//     x += 78*45;
//     x+5

//  }

// fn main (){
//     let x: String = String::from("hello,world");
//     let y: String = x.clone();
//     println!("{},{}",x,y);

// }
// fn main(){
//     let s1: String =String::from("hello,world hjdbjhvjhsdgyjjas");
//     let s2: String = take_owner_ship(s1);
//     println!("{}",s2);
// }
// fn take_owner_ship(s:String)-> String{
//     s

// }
// fn  main (){
//     let s: Vec<u8> =give_ownership();
//  println!("{:?}",s);
// }
// fn give_ownership()->Vec<u8>{
//     let s:String = String::from("hellow world");
//     //convert String to vec
//     let _s: Vec<u8> =s.into_bytes();
//    _s
// }
// fn main (){
//     let s = give_owner_ship();
//     println!("{}",s);

// }
// fn give_owner_ship()-> String{
//     let s:String= String::from("hello world");
//     let _s: &[u8] = s.as_bytes();
//      s

// }
//  fn main (){
//     let x: (i32, i32, (), &str)= (1,2,(),"hello world");
//     let y: (i32, i32, (), &str)=x.clone();
//     println!("{:?},{:?}",x,y);

//  }
//mutabilty can be changed when ownership is transfered

// fn main (){
//    let s1:String=String::from ("hello,");
//  let mut s2:String=s1;
//  s2.push_str("world");
//  s2.push_str(" glen");
//  println!("{}",s2);

// }
//asigning values as to the heap memory directly
// fn main(){
//   let x:Box<i32>=Box::new(5);
//   let mut y:Box <i32>=Box::new(1);
//     *y=4;
//     assert!(*x==5);
//     println!("success {},{},{}",y,*x,*y-*x);

// }
// //ownership in struct
// fn main() {
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }
//     let person: Person = Person {
//         name: String::from("Glen Ayienda"),
//         age: Box::new(20),
//     };
//     let Person { name, ref age } = person;
//      println!("My name is {},I am {} years old",name,age);
//      println!("My name is {},I am {} years old",name,person.age);
// }
// fn main (){
//   let t:(String,String)=(String::from("hello"),String::from("world"));
//   let _s=t.0;
//   println!("{}",t.1);
// }
//Borrowing
// fn main (){
//   let s1: String="hello".to_string();
//   let length: usize=caluculate_length(&s1);
//   println!("the length of {} is {}",s1,length)
// }
// fn caluculate_length(s:&String)->usize{
//   s.len()
// }
// //mutable reference
// fn main (){
//   let mut s: String=String::from("hello");
//   change(  &mut s);
// println!("{}",s);
// }
// fn change(some_string:&mut String)->(){
//   some_string.push_str(",world")

// }
//dangling function reference
// fn main (){
//   let reference= dangle();
//   println!("{}",reference);

// }
// fn dangle()-> &String {
//   let s1: String=String::from("hellow world");
//   &s1

// }
//function no dangle
//  fn no_dangle()-> String{
//   let s1:String=String::from("hello  world");
//   s1
//  }
// fn main (){
//   let reference: String= no_dangle();
//   println!("{}",reference);

// }
//Reference
// //
// fn main (){
//     let mut s:String=String::from("hello");
//     s.push(',');
//     s.push_str(" world");
//     s += "!";
//     println!("{}",s);
// }
// fn main(){
//     let mut v: Vec<i32> = vec![10,20,30];
//     v.push(80);
//     v.push(90);
//     println!("{:?}",v);
//     println!("{}",v[0]);

// }
// fn main (){
//     let v: Vec<i32> = vec![1; 3];
// assert_eq!(v, [1, 1, 1]);

// }
// fn main (){
//     let  s: String=String::from("I like dogs");
//     let s1: String=s.replace("dogs","cats");
//     assert_eq!(s1,"I like cats");
//     println!("success");
// }
// extern crate chrono;

// use chrono::DateTime;

// fn main() {
//     let timestamp = "2023-11-06T06:52:56.773Z";
//     match DateTime::parse_from_rfc3339(timestamp) {
//         Ok(parsed_date) => {
//             println!("Parsed timestamp: {}", parsed_date);
//         },
//         Err(e) => {
//             eprintln!("Error parsing timestamp: {}", e);
//         }
//     }
// }

//concatination with a string
// fn main (){
//     let s1:String=String::from("hello,");
//     let s2:String=String::from("world");
//     let s3: String=s1+&s2;
//     assert_eq!(s3,"hello,world");
//     println!("{}",s3);
// }
// //converting &str to a String
// fn main(){
//     let s:&str="hello,world";
//     // greetings(s.to_string())
//     // or
//     greetings(String::from(s))

// }
// fn greetings(s:String){
//     println!("{}",s)
// }

// fn main (){
//     let s:String="hello, world".to_string();
//     let _s1:&str=&s;
//     println!("Success!");
// }
// fn main (){
//     for c in "my love for rust is out standing".chars(){
//         println!("{}",c);
//     }
// }
//arrays
// fn main(){
//     let arr: [i32; 5]=[2,3,4,5,6];
//     assert!(arr.len()==5);
//     println!("Success");
// }\
// fn main (){
//     let arr: [char; 7] =['a','b','c','d','f','g','h'];
//     println!("{}",std::mem::size_of_val(&arr));
//    assert_eq!(std::mem::size_of_val(&arr),28);
//    print!("success");
// }
// fn main (){
//     let arr:[char;7]=['a','b','c','d','f','g','h'];
//     let slice:&[char]=&arr[1..2];
//     println!("{:?}",slice);

// }
// fn main() {
//     let arr: [&str; 12] = [
//         "Terminal", "will", "be", "reused", "by", "tasks", "press", " any", "key", "to", "close",
//         "it. "
//     ];
//     println!("{}",std::mem::size_of_val(&arr));
//     println!("{}",std::mem::size_of_val(&arr)/arr.len());
//     let s: &str="hello,world hjhsdb";
//     println!("{}",std::mem::size_of_val(&s));
// }
// fn main(){
//     let arr=['a','b','c','d','f','g','h'];
//     let slice:&[char] =&arr[..2];
//     println!("{}",std::mem::size_of_val(&slice));
//     println!("{:?}",slice.len());

// }
// fn main (){
//     let arr :[i32;5]=[1,2,3,4,5];
//     let slice:&[i32] = &arr[1..4];
//     assert_eq!(slice,&[2,3,4]);
//     println!("success :{:?}",slice)
// }println!("{}",slice1);
// fn main (){
//     let s =String::from("hello");
//     let slice1=&s[0..2];
//     println!("{}",slice1);
//     let slice2=&s[..2];
//     assert_eq!(slice1,slice2);
//     println!("Success!");

// }
// fn main (){
//     let mut s:String=String::from("hello,world");
//     let word=first_word(&s);
//     println!("the first word is:{}",word);
//     s.clear();

// }
// fn first_word(s:&str)->&str{
//     &s[..1]
// }

//Structs
// #[allow(dead_code)]
// #[allow(unused_variables)]
// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8,
//     hobby: String,
// }
// fn main() {
//     let user: Person = Person {
//         name: String::from("Glen Ayienda"),
//         age: 35,
//         hobby: String::from("coding and playing video games")
//     };

//     println!(
//         "name:{}\n age:{}\n hobby:{}\n",
//         user.name, user.age, user.hobby
//     );
// }
// #[allow(unused_variables)]
// fn main(){

//     let age:u8=30;
//     let p: Person=Person{
//         name:"Alice johnson".to_string(),
//         age,
//         hobby:"dancing".to_string()
//     };
//     println!("Success!")
// }

// #[allow(dead_code)]
// #[allow(unused_variables)]
// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8,

// }
// fn main(){
//  let age: u8=32;
//  let mut p =Person{
//     name:String::from("glen ayienda"),
//     age
//  };
//  p.name=String::from("jmaes tyson");
//  p.age=43;
//  let Person{name,age}=p;

//  println!("name:{}\n age:{}",name,age);

//  println!("name:{}\n age:{}",name,age);
// }

//struct for tuple
// #[allow(dead_code)]
// #[derive(Debug)]
// struct Color(i32,i32,i32);
// fn main(){
//     let background_color:Color=Color(3,9,02);
//     check_color(background_color);
//     println!("Success");

// }
// fn check_color(c:Color){
//     let Color(x,y,z)=c;

//     assert_eq!(y,9);
//     assert_eq!(z,02);
//     assert_eq!(x,3);
//     println!("{}",c.0);

// }
// #[allow(dead_code)]
// #[derive(Debug)]
// struct Person{
//     name:String,
//     age:u8
// }
// fn main(){
//     let p:Person=helper(String::from("James Otiso"),32);
//     println!("{:?}",p);
//     println!("Success!");

// }
// fn helper(name:String,age:u8)->Person{
//     let p:Person=Person{
//         name,
//         age
//     };
//     p

// }
//using field init shorthand syntax to reduce repetions
// #[derive(Debug)]
// #[allow(dead_code)]
// struct User {
//     username: String,
//     active: bool,
//     sign_in_count: u64,
//     email: String,
// }
// fn main() {
//     let user1: User = User {
//         username: String::from("james Tyson"),
//         email: String::from("jamesTyson@gmail.com"),
//         active: true,
//         sign_in_count: 1,
//     };
//     println!("user1={:?}\n",user1);
//     let user2: User=set_email(user1);
   
//     println!("user2={:?}\n",user2);
//     println!("success!");
// }
// fn set_email(u:User ) ->User {
//     User {
//         email: String::from("homes@gmail.com"),
//         ..u
//     }
// }
//enums
//way of defining a type with only one possible set of value

// enum IpAddres{
//     V4(String),
//     V6(String)
// }

// fn main(){
//     let home:IpAddres=IpAddres::V4(String::from("127.0.0.1"));
//     let loopback:IpAddres=IpAddres::V6(String::from("::1"));
//     println!("home={:?}\n loopback={:?}",home,loopback);


// }
//an enum can hold different dataTypes
// #[derive(Debug)]
// #[allow(dead_code)]
// enum Message{
//     Quit,
//     Move{x:i32,y:i32},
//     Write(String),
//     ChangeColor(i32,i32,u8)
// }
// fn main(){
//     let m1:Message=Message::ChangeColor(23, 20, 3);
//     let m3:Message=Message::Move { x: 3, y: 3 };
//     let message:Message=Message::Write("I love My work".to_string());
//     println!(
//         "
//         {:?}\n
//         {:?}\n
//         {:?}\n
//         ",m1,m3,message
//     );
// }
#[derive(Debug)]
#[allow(dead_code)]

enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,u8)
}
// }
// fn main(){
//     let msg:Message=Message::Move { x: 5, y: 5 };
//     if let Message::Move { x, y }=msg{
//         assert_eq!(x,y)
//     }else{
//         panic!("not a move command");
//     }
//     println!("Success!")
// }
// fn main(){
//     let msgs:[Message;3]=[
//         Message::Move { x: 3, y: 3 },
//         Message::ChangeColor(223, 220, 0),
//         Message::Quit
//     ];
//     for msg in msgs{
//         show_message(msg);
//     }
// }
// fn show_message(msg:Message){
//     println!("{:?}",msg);
// }

//if let
// fn main(){
//     let favourite_color:Option<&str>=Some("red");

// if let Some(color)=favourite_color{
//     println!("favourite color is {}",color);
// }else {
//     println!("I don't have a favorite color");
// }

// }


   
         
                fn parse_input(input: &str) -> Result<i32, std::num::ParseIntError> {
                    match input {
                        "add" => Ok(1),
                        "subtract" => Ok(2),
                        "multiply" => Ok(3),
                        "divide" => Ok(4),
                        _ => panic!("Invalid input"),
                    }
                }
            
   

fn main() {
    let input = "divide";
    let operation = parse_input(input).unwrap();
    println!("Operation: {}", operation);
}