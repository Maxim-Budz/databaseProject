use crate::file_manager::page::Page;
use crate::file_manager::page::Page_type;
use crate::file_manager::block::Block_ID;
use crate::file_manager::file_manager::File_manager;
use crate::buffer_pool::page_table::Page_table;


#[derive(Debug)]
pub struct Table{
    pub table_name:     String,
    pub column_schema:  Vec<Column>,

    pub first_data_page_num:    u32,    
    pub first_record_page_num:  u32,
    pub b_tree_page_num:        u32,
    pub data_free_space_tracker_page_num:       u32,
    pub record_free_space_tracker_page_num:     u32,
    
}
#[derive(Debug, Clone)]
pub struct Column{
    pub column_name:    String,
    pub data_type:      Data_type,

}

#[derive(Debug, Clone, PartialEq)]
pub enum Value{
    Int(i64),
    Float(f64),
    String(String),
    Datetime(u64),
    Date(u32),
    Time(u32),
    Bool(bool),
    Enum(String),
    Blob(Vec<u8>),
}



impl PartialEq<Data_type> for Value {
    fn eq(&self, other: &Data_type) -> bool {
        match self {
            Value::Int(_)      => *other == Data_type::Int,
            Value::Float(_)    => *other == Data_type::Float,
            Value::String(_)   => *other == Data_type::String,
            Value::Datetime(_) => *other == Data_type::Datetime,
            Value::Date(_)     => *other == Data_type::Date,
            Value::Time(_)     => *other == Data_type::Time,
            Value::Bool(_)     => *other == Data_type::Bool,
            Value::Enum(_)     => *other == Data_type::Enum,
            Value::Blob(_)     => *other == Data_type::Blob,
        }
    }
}

impl Value{

    fn size(&self) -> u8{

        return match self{
            Value::Int(_)
            | Value::Float(_)
            | Value::Datetime(_) => 8,

            Value::String(_) 
            | Value::Enum(_) 
            | Value::Blob(_)  => 6,

            Value::Date(_) 
            | Value::Time(_)  => 4,

            Value::Bool(_) => 1,
        }
    }


}



#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum Data_type{
    Int = 0,                    // 8 bytes.
    Float = 1,                  // 8 bytes.
    String = 3,                 // 6 bytes: 4 bytes page num 2 bytes index
    Datetime = 4,               // 8 bytes.
    Date = 5,                   // 4 bytes.
    Time = 6,                   // 4 bytes.
    Bool = 7,                   // 1 bytes.
    Enum = 8,                   // 6 bytes: 4 bytes page num 2 bytes index
    Blob = 9,                   // 6 bytes: 4 bytes page num 2 bytes index

}





impl PartialEq<Value> for Data_type {
    fn eq(&self, other: &Value) -> bool {
        other == self
    }
}



impl std::convert::TryFrom<u8> for Data_type {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Data_type::Int),
            1 => Ok(Data_type::Float),
            3 => Ok(Data_type::String),
            4 => Ok(Data_type::Datetime),
            5 => Ok(Data_type::Date),
            6 => Ok(Data_type::Time),
            7 => Ok(Data_type::Bool),
            8 => Ok(Data_type::Enum),
            9 => Ok(Data_type::Blob),
            _ => Err(()),
        }
    }
}

pub fn open_table(name: String, file_manager: &mut File_manager, page_table: &mut Page_table) -> Option<Table>{
    let mut table = Table::new(name, file_manager);
    table.column_schema = table.parse_columns(page_table, file_manager);

    //load the correct starting page nums aswell (not important)
    return Some(table);
}


//in future implement the method to ensure the file is locked and all pages of the file are also
//locked while changes ar ebeing made.

impl Table{

    pub fn new(name: String, file_manager: &mut File_manager) -> Table{
        let table = Table{
            table_name: name,
            column_schema: Vec::new(),
            first_data_page_num:                5, //TEMP
            first_record_page_num:              4, //TEMP
            b_tree_page_num:                    3, //TEMP
            data_free_space_tracker_page_num:   2, //TEMP
            record_free_space_tracker_page_num: 1, //TEMP
        };
        //TODO CHECK IF FILE ALREADY EXISTS AND JUST LOAD THE DATA....
       // table.init_file(file_manager);

        return table
    }



