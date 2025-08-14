// CLI entry point
//

mod file_manager;

fn main() {
    let mut page1 = file_manager::page::Build_page(16384);
    let src1:String = String::from("MIBOMBOCLAT");
    let bites: Vec<u8> = src1.as_bytes().to_vec();
    page1.Write(5, bites);
    let mut bites2: Vec<u8> = vec![0; 11];
    page1.Read(5, &mut bites2);

    println!("{:?}",bites2);


}
