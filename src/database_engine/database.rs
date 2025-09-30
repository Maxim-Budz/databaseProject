pub struct Database {
    pub tables: HashMap<String, Table>;

}





impl Database{
    pub fn new(&mut file_manager, &mut page_table){

        //Initialise the table that stores IDs that link variable data.

        let variable_data_index_table = Table::new("Variable_Data_Index_Table".to_string());
        //TODO: IMPLEMENT U32 and U16 data types for table.
        variable_data_index_table.add_column("ID".to_string(), Data_Type::U32, page_table, file_manager);
        variable_data_index_table.add_column("Page_Num".to_string(), Data_Type::U32, page_table, file_manager);
        variable_data_index_table.add_column("Page_Index".to_string(), Data_Type::U16, page_table, file_manager);

        
        //Continued if I need more initialisation code


    }


}
