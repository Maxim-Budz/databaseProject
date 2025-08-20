use std::collections::HashMap;
use std::collections::hash_map::Entry;
use crate::file_manager::page::Page;
use crate::file_manager::block::Block_ID;
use std::fs::File;
use std::io::SeekFrom;
use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;

pub struct File_manager{
    block_size: u32,
    data_directory: String,
    opened_files: HashMap<String, File>,

    //implement mutex
    
}

pub fn build_file_manager(block_size: u32, data_directory: String) -> File_manager{
    File_manager{
        block_size,
        data_directory,
        opened_files: HashMap::new(),

    }
}



impl File_manager{



    //extract contents of a file block into a page's bytes vector
    
    pub fn read(&mut self, block: &Block_ID, page: &mut Page) -> Result<u8, std::io::Error>{

        let block_size = self.block_size;
        let block_total = self.total_blocks(&block.file_name)?;
        let file = self.get_file(&block.file_name)?;

        if(block_total < block.number){
            //block not in file error
            return Err(Error::new(ErrorKind::Other, "Block number is too large for file."));
        }


        file.seek(SeekFrom::Start(u64::from(block_size) * u64::from(block.number)));

        let mut write_buffer = vec![0; block_size as usize];

        file.read(&mut write_buffer)?;

        page.write(0, write_buffer.to_vec());

        return Ok(1);


    }






    //insert contents of a page's bytes into a file block

    pub fn write(&mut self, block: &Block_ID, page: &Page) -> Result<u8, std::io::Error>{

        let block_size = self.block_size;
        let block_total = self.total_blocks(&block.file_name)?;
        let file = self.get_file(&block.file_name)?;

        if(block_total < block.number){

            let blocks_to_be_added_number = block.number - block_total + 1;
            let data = vec![0; (blocks_to_be_added_number * block_size) as usize ];
    
            file.write(&data)?;
        }

        file.seek(SeekFrom::Start(u64::from(block_size) * u64::from(block.number)));

        let mut data = vec![0; page.size()];

        page.read(0, &mut data);

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

