//implement multithreading and queue for page requests

use crate::file_manager::page::Page;
use crate::file_manager::block::Block_ID;
use crate::file_manager::file_manager::File_manager;
use crate::file_manager::page::Page_type;

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
    pub page_size: u16,
}


impl Page_table{




    pub fn new(total_size: u32, page_size: u16) -> Page_table{
        let max_size = total_size / page_size as u32;
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
                        entry.dirty = false;
                        entry.pin_count -= 1;
                        return result
                        },   

            None    =>  return Ok(0),

        }
    }



    pub fn write_all(&mut self, file_manager: &mut File_manager){
        let blocks: Vec<_> = self.pages_in_memory.keys().cloned().collect();
        for block in blocks{
            self.write_to_disk(&block, file_manager);
        }
    
    }



    //creates a page in between two pages
    pub fn create_overflow_page(&mut self, old_block: &Block_ID, page_type: Page_type, next_page_num: u32, overflow_bytes: &[u8], file_manager: &mut File_manager){
        //create the page and fill in the appropriate data.
        
        //create a new page at the end of the file.
        let overflow_page_num = match file_manager.total_blocks(&old_block.file_name){
                            Err(e)  => return (),// TODO
                            Ok(x)   => x,
        };

        let mut overflow_page = Page::new(self.page_size, overflow_page_num, page_type);
        let byte_length = overflow_bytes.len() as u16;

        overflow_page.write(17, overflow_bytes.to_vec() );

        overflow_page.data_end_point += byte_length;

        overflow_page.update_record_index_range(self.page_size - 1, overflow_page.record_index_end_point, byte_length, true);

        overflow_page.set_next_page_num(next_page_num);

        overflow_page.set_previous_page_num(old_block.number);

        let mut prev_page = self.get_mut_page(old_block.clone(), file_manager).unwrap();

        prev_page.set_next_page_num(overflow_page_num);

        if next_page_num != 0{
            let mut next_page = self.get_mut_page(Block_ID{file_name: old_block.file_name.clone(), number: next_page_num}, file_manager).unwrap();
            next_page.set_previous_page_num(overflow_page_num);
        };

        self.add_page(overflow_page, &Block_ID{file_name: old_block.file_name.clone(), number: overflow_page_num}, file_manager);
    }






    fn replace_page(&mut self, new_block_ID: &Block_ID, old_block_ID: Option<Block_ID>, file_manager: &mut File_manager) -> Result<u8, std::io::Error>{

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
        //figure out what page type to store in memory





        let entry = Page_table_entry{
                        page:       Page::new(self.page_size, new_block_ID.number, Page_type::Data ),
                        pin_count:  0,
                        referenced: true,
                        dirty:      false,
                    };


        self.pages_in_memory.insert(new_block_ID.clone(), entry );

        return file_manager.read(&new_block_ID, &mut self.pages_in_memory.get_mut(&new_block_ID).unwrap().page)
    }




    pub fn get_mut_page(&mut self, block: Block_ID, file_manager: &mut File_manager) -> Option<&mut Page>{

       if !self.pages_in_memory.contains_key(&block){
            let res = self.request_new_page(&block, file_manager); 
       };

        let fetch = self.pages_in_memory.get_mut(&block);

        return match fetch{
        
            None => return None,

            Some(entry) => {
                    
                entry.pin_count += 1;
                entry.referenced = true;
                return Some(&mut entry.page)
                        },

       }
        

    }

    




    pub fn request_new_page(&mut self, new_block_ID: &Block_ID, file_manager: &mut File_manager)-> Result<u8, std::io::Error>{
        //add to queue
        //
        //also check if it is already in there 
        //
        //then return the index of the page.

        let block_to_replace = self.find_next_replaceable_page();

        //println!("Block to be replaced: {:?}", block_to_replace);

        return self.replace_page(new_block_ID, block_to_replace, file_manager)

    }


    //adds a page that is not already saved in a file.

    pub fn add_page(&mut self, page: Page, block: &Block_ID, file_manager: &mut File_manager) -> Result<u8, std::io::Error>{
        let block_to_replace = self.find_next_replaceable_page();
        
        if block_to_replace == None{

            
            let Some(block_to_replace) = block_to_replace else{return Ok(0)};
       
            let write_to_disk = {

                let fetch = self.pages_in_memory.get(&block_to_replace);
                
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

                self.write_to_disk(&block_to_replace, file_manager)?;
            };

            self.pages_in_memory.remove(&block_to_replace);

        }

        let entry = Page_table_entry{
            page:       page,
            pin_count:  0,
            referenced: true,
            dirty:      true,
        };


        self.pages_in_memory.insert(block.clone(), entry );
        
        return Ok(0);

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



