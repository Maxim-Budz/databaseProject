use std::collections::HashMap;
use crate::file_manager::page::Page;
use crate::file_manager::block::Block_ID;
use std::fs::File;
use std::io::SeekFrom;
use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::path::Path;
use std::io;

pub struct File_manager{
    block_size: u64,
    data_directory: String,
    opened_files: HashMap<String,File>,

    //implement mutex
    
}

    pub fn build_file_manager(block_size: u64, data_directory: String) -> File_manager{
    File_manager{
        block_size,
        data_directory,
        opened_files: HashMap::new(),

    }
}



impl File_manager{


    //TODO ADD ERROR CHECKING.

    pub fn read(&self, block: &Block_ID, page: &mut Page) -> u8{


        let file = match self.get_file(&block.file_name){
            Ok(file)    => file,
            _           => return 0,
        };

        //implement check for range here

        file.seek(SeekFrom::Start(self.block_size * block.number));

        let mut write_buffer = vec![0; usize::from(self.block_size)];

        file.read(&mut write_buffer);

        page.Write(0, write_buffer.to_vec());

        return 1;


    }



    pub fn write(&self, block: &Block_ID, page: &Page) -> u8{

        let mut file = match self.get_file(&block.file_name){
            Ok(file)    => file,
            _           => return 0,

        };

        //implement check for range here

        file.seek(SeekFrom::Start(self.block_size * block.number));

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

    pub fn get_file(&self, file_name: &String) -> Result<File, io::Error>{
        let directory = self.data_directory.clone() + file_name;
        return {

                if self.opened_files.contains_key(file_name){
                    match self.opened_files.get_mut(file_name){
                        Some(file)  => Ok(file),
                        _           => Err("error  accessing file from hash table!"),
                    }
                    
                }else if Path::new(&directory).exists(){
                    File::open(&directory)
                }else{    
                    File::create(&directory)
                }
            }
        
        }

}
