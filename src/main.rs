// CLI entry point
//
//TODO LIST:
//
//-implement saving the u8 type and u64 size for data via variable_data_manager.
//
//
//-implement a  ID to Data reference table so that I can link records to the data reference so that
// if this changes it only needs to be updated at the table.

///BUG LIST:
/// NONE FOR NOW ...
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
use std::time::Instant;


fn main() {
    let now = Instant::now();
    let mut file_manager = build_file_manager(16384, "./files".to_string());
    let mut file_list: Vec<String> = Vec::new();
    file_list.push("Test_Table".to_string());
    let mut page_table = Page_table::new(163840, 16384, file_list, &mut file_manager);

    let mut table = open_table("Test_Table".to_string(), &mut file_manager, &mut page_table).unwrap();
    let mut table = Table::new("Test_Table".to_string(), &mut file_manager);

    //println!("{:?}", table);
    //table.print_columns_2(&mut page_table, &mut file_manager);

    let mut variable_data_manager = Variable_data_manager::new("Test_Table".to_string(), table.first_data_page_num, &table.data_free_space_tracker_page_num, &mut page_table, &mut file_manager);
    let d = "TestingTESTING123456789! ====MMMakndnwnoinfiowneio nri33nir12u848962389591y9248013hnp5rini2n3mrefs;';f#'eelfminwiorhhwrmm".as_bytes();
    for i in 0..1000000{
        variable_data_manager.add_data(&d, &mut page_table, &mut file_manager);
        //page_table.write_all(&mut file_manager);
        
    }

    //TODO TEST ADDING DATA TO DATA PAGES.

    page_table.write_all(&mut file_manager);
    //println!("................................................................................................................");
    //println!("page_table: {:?}", page_table);

    let duration = now.elapsed();
    println!("Completed!, Time elapsed: {:?}", duration);
}
