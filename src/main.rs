fn main() {
    //unsigned integer
    let unsigned: u8 = 100;

    //signed integer
    let signed: i8 = -15;

    //float
    let float: f32 =1.2;

    println!("unsign: {} sign: {} float {}", unsigned,signed, float);    

  //  println!("Hello, world!,{} {}",a,b);

  // char- can only be
  let letter ="c1232";
  let emoji = "\u{1F600}"; // :D

  println!("letter: {}, emoji: {}", letter , emoji);

  //booléen déclaration
  let is_true: bool =true;

  println!("isTrue: {}", is_true);

  //array declaration
  let arr: [u8;3] = [1,2,3];
  let other_arr: [u8; 5] = [100;5];

  println!("index: {}, length: {}", arr[0],other_arr.len());

  //print structure of arrray and other objects
  println!("{:?}", other_arr);
}
