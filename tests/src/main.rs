use std::collections::HashMap;
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut mp: HashMap<Vec<char>, Vec<String>> = HashMap::new();
    for st in strs {
        let mut a:Vec<char> = st.clone().chars().collect();
        a.sort();
        let vs:&mut Vec<String> = mp.entry(a).or_insert(vec![]);
        vs.push(st);
    }
    let mut res:Vec<Vec<String>> = vec![];
    for (_key, value) in mp{
        res.push(value);
    }
    
    return res;
}
fn checker(temp_vec:Vec<i32>)->Vec<i32>{
    temp_vec
}
fn main() {
    let b = vec![1,2,3];
    let c = checker(b);
    print!("{:?}",b);
    println!("Hello, world!");
}
