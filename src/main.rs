// CLI entry point
//

mod file_manager;
use crate::file_manager::file_manager::build_file_manager;
use crate::file_manager::page::build_page;
use crate::file_manager::block::Block_ID;

mod buffer_pool;
use crate::buffer_pool::page_table::Page_table;


fn main() {
    let mut file_manager = build_file_manager(8192, String::from("/home/maxim/Rust/databaseProject/files") );
    let file_name = String::from("Testing12345");
    let mut myFile = file_manager.get_file(&file_name);


    let mut page = build_page(8192);
    let page_content = String::from("hiMyNAme is bomborasclat man").as_bytes().to_vec();
    page.write(69, page_content);


    let block = Block_ID{file_name: String::from("Testing12345"), number: 100};
    file_manager.write(&block, &page);


    let mut page2 = build_page(8192);
    file_manager.read(&block, &mut page2);

    let mut page_table = Page_table::new(50000, 8192);

    page_table.request_new_page(block, &mut file_manager);

    let block  = Block_ID{file_name: String::from("Testing12345"), number: 100};
    page_table.request_new_page(block, &mut file_manager);

    let block  = Block_ID{file_name: String::from("Testing12345"), number: 100};

    page_table.request_new_page(block, &mut file_manager);

    println!("{:?}", page_table.pages_in_memory);

}
