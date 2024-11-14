// use block_block::Block;

// fn main(){

//     let mut b1 = Block::from(|p: &mut String| {
//                             p.push_str("b1");
//                             println!("{}", p);
//                         });
//     b1.name = Some("b1".to_string());

//     let mut b2 = Block::from(|p: &mut String| {
//                             p.push_str("b2");
//                             println!("{}", p);
//                         });
//     b2.name = Some("b2".to_string());

//     let mut b3 = Block::from(|p: &mut String| {
//                             p.push_str("b3");
//                             println!("{}", p);
//                         });
//     b3.name = Some("b3".to_string());

//     b2.connect(b3);

//     b1.connect(b2);


//     let mut data = String::from("aaa");

//     // b1.exec(&mut data);
//     // println!("1-> {}", data);



//     // println!("again");
//     // b1.connect_with_fun(|p: &mut String |{
//     //                     p.push_str("b4");
//     //                     println!("{}", p)
//     //                 });
//     b1.exec_by_iter(&mut data);
//     println!("2-> {}", data);
// }
