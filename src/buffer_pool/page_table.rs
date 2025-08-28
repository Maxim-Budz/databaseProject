//implement multithreading and queue for page requests

use crate::file_manager::page::Page;
use crate::file_manager::block::Block_ID;
use crate::file_manager::file_manager::File_manager;

#[derive(Debug)]
pub struct Page_table_entry{
    page:       Page,
    block_ID:   Block_ID,
    pin_count:  u32,
    referenced: bool,
    dirty:      bool,

}

#[derive(Debug)]
pub struct Page_table{
    pub pages_in_memory: Vec< Option< Page_table_entry > >,
    pub clock_index: usize,

    pub page_table_size: u32,
    pub page_size: u32,
}


impl Page_table{




    pub fn new(total_size: u32, page_size: u32) -> Page_table{
        let vec_size = total_size / page_size;
        let mut vector = Vec::new();
        vector.resize_with(vec_size.try_into().unwrap(), || None);
        Page_table{
            pages_in_memory: vector,
            clock_index: 0,
            page_table_size: vec_size,
            page_size: page_size,
        }
        

    }
    



    pub fn write_to_disk(&mut self, index: u32, mut file_manager: File_manager) -> Result<u8, std::io::Error>{
        
        if let Some(entry) = self.pages_in_memory[index as usize].as_mut() {
           entry.pin_count += 1;
           let mut page = &entry.page;
           let block = &entry.block_ID;
           let result = file_manager.write(&block, &mut page);
           entry.pin_count -= 1;
           return result
           

        }else{
            return Ok(0)

        };
    }






    fn write_to_table(&mut self, new_block_ID: Block_ID, index:u32, file_manager: &mut File_manager) -> Result<u8, std::io::Error>{
        
        let entry = if let Some(entry) = self.pages_in_memory[index as usize].as_mut(){
            
            entry.dirty         = false;
            entry.pin_count     = 0;
            entry.referenced    = false;
            entry.block_ID      = new_block_ID;

            entry



        }else{
            let entry = Page_table_entry{
                            page:       Page::new(self.page_size),
                            block_ID:   new_block_ID,
                            pin_count:  0,
                            referenced: false,
                            dirty:      false,
                        };

            self.pages_in_memory[index as usize] = Some(entry);

            self.pages_in_memory[index as usize].as_mut().unwrap()


        };

        

        return file_manager.read(&entry.block_ID, &mut entry.page) 
    }






    pub fn request_new_page(&mut self, block_ID: Block_ID, file_manager: &mut File_manager)-> Result<u8, std::io::Error>{
        //add to queueu
        //
        //also check if it is already in there 
        //
        //then return the index of the page.

        let index = self.find_next_replaceable_page();

        println!("Index to be replaced: {:?}", index);

        return self.write_to_table(block_ID, index, file_manager)

    }







    pub fn find_next_replaceable_page(&mut self) -> u32{

        while true{
            self.clock_index = (self.clock_index + 1) % self.page_table_size as usize;
            let content = &self.pages_in_memory[self.clock_index];

            match content {
                None    =>  return self.clock_index.try_into().unwrap(),

                _   =>{

                    let content = content.as_ref().unwrap();

                    if content.pin_count == 0 && !content.referenced{
                        return self.clock_index.try_into().unwrap()

                    }else{
                        let mut referenced = content.referenced;
                        referenced = false;
                    };
                    },
            };

        }

        return 0;

    }





}



