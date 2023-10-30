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
fn main(){
   for c in 'a' ..='z'{
      println!("{}",c as u8);
   }

}