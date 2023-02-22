// Logic gates codingame
/*
A contribution by b0n5a1
Approved by Zorg1 , Remi. and Westicles
 */


 use std::io;
 use std::collections::HashMap;
 mod game;
 use crate::game::init_formule;
 use crate::game::transform_string;
 use crate::game::calcul_formule;
 use crate::game::ElementFormule;

 macro_rules! parse_input {
     ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
 }
 
 /**
  * Auto-generated code below aims at helping you parse
  * the standard input according to the problem statement.
  **/
 fn main() {
     let mut input_line = String::new();
     io::stdin().read_line(&mut input_line).unwrap();
     let n = parse_input!(input_line, i32);
     let mut input_line = String::new();
     io::stdin().read_line(&mut input_line).unwrap();
     let m = parse_input!(input_line, i32);
     //create liste of element
     let mut map_signal = HashMap::new();
     let mut size=0;
     for _i in 0..n as usize {
         let mut input_line = String::new();
         io::stdin().read_line(&mut input_line).unwrap();
         let inputs = input_line.split(" ").collect::<Vec<_>>();
         let input_name = inputs[0].trim().to_string();
         let input_signal = inputs[1].trim().to_string();
         size = input_signal.len();
         map_signal.insert(input_name, transform_string(input_signal));
     }
     let mut tab_formule:Vec<ElementFormule> = Vec::new();
     for _i in 0..m as usize {
         let mut input_line = String::new();
         io::stdin().read_line(&mut input_line).unwrap();
         let inputs = input_line.split(" ").collect::<Vec<_>>();
         let output_name = inputs[0].trim().to_string();
         let _type = inputs[1].trim().to_string();
         let input_name_1 = inputs[2].trim().to_string();
         let input_name_2 = inputs[3].trim().to_string();
         tab_formule.push(init_formule(output_name,_type, input_name_1, input_name_2));
         println!("{} {}",tab_formule[_i].name,calcul_formule(&tab_formule[_i],&map_signal,size));
     }
 }
