struct Laptop {
    brand:String,
    price:u32
}

impl Laptop {
    fn i_total(&self)->u32 {
    println!("\nHow many do you want to buy");
    let qty:u32 = input().parse().expect("Invalid input");
    qty * self.price
    }
}

fn main() {
    let mut total = 0;

'outer: loop {

    let pc_1 = Laptop {
        brand:String::from("HP"),
        price:650000
    };

    let pc_2 = Laptop {
        brand:String::from("IBM"),
        price:755000
    };

    let pc_3 = Laptop {
        brand:String::from("Toshiba"),
        price:550000
    };

    let pc_4 = Laptop {
        brand:String::from("Dell"),
        price:850000
    };

    println!("\nWhat brand of laptop do you want to buy:");
    println!("HP\nIBM\nToshiba\nDell\n");

    let i_total = loop {
    let laptop = input().to_uppercase();
    match laptop.as_str() {
        "1" | "HP" => {
            println!("\nLaptop: {}\nPrice: {}",pc_1.brand,pc_1.price);
            break pc_1.i_total();
        }
        "2" | "IBM" => { 
            println!("\nLaptop: {}\nPrice: {}",pc_2.brand,pc_2.price);
            break pc_2.i_total();
        }
        "3" | "TOSHIBA" => {
            println!("\nLaptop: {}\nPrice: {}",pc_3.brand,pc_3.price);
            break pc_3.i_total();
        }
        "4" | "DELL" => {
            println!("\nLaptop: {}\nPrice: {}",pc_4.brand,pc_4.price);
            break pc_4.i_total();
        }
        _ => {
            println!("\nWe do not have that laptop in stock right now");
        }
    }
  };
 total += i_total;

 println!("\nDo you want to buy any other laptop");
 loop {
 let repeat = input().to_uppercase();
 match repeat.as_str() {
    "YES" | "Y" => break,
    "NO" | "N" => break 'outer,
    _ => {
        println!("\nInput either yes or no");
    }
  }
 }
}
  println!("\nYour total is: N{}", total);
}

fn input()->String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Invalid input");
    let input = input.trim();
    input.to_string()
}
