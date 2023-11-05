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
 fn main(){
  let x: i32=5;
  let p: &i32=&x;
  println!("the memory of x is {:p}",p)
 }