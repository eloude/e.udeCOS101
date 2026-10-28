struct Employee {
    ceo:String,
    company:String,
    age:u32,
}


fn main() {
    //initialize a structure
    let emp1 = Employee {
        ceo:String::from("Satya Nadella"),
        company:String::from("Microsoft Corporation"),
        age:56
    };

    let emp2 = Employee {
        ceo:String::from("Sundai Pichai"),
        company:String::from("Google Inc."),
        age:51
    };
    //pass emp1 and emp2 to display()
    display(emp1);
    display(emp2);
}

//fetch values of specific structure fields using the 
//operator and print it to the console
fn display(emp:Employee) {
    println!("Name is : {} company is {} age is {}", emp.ceo,emp.company,emp.age);
}
