// CLI entry point
//

mod file_manager;
use crate::file_manager::file_manager::build_file_manager;
use crate::file_manager::page::build_page;
use crate::file_manager::block::Block_ID;

mod buffer_pool;
use crate::buffer_pool::page_table::Page_table;


fn main() {
    test_page_replacement();

}


fn test_new_page_table() {
    let pt = Page_table::new(1024, 256);
    assert_eq!(pt.pages_in_memory.len(), 4);

}

fn test_request_new_page_inserts_page() {
    let mut pt = Page_table::new(512, 128);
    let mut fm = build_file_manager(128, "./files".to_string());

    let block = Block_ID{file_name: "Testing12345".to_string(), number: 42};
    let res = pt.request_new_page(block.clone(), &mut fm);
    assert!(res.is_ok());
    
    println!("{:?}",pt);
}

fn test_page_replacement() {
    let mut pt = Page_table::new(1000000,16384);
    let mut fm = build_file_manager(16384, "./files".to_string());


    for i in 1..6000{
        let b = Block_ID{file_name: "Testing12345".to_string(), number: i};
        let res = pt.request_new_page(b, &mut fm);
    }

        
    //println!("{:?}",pt);
    println!("{:?}",pt.pages_in_memory.len());


}

