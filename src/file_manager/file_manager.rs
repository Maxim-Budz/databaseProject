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


    //TODO ADD ERROR CHECKING.

    pub fn read(&mut self, block: &Block_ID, page: &mut Page) -> u8{
        let block_size = self.block_size;


        let file = match self.get_file(&block.file_name){
            Ok(file)    => file,
            _           => return 0,
        };

        //implement check for range here

        file.seek(SeekFrom::Start(u64::from(block_size) * u64::from(block.number)));

        let mut write_buffer = vec![0; block_size as usize];

        file.read(&mut write_buffer);

        page.Write(0, write_buffer.to_vec());

        return 1;


    }



    pub fn write(&mut self, block: &Block_ID, page: &Page) -> u8{
        let block_size = self.block_size;

        let file = match self.get_file(&block.file_name){
            Ok(file)    => file,
            _           => return 0,

        };

        //implement check for range here

        file.seek(SeekFrom::Start(u64::from(block_size) * u64::from(block.number)));

        let mut data = Vec::<u8>::new();

        page.Read(0, &mut data);

        file.write(&data);

        return 1
    }


// TODO
    pub fn close(&self){}


// TODO
    pub fn file_size(&self, fileName: &String) -> u64{
        return 0
    }

// 
// 
// fix getting the file from the hash map

    pub fn get_file(&mut self, file_name: &String) -> Result<&mut File, std::io::Error> {
        
        match self.opened_files.entry(file_name.to_string()) {
            Entry::Occupied(entry)  => Ok(entry.into_mut()),
            Entry::Vacant(entry)    =>{
                let path = format!("{}{}", self.data_directory, file_name);
                let file = File::options().read(true).write(true).open(&path)?;
                Ok(entry.insert(file))

            }

        }
    }
}

