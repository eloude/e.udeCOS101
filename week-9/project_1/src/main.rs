use std::io::Write;

fn main() {
    let mut drinks_ctgr = std::fs::File::create("nb_drinks.csv").expect("could not create file");
    drinks_ctgr.write_all(",DRINKS,".as_bytes()).expect("could not write");
    drinks_ctgr.write_all("\n\nLAGER,STOUT,NON-ALCOHOLIC".as_bytes()).expect("could not write");
    drinks_ctgr.write_all("\n33 Export,Legend,Maltina".as_bytes()).expect("could not write");
    drinks_ctgr.write_all("\nDesperados,Turbo King,Amstel Malta".as_bytes()).expect("could not write");
    drinks_ctgr.write_all("\nGoldberg,Williams,Malta Gold".as_bytes()).expect("could not write");
    drinks_ctgr.write_all("\nGulder,,Fayrouz".as_bytes()).expect("could not write");
    drinks_ctgr.write_all("\nHeineken,,".as_bytes()).expect("could not write");
    drinks_ctgr.write_all("\nStar,,".as_bytes()).expect("could not write");
}
