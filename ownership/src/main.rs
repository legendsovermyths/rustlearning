fn reverse_string(s: &str) {
   let bytes = s.as_bytes();

    for (_i, &item) in bytes.iter().enumerate() {
       print!("{}", item.to_string()); 
    } 
}

fn main() {
    let a = "a";
    reverse_string(a);

    for i in 1..100{
        print!("{}",i);
    }
}
