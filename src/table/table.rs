use crate::file_manager::page::Page;
use crate::file_manager::page::Page_type;
use crate::file_manager::block::Block_ID;
use crate::file_manager::file_manager::File_manager;
use crate::buffer_pool::page_table::Page_table;



pub struct Table{
    pub table_name:         String,
    pub column_schema:      Vec<Column>,
    
}

pub struct Column{
    pub column_name:    String,
    pub data_type:      Data_type,

}

#[repr(u8)]
pub enum Data_type{
    Int = 0,                    // 8 bytes.
    Float = 1,                  // 8 bytes.
    //FixedLength(u16) = 2,       // 2 bytes for size in bytes then string follows

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
            column_schema: Vec::new(),
        }
    }

    pub fn init_file(&self, mut file_manager: File_manager) -> Result<u8, std::io::Error>{
        let file = file_manager.get_file(&self.table_name);
        let page = Page::new(file_manager.block_size, 0, Page_type::Table_structure);
        
        // add meta data
        
        return file_manager.write(&Block_ID{file_name: self.table_name.clone(), number: 0}, &page)

    }

    pub fn add_column(&self, name: String, data_type: Data_type, page_table: &mut Page_table,file_manager: &mut File_manager){
        // search if there needs to be another page to have the columns in.
        //let mut page = Page::new(file_manager.block_size, 0, Page_type::Table_structure );
        let block = Block_ID{file_name: self.table_name.clone(), number: 0};

        //file_manager.read(&block, &mut page);
        //
        let mut page = page_table.get_mut_page(block, file_manager);


        //writing the column to the data section


        let column_name_byte_num = name.len();

        if column_name_byte_num > 255{todo!()};        //too long name error.

        let column_name_bytes = name.into_bytes();

        // 4 bytes for next page pointer                // 2 bytes for col type and col name size
        
        if ((page.record_index_end_point + 4) - page.data_end_point) < (column_name_byte_num + 2) as u16 { todo!()} // not enough space in page
                                                                                                      //
        page.bytes[(page.data_end_point + 1) as usize]                          =   data_type as u8;
        page.bytes[(page.data_end_point + 2) as usize]  = column_name_byte_num  as u8;

        //println!("page.data_end_point: {}",page.data_end_point);
        //println!("column_name_byte_num: {}", column_name_byte_num);
        //println!("column_name_bytes: {:?}", column_name_bytes);


        let dst = &mut page.bytes[(page.data_end_point + 3) as usize .. (page.data_end_point + 3) as usize + column_name_byte_num];
        let src = &column_name_bytes;

        dst.copy_from_slice(src);



        //updating page meta data
        page.add_record_index(page.data_end_point + 1);

        page.data_end_point = page.data_end_point + 2 + column_name_byte_num as u16;



        //println!("Page: {:?}", page);


    }

    pub fn find_column_index(&self, name: String, page_table: &mut Page_table, file_manager: &mut File_manager ) -> Option<(u32, u16)>{

        //get byte length of the name then linearly search through the columns and find one which
        //matches the byte length number. Then check if the names are the same.
        let column_name_bytes_num = name.len();
    
        let block = Block_ID{file_name: self.table_name.clone() ,number: 0};

        let page = match page_table.get_mut_page(block, &file_manager){
                        None    => return None,
                        Some(p) => p,
                    };

        

        let index = 19;

        let page_num = 0;

        loop{
            if (index > page.record_index_end_point - 2){
                let next_page_bytes = page.bytes[ (page.record_index_end_point-4) .. (page.record_index_end_point-1) ];

                let page_num = (( next_page_bytes[0] as u32) << 24)
                             | (( next_page_bytes[1] as u32) << 16)
                             | (( next_page_bytes[2] as u32) << 8 )
                             |  ( next_page_bytes[3] as u32);


                let block = Block_ID{file_name: &self.file_name, number: &page_num};

                let page = match page_table.get_mut_page(block, &file_manager){
                                None    => return None,
                                Some(p) => p
                            };


            }

            if (page.bytes[index] == column_name_bytes_num){
                let name_bytes = page.bytes[index+1..index+column_name_bytes_num];

                let s = String::from_utf8_lossy(name_bytes.to_vec()).unwrap();

                if s == name{
                    return (page_num, index)
                }
            }

            //increment to next column name byte num
            index += (page.bytes[index] + 1);
            
        };

        return None
    }

    pub fn remove_column(&self, name: String, page_table: &mut Page_table, file_manager: &mut File_manager){
        let location = self.find_column_index(name);

        let block = Block_ID{file_name: &self.file_name, number: location[0]};

        let mut page = match page_table.get_mut_page(block, file_manager){
                        None    => return None,
                        Some(p) => p,
        };
        
        let start_index = location[1];
        let string_byte_num = page.bytes[start_index + 1];
        let end_index = start_index + 1 + string_byte_num;

        //start at the index found from location then calculate the size of the column data to be
        //removed
        //
        //if the part of the column is in another page 
        //(index + string byte count > index_ref_end_point)
        //remove all remaining bytes as usual but then open the other page and remove those bytes
        //aswell and do the appropriate stuff there. (currently not implemented).
        //
        //after size calc, move all column bytes upwards to replace the column to be removed.
        //
        //then subtract the bytes of the removed column from all reference indexes and then shift
        //them all up two places.
        
        if end_index > page.data_end_point{
            // the record is in another page.
            page.remove_record(start_index, page.data_end_point);
            //then get the 5 bytes after to figure out the page num and index then remove the
            //remaining bytes.

        }else{
            page.remove_record(start_index, end_index);
        }



    }

    pub fn modify_column_name(&self, old_name: String, new_name: String, new_type: Data_type){
        let location = self.find_column_index(old_name);

        
    }

    pub fn modify_column_type(&self, name: String, new_type: Data_type){
        let location = self.find_column_index(name);
        let block    = Block_ID{file_name: &self.file_name, number: location[0]};
        let mut page = page_table.get_mut_page(block, file_manager);

        page.bytes[location[1]] = new_type as u8;

    }






}
