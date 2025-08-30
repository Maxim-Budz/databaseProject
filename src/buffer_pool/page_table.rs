//implement multithreading and queue for page requests

use crate::file_manager::page::Page;
use crate::file_manager::block::Block_ID;
use crate::file_manager::file_manager::File_manager;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Page_table_entry{
    pub page:       Page,
    pub pin_count:  u32,
    pub referenced: bool,
    pub dirty:      bool,

}

#[derive(Debug)]
pub struct Page_table{
    pub pages_in_memory: HashMap<Block_ID, Page_table_entry>,
    pub clock_index: usize,

    pub max_page_count: u32,
    pub page_size: u32,
}


impl Page_table{




    pub fn new(total_size: u32, page_size: u32) -> Page_table{
        let max_size = total_size / page_size;
        let mut map = HashMap::new();
        map.reserve(max_size.try_into().unwrap());
        
        Page_table{
            pages_in_memory: map,
            clock_index: 0,
            max_page_count: max_size,
            page_size: page_size,
        }
        

    }
    



    pub fn write_to_disk(&mut self, block: &Block_ID, file_manager: &mut File_manager) -> Result<u8, std::io::Error>{
        
        let fetch = self.pages_in_memory.get_mut(block);
        
        match fetch{

            Some(entry)    =>  {
                        entry.pin_count += 1;
                        let mut page = &entry.page;
                        let result = file_manager.write(block, &mut page);
                        entry.pin_count -= 1;
                        return result
                        },   

            None    =>  return Ok(0),

        }
    }






    fn replace_page(&mut self, new_block_ID: Block_ID, old_block_ID: Option<Block_ID>, file_manager: &mut File_manager) -> Result<u8, std::io::Error>{

        if old_block_ID.is_some(){
            
            let Some(old_block_ID) = old_block_ID else{return Ok(0)};
       
            let write_to_disk = {

                let fetch = self.pages_in_memory.get(&old_block_ID);
                
                match fetch{
                    
                    None => false,

                    Some(entry) => {
                            if entry.dirty{
                                true
                            }else{
                                false
                            }
                                },

                }
            };

            if write_to_disk{

                self.write_to_disk(&old_block_ID, file_manager)?;
            };

            self.pages_in_memory.remove(&old_block_ID);
        }





        let entry = Page_table_entry{
                        page:       Page::new(self.page_size),
                        pin_count:  0,
                        referenced: true,
                        dirty:      false,
                    };

        self.pages_in_memory.insert(new_block_ID.clone(), entry );


        return file_manager.read(&new_block_ID, &mut self.pages_in_memory.get_mut(&new_block_ID).unwrap().page)
    }






    pub fn request_new_page(&mut self, new_block_ID: Block_ID, file_manager: &mut File_manager)-> Result<u8, std::io::Error>{
        //add to queue
        //
        //also check if it is already in there 
        //
        //then return the index of the page.

        let block_to_replace = self.find_next_replaceable_page();

        //println!("Block to be replaced: {:?}", block_to_replace);

        return self.replace_page(new_block_ID, block_to_replace, file_manager)

    }







    pub fn find_next_replaceable_page(&mut self) -> Option<Block_ID>{

        //println!("max count: {:?}, page num in memory: {:?}", self.max_page_count, self.pages_in_memory.len());
        
        if self.max_page_count > self.pages_in_memory.len().try_into().unwrap(){
            return None
        }

        else{

            loop {

                for (block, content) in self.pages_in_memory.iter_mut(){
                    
                    //self.clock_index = (self.clock_index + 1) % self.page_table_size as usize;

                    if content.pin_count <= 0 && !content.referenced{
                        return Some(block.clone());

                    }else{
                        
                        content.referenced = false;
                    };
                };

            };

        };

        return None;

    }





}



