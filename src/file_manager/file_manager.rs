use std::collections::HashMap;
use std::collections::hash_map::Entry;
use crate::file_manager::page::Page;
use crate::file_manager::block::Block_ID;
use std::fs::File;
use std::io::SeekFrom;
use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;

pub struct File_manager{
    pub block_size: u16,
    pub data_directory: String,
    pub opened_files: HashMap<String, File>,

    //implement mutex
    
}

pub fn build_file_manager(block_size: u16, data_directory: String) -> File_manager{
    File_manager{
        block_size,
        data_directory,
        opened_files: HashMap::new(),

    }
}




impl File_manager{



    //extract contents of a file block into a page's bytes vector
    //and then change the page values to account for metadata
    
    pub fn read(&mut self, block: &Block_ID, page: &mut Page) -> Result<u8, std::io::Error>{

        let block_size = self.block_size;
        let block_total = self.total_blocks(&block.file_name)?;
        let file = self.get_file(&block.file_name)?;

        if block_total < block.number {
            //block not in file error
            return Err(Error::new(ErrorKind::Other, "Block number is too large for file."));
        }


        file.seek(SeekFrom::Start(u64::from(block_size) * u64::from(block.number)));

        let mut write_buffer = vec![0; block_size as usize];

        file.read(&mut write_buffer)?;

        let page_num_bytes = &write_buffer[0..4];
        let page_type = &write_buffer[4];
        let prev_page_bytes: [u8; 4] = write_buffer[5..9].try_into().expect("error with getting prev page bytes at line 63 in filemanager");
        let next_page_bytes = &write_buffer[9..13];
        let data_end_point_bytes = &write_buffer[13..15];
        let record_index_end_point_bytes = &write_buffer[15..17];
        let content = &write_buffer[17..];

        page.page_num = block.number;

        page.previous_index = match u32::from_be_bytes(prev_page_bytes){
                                0 => None,
                                x => Some(x),
        };

        page.next_index = match u32::from_be_bytes(prev_page_bytes){
                            0 => None,
                            x => Some(x),
        };


        page.data_end_point = (data_end_point_bytes[0] as u16) << 8 | data_end_point_bytes[1] as u16;
        page.record_index_end_point = (record_index_end_point_bytes[0] as u16) << 8 | record_index_end_point_bytes[1] as u16;

        page.write(0, write_buffer.to_vec());

        //println!("------------------------>{:?}<-------------------------", page);

        return Ok(1);


    }





    //insert contents of a page's bytes into a file block

    pub fn write(&mut self, block: &Block_ID, page: &Page) -> Result<u8, std::io::Error>{

        let block_size = self.block_size;
        let block_total = self.total_blocks(&block.file_name)?;
        let file = self.get_file(&block.file_name)?;

        if block_total < block.number {

            let blocks_to_be_added_number = block.number - block_total + 1;
            let data = vec![0; (blocks_to_be_added_number * block_size as u32) as usize ];
    
            file.write(&data)?;
        }

        file.seek(SeekFrom::Start(u64::from(block_size) * u64::from(block.number)));

        let mut data = vec![0; page.size()];

        page.read(0, &mut data);

        //adding the meta data:
        data[0..4].copy_from_slice(&block.number.to_be_bytes());

        data[4] = page.page_type.clone() as u8;


        match page.previous_index{
            None => data[5..9].copy_from_slice(&[0,0,0,0]),
        
            Some(n) => data[5..9].copy_from_slice(&n.to_be_bytes()),
        };



        match page.next_index{
            None => data[9..13].copy_from_slice(&[0,0,0,0]),
            Some(n) => data[9..13].copy_from_slice(&n.to_be_bytes()),
        };
        
        data[13..15].copy_from_slice(&page.data_end_point.to_be_bytes());

        data[15..17].copy_from_slice(&page.record_index_end_point.to_be_bytes());



        file.write(&data)?;

        return Ok(1)
    }







    pub fn close_all(&mut self){

        let opened_files_iter = self.opened_files.iter();
        for file in opened_files_iter {
            drop(file);
        }

        self.opened_files.clear();
    }




    pub fn total_blocks(&mut self, file_name: &String) -> Result<u32, std::io::Error>{

         let file = self.get_file(file_name)?;
         return Ok( (file.metadata().unwrap().len() / u64::from(self.block_size)) as u32 )
    }




    pub fn get_file(&mut self, file_name: &String) -> Result<&mut File, std::io::Error> {
        
        match self.opened_files.entry(file_name.to_string()) {
            Entry::Occupied(entry)  => Ok(entry.into_mut()),
            Entry::Vacant(entry)    =>{
                let path_string = format!("{}/{}", self.data_directory, file_name);
                let path = Path::new(&path_string);
                let file = File::options().read(true).write(true).create(true).open(&path)?;

                Ok(entry.insert(file))

            }

        }
    }


}

