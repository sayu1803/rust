//fn main(){
//    let s1:String=get_string();
//    println!("this is s1:{}",s1);
//
//    let s2:String=String::from("world");
//    let s3:String=send_get_string(s2);
//
//    println!("This is s3;{}",s3)
//}
//
//fn get_string()->String{
//    let new_string=String::from("hello");
//    return new_string;
//}
//
//fn send_get_string(received_string:String)->String{
//    return received_string;
//}



//array
//fn main(){
//    let mut arr1;
//    arr1=[1,2,3,4,5,6];
//    println!("arr1[0]={}",arr1[0]);
//    println!("array length is {}",arr1.len())
//}

//Vector
fn main(){
    let mut v:Vec<i32>=Vec::new();
    let mut c= Vec::<i32>::new();

    let d=vec![1,2,3,4,5,6,7,8,9];

    v.push(1);
    v.push(2);
    c.push(3);

    println!("elements in Vector v is {:?}",d);
    println!("elements in Vector v is {:?}",v);
    println!("elements in Vector v is {:?}",c);
}