    pub fn init_file(&self, file_manager: &mut File_manager) -> Result<u8, std::io::Error>{

        //generating the initial pages
        let mut init_pages: Vec<Page> = Vec::new();
        let free_space: u16 = (1024 * 16) - 18;

        init_pages.push(Page::new(file_manager.block_size, 0, Page_type::Table_structure));
        init_pages.push(Page::new(file_manager.block_size, 1, Page_type::Free_space_tracker));

        let mut bytes: [u8;6]   = [0u8; 6];
        let page_num: u32 = 4;
        bytes[..4].copy_from_slice( &page_num.to_be_bytes()  );
        bytes[4..].copy_from_slice( &free_space.to_be_bytes());
        println!("1: {:?}", &bytes);
        init_pages.last_mut().unwrap().write_at_end(bytes.to_vec());


        init_pages.push(Page::new(file_manager.block_size, 2, Page_type::Free_space_tracker));

        let page_num: u32 = 5;
        bytes[..4].copy_from_slice( &page_num.to_be_bytes()  );
        bytes[4..].copy_from_slice( &free_space.to_be_bytes());
        println!("2: {:?}", &bytes);
        init_pages.last_mut().unwrap().write_at_end(bytes.to_vec());


        init_pages.push(Page::new(file_manager.block_size, 3, Page_type::B_tree));
        init_pages.push(Page::new(file_manager.block_size, 4, Page_type::Record)); 
        init_pages.push(Page::new(file_manager.block_size, 5, Page_type::Data));
        
        let file_name = self.table_name.clone();

        for page in init_pages{
            let res = file_manager.write(&Block_ID{file_name: file_name.clone(), number: page.page_num}, &page);

            if !res.is_ok(){
                return Ok(1) // ERROR CHECKING & Cancel all changes
            }
        }

        return Ok(0)

    }




