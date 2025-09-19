// CLI entry point
//



///BUG:
///
///Fix not updating the last name.....
mod file_manager;
use crate::file_manager::file_manager::File_manager;
use crate::file_manager::file_manager::build_file_manager;
use crate::file_manager::page::Page;
use crate::file_manager::page::build_page;
use crate::file_manager::block::Block_ID;
use crate::file_manager::page::Page_type;

mod buffer_pool;
use crate::buffer_pool::page_table::Page_table;

mod table;
use crate::table::table::Table;
use crate::table::table::Data_type;


fn main() {
    test_table_creation();
    let mut file_manager = build_file_manager(16384, "./files".to_string());

    let mut page_table = Page_table::new(40000, 16384);

    let table = create_test_table_instance();


    for i in 1..20{ 
        test_adding_basic_column(&table, &mut file_manager, &mut page_table);
        test_adding_basic_column_2(&table, &mut file_manager, &mut page_table);
    }

    //print_table_structure_page(&mut file_manager, &mut page_table);
    for i in 1..5{
        table.add_column("Strings".to_string(), Data_type::String, &mut page_table, &mut file_manager);
    }
    for i in 1..20{ 
        test_adding_basic_column(&table, &mut file_manager, &mut page_table);
        test_adding_basic_column_2(&table, &mut file_manager, &mut page_table);
    }
    table.print_columns_2(&mut page_table, &mut file_manager);
    test_finding_and_modifying_table_column(&table, &mut file_manager, &mut page_table);
    table.remove_column("Strings".to_string(), &mut page_table, &mut file_manager);

}


fn test_new_page_table() {
    let pt = Page_table::new(1024, 256);
    assert_eq!(pt.pages_in_memory.len(), 4);

}

fn test_request_new_page_inserts_page() {
    let mut pt = Page_table::new(512, 128);
    let mut fm = build_file_manager(128, "./files".to_string());

    let block = Block_ID{file_name: "Testing12345".to_string(), number: 42};
    let res = pt.request_new_page(&block, &mut fm);
    assert!(res.is_ok());
    
    println!("{:?}",pt);
}

fn test_page_replacement() {
    let mut pt = Page_table::new(1000000,16384);
    let mut fm = build_file_manager(16384, "./files".to_string());


    for i in 1..6000{
        let b = Block_ID{file_name: "Testing12345".to_string(), number: i};
        let res = pt.request_new_page(&b, &mut fm);
    }

        
    //println!("{:?}",pt);
    println!("{:?}",pt.pages_in_memory.len());


}


fn test_table_creation(){
    let file_manager = build_file_manager(16384, "./files".to_string());
    let table = Table::new("Test_Table".to_string());
    table.init_file(file_manager);
}

fn create_test_table_instance() -> Table{
    return Table::new("Test_Table".to_string())
}

fn print_table_structure_page(file_manager: &mut File_manager, page_table: &mut Page_table){
    let block = Block_ID{file_name: "Test_Table".to_string(), number: 0};
    let page = page_table.get_mut_page(block, file_manager).unwrap();
    println!("{:?} \n", &page.bytes);
    
}


fn test_adding_basic_column(table: &Table, file_manager: &mut File_manager, page_table: &mut Page_table){
    table.add_column("Integers".to_string(),Data_type::Int, page_table, file_manager);
}

fn test_adding_basic_column_2(table: &Table, file_manager: &mut File_manager, page_table: &mut Page_table){
    table.add_column("ABCDEFGHIJKLMNOPQRSTUVWXYZpwehthgegiahearjtoej4naojrfne".to_string(),Data_type::Blob, page_table, file_manager);
}

fn test_finding_and_modifying_table_column(table: &Table, file_manager: &mut File_manager, page_table: &mut Page_table){
    //let result = table.find_column_index("Strings".to_string(), page_table, file_manager).unwrap();
    //println!("Result from searching for Strings: {:?}", result);

    table.modify_column_type("Strings".to_string(), Data_type::Blob, page_table, file_manager);
    table.modify_column_name("Strings".to_string(), "Old, Dwarf's castle".to_string(), page_table, file_manager);
    
    for i in 0..101{
        //println!("{}",i);
        table.modify_column_name("ABCDEFGHIJKLMNOPQRSTUVWXYZpwehthgegiahearjtoej4naojrfne".to_string(), "BLOBs".to_string(), page_table, file_manager);
    }

}


