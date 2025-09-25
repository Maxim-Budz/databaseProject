// CLI entry point
//
//TODO LIST:
//
//-implement saving the u8 type and u64 size for data via variable_data_manager.
//
//-implement a hash map of file name to last page number in page table. This fixes bug NO. 2
//
//-implement a  ID to Data reference table so that I can link records to the data reference so that
// if this changes it only needs to be updated at the table.

///BUG LIST:
/// 1 -infinite stalling when replacing a page, possibly due to the pin count not being reduced when
/// page is no longer in use.
///
/// 2 -creating new overflow pages via total blocks doesn't get the correct page num when there a
/// blocks in the page table that are not yet written to file
///
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
use crate::table::table::open_table;
use crate::table::variable_data_manager::Variable_data_manager;


fn main() {
    let mut file_manager = build_file_manager(16384, "./files".to_string());

    let mut page_table = Page_table::new(163840, 16384);

    let mut table = open_table("Test_Table".to_string(), &mut file_manager, &mut page_table).unwrap();
    let mut table = Table::new("Test_Table".to_string(), &mut file_manager);

    //println!("{:?}", table);
    //table.print_columns_2(&mut page_table, &mut file_manager);

    let mut variable_data_manager = Variable_data_manager::new("Test_Table".to_string(), table.first_data_page_num, &table.data_free_space_tracker_page_num, &mut page_table, &mut file_manager);
    let d = "TestingTESTING123456789! ====MMMakndnwnoinfiowneio nri33nir12u848962389591y9248013hnp5rini2n3mrefs;';f#'eelfminwiorhhwrmm".as_bytes();
    for i in 0..1000{
        variable_data_manager.add_data(&d, &mut page_table, &mut file_manager);
        //page_table.write_all(&mut file_manager);
        
        println!("{:?}", variable_data_manager.free_bytes);
    }

    //TODO TEST ADDING DATA TO DATA PAGES.

    page_table.write_all(&mut file_manager);
    //println!("................................................................................................................");
    //println!("page_table: {:?}", page_table);


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
    let mut file_manager = build_file_manager(16384, "./files".to_string());
    let table = Table::new("Test_Table".to_string(), &mut file_manager);
    table.init_file(&mut file_manager);
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

fn add_all_columns(table: &mut Table, page_table: &mut Page_table, file_manager: &mut File_manager) {
    use Data_type::*;

    let all_types = [
        (Int, "Int"),
        (Float, "Float"),
        (String, "String"),
        (Datetime, "DateTime"),
        (Date, "Date"),
        (Time, "Time"),
        (Bool, "Bool"),
        (Enum, "Enum"),
        (Blob, "Blob"),
    ];

    for (dtype, name) in all_types {
        table.add_column(name.to_string(), dtype, page_table, file_manager);
    }
}


