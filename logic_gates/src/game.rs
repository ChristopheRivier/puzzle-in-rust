use std::collections::HashMap;
enum Operator{
    And,
    Or,
    XOr,
    NAnd,
    NXOr,
    NOr
}

pub struct ElementFormule{
    pub name:String,
    operator:Operator,
    first:String,
    second:String
}

pub fn init_formule(name: String, op:String, f: String, s: String) ->ElementFormule{
    ElementFormule{
        name: name,
        operator:match op.as_str(){
            "AND" => Operator::And,
            "OR" => Operator::Or,
            "XOR"=> Operator::XOr,
            "NAND"=> Operator::NAnd,
            "NXOR"=> Operator::NXOr,
            "NOR" => Operator::NOr,
            &_ => todo!()
        },
        first:f,
        second:s
    }
}

pub fn calcul_formule(formule: &ElementFormule, element : &HashMap<String,Vec<bool>>, s: usize)->String{
let mut ret: String = String::from("");
match formule.operator {
    Operator::And => {
        for i in 0..s as usize{
            if element.get(&formule.first).unwrap()[i] && element.get(&formule.second).unwrap()[i] {
                ret.push('-');
            }else {
                ret.push('_');
            }
        }
    }
    Operator::Or => {
        for i in 0..s as usize{
            if element.get(&formule.first).unwrap()[i] || element.get(&formule.second).unwrap()[i] {
                ret.push('-');
            }else {
                ret.push('_');
            }
        }
    } 
    Operator::NOr => {
        for i in 0..s as usize{
            if !element.get(&formule.first).unwrap()[i] && !element.get(&formule.second).unwrap()[i] {
                ret.push('-');
            }else {
                ret.push('_');
            }
        }
    }
    Operator::NAnd => {
        for i in 0..s as usize{
            if !(element.get(&formule.first).unwrap()[i] && element.get(&formule.second).unwrap()[i]) {
                ret.push('-');
            }else {
                ret.push('_');
            }
        }
    }
    Operator::NXOr => {
        for i in 0..s as usize{
            if !(element.get(&formule.first).unwrap()[i] ^ element.get(&formule.second).unwrap()[i]) {
                ret.push('-');
            }else {
                ret.push('_');
            }
        }
    }
    Operator::XOr => {
        for i in 0..s as usize{
            if element.get(&formule.first).unwrap()[i] ^ element.get(&formule.second).unwrap()[i] {
                ret.push('-');
            }else {
                ret.push('_');
            }
        }
    }
}
ret
} 
pub fn transform_string(signal: String)-> Vec<bool>{
    // attention il faut penser à la portabilité de la variable.
    let mut ret:Vec<bool> = Vec::new();
    for s in signal.chars(){
        ret.push(s=='-');
    }
    ret
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let mut c = Vec::new();
        c.push(true);
        let a = transform_string("-".to_string());
        assert_eq!(a.len(),c.len());
        assert_eq!(a[0],c[0])
    }

    #[test]
    fn test_deux() {
        let mut c = Vec::new();
        c.push(true);
        c.push(false);
        let a = transform_string("-_".to_string());
        assert_eq!(a.len(),c.len());
        assert_eq!(a,c);
    }

    #[test]
    fn test_and(){
        let mut map_signal = HashMap::new();
        map_signal.insert("A".to_string(), transform_string("-_".to_string()));
        map_signal.insert("B".to_string(), transform_string("--".to_string()));
        let ee =ElementFormule{name:"C".to_string(),
        operator:Operator::And,
        first:"A".to_string(),
        second:"B".to_string()};
        let a = calcul_formule(&ee, &map_signal, 2);
        assert_eq!("-_",a);
    }
    #[test]
    fn test_nand(){
        let mut map_signal = HashMap::new();
        map_signal.insert("A".to_string(), transform_string("-_".to_string()));
        map_signal.insert("B".to_string(), transform_string("--".to_string()));
        let ee =ElementFormule{name:"C".to_string(),
        operator:Operator::NAnd,
        first:"A".to_string(),
        second:"B".to_string()};
        let a = calcul_formule(&ee, &map_signal, 2);
        assert_eq!("_-",a);
    }

    #[test]
    fn test_or(){
        let mut map_signal = HashMap::new();
        map_signal.insert("A".to_string(), transform_string("-_".to_string()));
        map_signal.insert("B".to_string(), transform_string("--".to_string()));
        let ee =ElementFormule{name:"C".to_string(),
        operator:Operator::Or,
        first:"A".to_string(),
        second:"B".to_string()};
        let a = calcul_formule(&ee, &map_signal, 2);
        assert_eq!("--",a);
    }
    #[test]
    fn test_nor(){
        let mut map_signal = HashMap::new();
        map_signal.insert("A".to_string(), transform_string("-__".to_string()));
        map_signal.insert("B".to_string(), transform_string("--_".to_string()));
        let ee =ElementFormule{name:"C".to_string(),
        operator:Operator::NOr,
        first:"A".to_string(),
        second:"B".to_string()};
        let a = calcul_formule(&ee, &map_signal, 3);
        assert_eq!("__-",a);
    }
    #[test]
    fn test_nxor(){
        let mut map_signal = HashMap::new();
        map_signal.insert("A".to_string(), transform_string("-__".to_string()));
        map_signal.insert("B".to_string(), transform_string("--_".to_string()));
        let ee =ElementFormule{name:"C".to_string(),
        operator:Operator::NXOr,
        first:"A".to_string(),
        second:"B".to_string()};
        let a = calcul_formule(&ee, &map_signal, 3);
        assert_eq!("-_-",a);
    }
    #[test]
    fn test_xor(){
        let mut map_signal = HashMap::new();
        map_signal.insert("A".to_string(), transform_string("-__".to_string()));
        map_signal.insert("B".to_string(), transform_string("--_".to_string()));
        let ee =ElementFormule{name:"C".to_string(),
        operator:Operator::XOr,
        first:"A".to_string(),
        second:"B".to_string()};
        let a = calcul_formule(&ee, &map_signal, 3);
        assert_eq!("_-_",a);
    }
}