    pub fn add_column(&self, name: String, data_type: Data_type, page_table: &mut Page_table,file_manager: &mut File_manager){
        
        let block = Block_ID{file_name: self.table_name.clone(), number: 0};


        let mut page = page_table.get_mut_page(block, file_manager).unwrap();


        //writing the column to the data section


        let column_name_byte_num = name.len();

        if column_name_byte_num > 255{todo!()};        //too long name error.

        let column_name_bytes = name.into_bytes();

        // 4 bytes for next page pointer                // 2 bytes for col type and col name size
        
        if ((page.record_index_end_point + 4) - page.data_end_point) < (column_name_byte_num + 2) as u16 {

            //create new page



        } 
                                                                                                      
        page.bytes[(page.data_end_point + 1) as usize]  =   data_type as u8;
        page.bytes[(page.data_end_point + 2) as usize]  = column_name_byte_num  as u8;


        //let dst = &mut page.bytes[(page.data_end_point + 3) as usize .. (page.data_end_point + 3) as usize + column_name_byte_num];
        //let src = &column_name_bytes;

        //dst.copy_from_slice(src);


        page.write((page.data_end_point + 3), column_name_bytes);



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
        //println!("{:?}", column_name_bytes_num);
        //println!("{:?}", page.get_record_index());

        

        let indexes = page.get_record_index();

        let mut page_num = 0;

        for index in indexes{
            //println!("index: {:?}", index);
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
           // println!("size: {:?} \n", &page.bytes[index as usize +1]);

            if (page.bytes[index as usize + 1] == column_name_bytes_num ){

                //println!("AA");
                let name_bytes = &page.bytes[(index+2) as usize .. (index+2+column_name_bytes_num as u16) as usize ];

                let s = String::from_utf8(name_bytes.to_vec()).unwrap();

                //println!("{:?}", s);
                //println!("{:?}", name);

                if s == name{
                    return Some( (page_num, index) ) // index -1 because the start of the column
                                                       // is 1 before the stringname size.
                }
            }

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
        
        page.update_records_after(start_index, (end_index - start_index), false);
        page.remove_record_index(start_index);


    }




    pub fn modify_column_name(&self, old_name: String, new_name: String, page_table: &mut Page_table, file_manager: &mut File_manager){
        
        let location = match self.find_column_index(old_name.clone(), page_table, file_manager){
                        None    => {
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
            //let dst* = &mut page.bytes[2 + start_index as usize .. 2 +start_index as usize + new_name.len()];
            //let src = new_name.as_bytes();

            //dst.copy_from_slice(src);

            page.write( (2 + start_index), new_name.as_bytes().to_vec());

            page.remove_data_range(start_index + 2 + new_name.len() as u16, end_index);

            page.update_records_after(start_index, (old_name.len() - new_name.len()) as u16 , false);

        }else{

            let bytes_to_add_number = new_name.len() - old_name.len();
            if page.data_end_point + bytes_to_add_number as u16 >= page.record_index_end_point{
                //TODO make a new page and insert remaining stuff in there like a linked list.
            }

            page.bytes.copy_within(end_index as usize .. page.data_end_point as usize + 1, end_index as usize + bytes_to_add_number );

            //let dst* = &mut page.bytes[start_index as usize + 2 .. start_index as usize + 2 + new_name.len()];
            //let src = new_name.as_bytes();
            //dst.copy_from_slice(src);

            page.write((start_index + 2), new_name.as_bytes().to_vec());


            page.update_records_after(start_index, bytes_to_add_number as u16, true);
            page.data_end_point += bytes_to_add_number as u16;
        }

        page.bytes[start_index as usize + 1] = new_name.len() as u8;
        
    }



    pub fn modify_column_type(&self, name: String, new_type: Data_type, page_table: &mut Page_table, file_manager: &mut File_manager){
        let location = self.find_column_index(name, page_table, file_manager).unwrap();
        let block    = Block_ID{file_name: self.table_name.clone(), number: location.0};
        let mut page = page_table.get_mut_page(block, file_manager).unwrap();
        
        page.bytes[location.1 as usize] = new_type as u8;
        
    }

    pub fn parse_columns(&self, page_table: &mut Page_table, file_manager: &mut File_manager) -> Vec<Column>{
        let mut page = page_table.get_mut_page(Block_ID{file_name: self.table_name.clone(), number: 0}, file_manager).unwrap();
        let mut indexes = page.get_record_index();
        let mut count = 0;
        let mut column_vector = Vec::new();
        for index in indexes.iter().rev(){
            count += 1;
            let t = page.bytes[*index as usize];
            let string_size_bytes = page.bytes[*index as usize + 1];
            let string = std::str::from_utf8(&page.bytes[(*index + 2) as usize .. (*index as usize + 2 + string_size_bytes as usize)]).unwrap();
            column_vector.push(Column{column_name: string.to_string().clone(), data_type: Data_type::try_from(t.clone()).unwrap()})
        }

        return column_vector

        //now check if we need to go to other page.

    }




    pub fn print_columns_2(&self, page_table: &mut Page_table, file_manager: &mut File_manager){ 

        let mut page = page_table.get_mut_page(Block_ID{file_name: self.table_name.clone(), number: 0}, file_manager).unwrap();

        let indexes = page.get_record_index();

        let mut count = 0;

        for index in indexes.iter().rev(){
            count += 1;
            let t = page.bytes[*index as usize];
            let string_size_bytes = page.bytes[*index as usize + 1];
            let string = std::str::from_utf8(&page.bytes[(*index + 2) as usize .. (*index as usize + 2 + string_size_bytes as usize)]).unwrap();
            println!("Number: {} \t | \t Index: {}, \t | \t Type: {}, \t | \t Name: {}", count,index, t, string);

        }
    }

    pub fn add_record(&mut self, record: Vec<Value>, page_table: &mut Page_table, file_manager: &mut File_manager){
        //verify if [values] follows the table schema.
        //
        //Find the page to write the record to.
        //
        //Find suitable data pages to write larger data to.
        //
        //
        //Write the record to the record page and then write all the data to the data pages.
        //
        //
        //Overflow if necessary.
        //
        let mut record_size = 0;
        
        for i in 0..self.column_schema.len(){
            if self.column_schema[i].data_type != record[i]{
                //ERROR entry type doesnt't match the column schema.
                return ()
            }

            record_size += record[i].size();
        }


        // generate the bytes that shall be stored with empty spaces for the strings, enums, blobs
        // for now...


        
        


    }




    pub fn find_record(){
        //using B-Tree we can find the record by comparing fields with the search term.
    }
    pub fn modify_record(){
        //Find record location
        //
        //
        //If simple data then modify that and continue (no need for resizing)
        //
        //
        // If larger data is modified, go to the correct data page and modify the data there. There
        // will be a need to resize it.
    }
    pub fn remove_record(){
        //First get a list of all larger data locations
        //Then remove the record and shift all the data left to fill the place, also locate the overflow
        //bytes and shift them aswell. 
        //
        //Finally, go to data page and de-allocate space so that new data can go there.

    }






}
