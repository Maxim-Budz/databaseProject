// File/page   I/O
use std::fs::File;
use std::io::Write;


pub fn create_database_file() -> std::io::Result<()> {

    let mut file = File::create_new("test.mdbf")?;
    file.write_all(".mdbf".as_bytes())?;
    file.write_all("1.0.0".as_bytes())?;

    Ok(())
   

}

fn add_page() -> u32{

}

fn remove_page(){}

fn modify_page(){}


struct Header{
    file_type:      String,
    version:        u8,
    page_size:      u16,
}

fn build_header(version: u8) -> Header{

    Header{
        file_type:  String::from(".mdbf"),
        version:    1,
        page_size:  16_384,
    }
}

struct database_page{
    page_type: PageType,
    next_page_pointer: i32,
}

enum PageType{
    data,                //{ content: &[field_type] },
    table_index,         //{ record_pointers: &[u32] },
    table_schema,        //{ table_index_pointer: u32, format: &[field_type] },
    database_table_list, //{ table_name: &str, table_pointers: &[u32]},
}


struct table_schema_content{
    table_name:             String,
    table_index_pointer:    u32,
    format:                 [field_type],
}

enum field_type{
    integer,
    float,      
    boolean,
    text(length_type),
    dateTime,
    blob,

}


enum length_type{
    fixed       {size: u32},
    non_fixed,
}
