#[derive(Debug)]
enum Electronics {
    Tablet(String),
    Phone(String),
    Keyboard { brand: String, layout: i32 },
}

fn main() {
    let a = Electronics::Phone(String::from("iPhone 15"));
    match a {
        Electronics::Phone(name) => println!("{}", name),
        _ => (),
    }
    let b = Electronics::Keyboard {
        brand: String::from("Royal Kludge"),
        layout: 65,
    };
    println!("{:?}", b);
    if let Electronics::Keyboard { brand, layout } = &b {
        println!("{}, {}", brand, layout);
    }
    let c = Some(Electronics::Tablet(String::from("iPad Air 5")));
    match &c {
        Some(varible) => {
            println!("{:?}", varible);
        }
        None => {
            println!("None Type");
        }
    }

    let d = Electronics::Tablet(String::from("iPad Air 5"));

    println!("{:?}", d);
}