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

    String = 3,        // 1 byte for the size of string. if 0 then store 3 bytes for page
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
        //TODO error checking here 
        let mut page = page_table.get_mut_page(block, file_manager).unwrap();


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

    }



//TODO test and fix going to other pages
    pub fn find_column_index(&self, name: String, page_table: &mut Page_table, file_manager: &mut File_manager ) -> Option<(u32, u16)>{

        //get byte length of the name then linearly search through the columns and find one which
        //matches the byte length number. Then check if the names are the same.
        let column_name_bytes_num = name.len() as u8;
    
        let block = Block_ID{file_name: self.table_name.clone() ,number: 0};

        let mut page = match page_table.get_mut_page(block, file_manager){
                        None    => return None,
                        Some(p) => p,
                    };
        println!("{:?}", column_name_bytes_num);
        println!("{:?}", page.get_record_index());

        

        let mut index = 19;

        let mut page_num = 0;

        loop{
            if (index > page.record_index_end_point - 2){
                let next_page_bytes = &page.bytes[ (page.record_index_end_point-4) as usize .. (page.record_index_end_point) as usize ];

                let page_num = (( next_page_bytes[0] as u32) << 24)
                             | (( next_page_bytes[1] as u32) << 16)
                             | (( next_page_bytes[2] as u32) << 8 )
                             |  ( next_page_bytes[3] as u32);

                if page_num == 0{
                    return None
                }


                let block = Block_ID{file_name: self.table_name.clone(), number: page_num};

                page = match page_table.get_mut_page(block, file_manager){
                                None    => return None,
                                Some(p) => p
                            };


            }


            if (page.bytes[index as usize] == column_name_bytes_num ){
                let name_bytes = &page.bytes[(index+1) as usize .. (index+1+column_name_bytes_num as u16) as usize ];

                let s = String::from_utf8(name_bytes.to_vec()).unwrap();

                if s == name{
                    return Some( (page_num, index-1) ) // index -1 because the start of the column
                                                       // is 1 before the stringname size.
                }
            }

            //increment to next column name byte num
            index += (page.bytes[index as usize] + 2) as u16;
            
        };

        return None
    }

    pub fn remove_column(&self, name: String, page_table: &mut Page_table, file_manager: &mut File_manager){
        let location = self.find_column_index(name, page_table, file_manager).unwrap();

        let block = Block_ID{file_name: self.table_name.clone(), number: location.0};

        let mut page = match page_table.get_mut_page(block, file_manager){
                        None    => return (),
                        Some(p) => p,
        };


        let start_index = location.1;
        let string_byte_num = page.bytes[ (start_index + 1) as usize];
        let end_index = start_index + 2 + string_byte_num as u16;

        
        if end_index > page.data_end_point{
            page.remove_data_range(start_index, page.data_end_point - 1);
        }else{
            page.remove_data_range(start_index, end_index);
        }
        
        page.remove_record_index(start_index);


    }

    pub fn modify_column_name(&self, old_name: String, new_name: String, page_table: &mut Page_table, file_manager: &mut File_manager){
        
        let location = match self.find_column_index(old_name.clone(), page_table, file_manager){
                        None    => {
                            println!("A");
                            return ()}, // update
                        Some(n) => n,
        };

        let block = Block_ID{file_name: self.table_name.clone(), number: 0};
        let mut page = page_table.get_mut_page(block, file_manager).unwrap();

        //if the string size is > to original then shift everything after the end of the string
        //enough places and then overwrite
        //but if the bytes requireed exceeds the data heap then if there is no next page, create
        //new one 
        //otherwise find the existing page and try to add.
        
        let start_index = location.1;
        let string_bytes_num = page.bytes[ (start_index + 1) as usize];
        let end_index = start_index + 2 + string_bytes_num as u16;

        if new_name.len() <= old_name.len(){
            let dst = &mut page.bytes[2 + start_index as usize .. 2 +start_index as usize + new_name.len()];
            let src = new_name.as_bytes();

            dst.copy_from_slice(src);
            page.remove_data_range(start_index + 2 + new_name.len() as u16, end_index);

            let start_record_index_point = page.record_index_end_point - page.get_record_count_after(start_index);
            let end_record_index_point = page.record_index_end_point;
            page.update_records_after(start_index, (old_name.len() - new_name.len()) as u16 , false);

        }else{

            let bytes_to_add_number = new_name.len() - old_name.len();
            if page.data_end_point + bytes_to_add_number as u16 >= page.record_index_end_point{
                //TODO make a new page and insert remaining stuff in there like a linked list.
            }

            page.bytes.copy_within(end_index as usize .. page.data_end_point as usize, end_index as usize + bytes_to_add_number );

            let dst = &mut page.bytes[start_index as usize + 2 .. start_index as usize + 2 + new_name.len()];
            let src = new_name.as_bytes();
            dst.copy_from_slice(src);

            let start_record_index_point = page.record_index_end_point - page.get_record_count_after(start_index);
            let end_record_index_point = page.record_index_end_point;

            page.update_records_after(start_index, bytes_to_add_number as u16, true);
        }

        page.bytes[start_index as usize + 1] = new_name.len() as u8;
        
    }



    pub fn modify_column_type(&self, name: String, new_type: Data_type, page_table: &mut Page_table, file_manager: &mut File_manager){
        let location = self.find_column_index(name, page_table, file_manager).unwrap();
        let block    = Block_ID{file_name: self.table_name.clone(), number: location.0};
        let mut page = page_table.get_mut_page(block, file_manager).unwrap();
        
        page.bytes[location.1 as usize] = new_type as u8;
        
    }

    pub fn print_columns(&self, page_table: &mut Page_table, file_manager: &mut File_manager){
        //add so it works for all pages
        let mut page = page_table.get_mut_page(Block_ID{file_name: self.table_name.clone(), number: 0}, file_manager).unwrap();
        let mut index = 18;
        let mut count = 0;
        loop{
            let t = page.bytes[index];
            let string_size_bytes = page.bytes[index + 1];
            let string = std::str::from_utf8(&page.bytes[(index + 2) as usize .. (index as usize + 2 + string_size_bytes as usize) as usize]).unwrap();
            count += 1;
            println!("Index: {}, Type: {}, Name: {}", count, t, string);

            index += 2 + string_size_bytes as usize;

            if index > page.data_end_point as usize{
                return ()
            }

        }

    }







}
