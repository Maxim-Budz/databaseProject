use crate::file_manager::page::Page;
use crate::file_manager::page::Page_type;
use crate::file_manager::block::Block_ID;



pub struct Table{
    pub table_name:         String,
    pub column_schema:      Vec<Column>,
    
}

pub struct Column{
    pub column_name:    String,
    pub data_type:      Data_type,

}

enum Data_type{
    Int = 0,                    // 8 bytes.
    Float = 1,                  // 8 bytes.
    FixedLength(u16) = 2,       // 2 bytes for size in bytes then string follows

    Variable_string = 3,        // 1 byte for the size of string. if 0 then store 3 bytes for page
                                // num and 2 bytes for index. This will be stored in an overflow
                                // page for larger pieces of data otherwise store up to 255 bytes

    Datetime = 4,               // 8 bytes.
    Date = 5,                   // 4 bytes.
    Time = 6,                   // 4 bytes.
    Bool = 7,                   // 1 bytes.
    Blob = 8,                   // 5 bytes: 3 bytes page num 2 bytes index

}

//in future implement the method to ensure the file is locked and all pages of the file are also
//locked while changes ar ebeing made.

impl Table{

    pub fn new(name: String) -> Table{
        return Table{
            table_name: name,
            column_schema: Vec<Column>,
        }
    }

    pub fn init_file(&self, file_manager: File_manager) -> Result<u8, std::io::Error>{
        let file = file_manager.get_file;
        let page = Page::new(file_manager.block_size);
        
        // add meta data
        
        return file_manager.write(Block{file_name: self.table_name, number: 0}, page)

    }

    pub fn add_column(&self, name: String, type: Data_type, file_manager: File_manager){

        let mut page = Page::new(&file_manager.block_size);
        let block = Block_ID{file_name: self.table_name, number: 0}

        file_manager.read(&block, &page);


        //writing the column to the data section

        let mut bytes = page.bytes;

        let column_name_byte_num = name.len();

        if column_name_byte_num > 255{todo!()};        //too long name error.

        let column_name_bytes = name.into_bytes();

        // 4 bytes for next page pointer                // 2 bytes for col type and col name size
        
        if ((page.record_index_end_point + 4) - page.data_end_point) < (column_name_byte_num + 2 ){ todo!()} // not enough space in page
                                                                                                      //
        page.bytes[page.data_end_point + 1]                          =   column_name_byte_num as u8;
        page.bytes[page.data_end_point + 2 + column_name_bytes_num]  =   type as u8;

        let dst = &mut page.bytes[page.data_end_point + 2 .. page.data_end_point + 1 + column_name_bytes_num];
        let src = &column_name_bytes;

        dst.copy_from_slice(src);



        //updating page meta data
        page.add_record_index(page.data_end_point + 1);

        page.data_end_point = page.data_end_point + 2 + column_name_bytes_num;


    }

    pub fn find_column_index(&self, name: String) -> (u32, u16){

    }

    pub fn remove_column(&self, name: String){
        let location = self.find_column_index(name);

    }

    pub fn modify_column(&self, old_name: String, new_name: String, new_type: Data_type){
        let location = self.find_column_index(name);
    }






}
