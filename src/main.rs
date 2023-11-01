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
fn main (){
    let s1=String :: from ("hello");
    let s3:String="hello world ".to_string();
    let s2=s1;//after reasinging the string s1 to s2 ,si pointer is copied to the heap ,but its values are droped since they can only be one owner at a time 
    println!("{}",s2);
    println!("{}",s3);
  
}
