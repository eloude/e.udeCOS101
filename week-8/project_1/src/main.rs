use std::io;

fn main() {
    let user = get_name();
    let aps = get_profession();
    println!("\nDear {}, your APS level is {}", user, aps);
}

fn input()-> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    input.trim().to_string()
}

fn get_name() ->String{
    println!("\nEnter your First Name");
    let f_name = input();


    println!("\nWhat is your Gender? (M/F)");

    let title = loop {

    let g = input().to_uppercase();
    match g.as_str() {
        "M" | "MALE" => break "Mr",
        "F" | "FEMALE" => break "Mrs",
        _ => {println!("\nINVALID INPUT");
                println!("Try again");
            }
        }
    }; 

   format!("{} {}", title, f_name)
}

fn get_profession() -> String{
    println!("\nChoose your profession:
\n1 Office Admin
2 Academic
3 Lawyer
4 Teacher");

loop {
    let profession = input().to_lowercase();
    match profession.as_str() {
        "1" | "office admin" =>break office(),
        "2" | "academic" => break academic(),
        "3" | "lawyer" => break lawyer(),
        "4" | "teacher" => break teacher(),
        _ => {
            println!("\nINVALID INPUT");
                println!("Try again");
             }
    }
  }
}


fn aps() ->Vec<String> {
    let x = vec!["APS 1 - 2".to_string(),"APS 3 - 5".to_string(),"APS 5 - 8".to_string(),"EL1 8 - 10".to_string(),"EL2 10 - 13".to_string(),"SES".to_string()];
    x
}


fn position() -> String {
    println!("\nIf you are unsure input 'none'");
    let position = input();
    position
}

fn office () -> String {
    println!("\nChoose your position:");
    println!("1 Intern");
    println!("2 Administrator");
    println!("3 Senior Administrator");
    println!("4 Office Manager");
    println!("5 Director");
    println!("6 CEO");

    let a = aps();
    loop {

    let p_office = position().to_lowercase();
    
    match p_office.as_str() {
        "1" | "intern" => break a[0].clone(),
        "2" | "administrator" => break a[1].clone(),
        "3" | "senior administrator" => break a[2].clone(),
        "4" | "office manager" => break a[3].clone(),
        "5" | "director" => break a[4].clone(),
        "6" | "ceo" => break a[5].clone(),
        "none" => break years(),
        _ => {
            println!("\nINVALID INPUT");
                println!("Try again");
             }
    }
  }
}


fn academic() -> String {

    println!("\nChoose your position:");
    println!("1 Research Assistant");
    println!("2 PhD Candidate");
    println!("3 Post-Doc Researcher");
    println!("4 Senior Lecturer");
    println!("5 Dean");
    let a = aps();

    loop {
    let p_academic = position().to_lowercase();
    match p_academic.as_str() {
        "1" | "research assistant" => break a[1].clone(),
        "2" | "phd candidate" => break a[2].clone(),
        "3" | "post-doc researcher" => break a[3].clone(),
        "4" | "senior lecturer" => break a[4].clone(),
        "5" | "dean" => break a[5].clone(),
        "none" => break years(),
        _ => {
            println!("\nINVALID INPUT");
                println!("Try again");
             }
    }
  }
}

fn lawyer() -> String {
let a = aps();

        println!("\nChoose your position:");
        println!("1. Paralegal");
        println!("2. Junior Associate");
        println!("3. Associate");
        println!("4. Senior Associate 1");
        println!("5. Senior Associate 2");
        println!("6. Senior Associate 3");
        println!("7. Senior Associate 4");
        println!("8. Partner");

        loop {
        match position().as_str() {
            "1" | "paralegal" => break a[0].clone(),
            "2" | "junior associate" => break a[1].clone(),
            "3" | "associate" => break a[2].clone(),
            "4" | "senior associate 1" => break a[3].clone(),
            "5" | "senior associate 2" => break a[3].clone(),
            "6" | "senior associate 3" => break a[4].clone(),
            "7" | "senior associate 4" => break a[4].clone(),
            "8" | "partner" => break a[5].clone(),
            "none" => break years(),
            _ => {
            println!("\nINVALID INPUT");
                println!("Try again");
             }

        }
    }
}

fn teacher() -> String {
    println!("\nChoose your position:");
    println!("1 Placement");
    println!("2 Classroom Teacher");
    println!("3 Senior Teacher");
    println!("4 Leading Teacher");
    println!("5 Deputy Principal");
    println!("6 Principal");

let a = aps();
    loop {
        let p_teacher = position().to_lowercase();
        
        match p_teacher.as_str() {
            "1" | "placement" => break a[0].clone(),
            "2" | "classroom teacher" => break a[1].clone(),
            "3" | "senior teacher" => break a[2].clone(),
            "4" | "leading teacher" => break a[3].clone(),
            "5" | "deputy principal" => break a[4].clone(),
            "6" | "principal" => break a[5].clone(),
            "none" => break years(),
            _ => {
            println!("\nINVALID INPUT");
                println!("Try again");
             }
        }
    }

}


fn years() -> String {
    println!("\nHow many years of experience do you have");
let a = aps();
    loop {
        
    let yrs = input();
    
    let yrs: i16 = yrs.parse().expect("Invalid input");

    match yrs {
        1..=2 =>break a[0].clone(),
        3..=4 =>break a[1].clone(),
        5..=7 =>break a[2].clone(),
        8..=9 =>break a[3].clone(),
        10..=13 =>break a[4].clone(),
        14..200 =>break a[5].clone(),
        _ => {
            println!("\nPlease enter the actual number of years of experience");
        }
    }
  }
}