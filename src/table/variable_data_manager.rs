//this handles all operations involving data pages for a specific table.
//Contains a sorted list of pages with free bytes ordered by size for efficient data manipulation
// Will update this every time data is added, removed or modified. Can also be used to retrieve a
// specific piece of data if required.
//
//
// each data page entry is made up of a 1 byte type, and 4 bytes size then the rest is the actual
// data.
//
// The data will be added to appropriate pages via worst-fit i.e. smaller data goes to the largest
// blocks of free space
//
//
// GOOD IDEA: instead of storing a reference to variable data's position in records, I can store an
// ID which links to a ID -> Location hashmap. Then, if a piece of data's location is changed, We
// only need to change the value in the table. I can also store size and type in the table
//
//
use std::collections::BinaryHeap;

use crate::file_manager::file_manager::File_manager;
use crate::file_manager::file_manager::build_file_manager;
use crate::file_manager::page::Page;
use crate::file_manager::page::build_page;
use crate::file_manager::block::Block_ID;
use crate::file_manager::page::Page_type;

use crate::buffer_pool::page_table::Page_table;

use crate::table::table::Table;
use crate::table::table::Data_type;
use crate::table::table::open_table;
use crate::table::variable_data_manager;

#[derive(Debug, Eq, PartialEq)]
pub struct Page_free {
    free: u16,
    page_num: u32,
}

impl Ord for Page_free{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.free.cmp(&other.free)
            .then_with(|| self.page_num.cmp(&other.page_num))

    }

}

impl PartialOrd for Page_free{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }

}

pub struct Variable_data_manager{

    pub free_bytes:                     BinaryHeap<Page_free>,
    //
    pub file_name:                      String,
    //
    pub last_data_page_num:             u32,
    //
    pub free_space_tracker_page_num:    u32,
    
}


impl Variable_data_manager{
//TODO performance cost of clone here..
    pub fn new(file_name: String, last_data_page_num: u32, free_space_tracker_page_num: &u32, page_table: &mut Page_table, file_manager: &mut File_manager ) -> Variable_data_manager{

        //retrieving the free byte tracker from the file.
        let mut free_bytes: BinaryHeap<Page_free> = BinaryHeap::new();
        let mut page = page_table.get_mut_page(Block_ID{file_name: file_name.clone(), number: *free_space_tracker_page_num}, file_manager).unwrap();//ERROR CHECKING
        let mut index: usize = 17;

        loop{
            let data_end_point = page.data_end_point.clone();
            if index > data_end_point as usize{ //MAGIC NUM
            

                if page.next_index == None || page.next_index == Some(0){
                    break;
                }else{
                    let next_index = page.next_index.unwrap();
                    page = page_table.get_mut_page(Block_ID{file_name: file_name.clone(), number: next_index}, file_manager).unwrap(); //ERROR CHECKING
                    index = 18;
                }

            }else{
                
                let page_num   =  u32::from_be_bytes(page.bytes[index..index+4].try_into().unwrap());
                let free_space =  u16::from_be_bytes(page.bytes[index+4 ..index+6].try_into().unwrap());

                free_bytes.push(Page_free{free: free_space, page_num: page_num});

                index += 6;
            }
        }
 
        return Variable_data_manager{
            free_bytes:                  free_bytes,
            file_name:                   file_name,
            last_data_page_num:          last_data_page_num,
            free_space_tracker_page_num: *free_space_tracker_page_num,

        }
    }

    fn allocate(heap: &mut BinaryHeap<Page_free>, data_size: u64) -> Option<u32>{
        let mut page = heap.pop()?;
        if (page.free as u64) < data_size {
            return None
        }

        page.free -= data_size as u16;
        let chosen = page.page_num;
        heap.push(page);
        Some(chosen)

    }

    fn shrink_top(heap: &mut BinaryHeap<Page_free>, amount: u16) -> Option<u8>{
        let mut page = heap.pop()?;
        if page.free < amount{
            
            return None; //ERROR CHECKING
        }

        page.free -= amount;
        heap.push(page);

        return Some(1);

    }
    
    pub fn add_data(&mut self, bytes: &[u8], page_table: &mut Page_table, file_manager: &mut File_manager){
    //get largest free space and compare. If data is smaller store it there
    
    let chosen = Self::allocate(&mut self.free_bytes, bytes.len().try_into().unwrap());


    if let Some(page_num) = chosen{
        let block = Block_ID{file_name: self.file_name.clone(), number: page_num};
        let mut page = page_table.get_mut_page(block.clone(), file_manager).unwrap(); //ERROR CHECKING
        page.write_at_end(bytes.to_vec());
        page_table.set_dirty(&block);
        Self::shrink_top(&mut self.free_bytes, bytes.len().try_into().unwrap());
        
    }else{
        let res = page_table.create_multiple_overflow_pages_by_data(bytes, Block_ID{file_name: self.file_name.clone(), number: self.last_data_page_num}, file_manager);
        self.last_data_page_num = res.0;
        //update the free space heap
        self.free_bytes.push(Page_free{free: res.1, page_num: res.0});


    }
    



    //otherwise make a new page and store the data there.



    }

    pub fn remove_data(&mut self, page_num: u32, index: u16, page_table: &mut Page_table, file_manager: &mut File_manager){
        //access the index point
        //
        //check the size of the data
        //
        //then begin the removal process, if the end of the data is before the end of a page then
        //remove all of that data by dereferncing it, and shifting all the data after back (make
        //sure to update all references to this data.)
        //
        //If the data continues after the end of the page then go to the overflow page and keep
        //removing data there until end of data or end of page is reached. Repeat that
        //
        //update the free bytes aswell after every page.

    }

    pub fn modify_data(&mut self, page_num: u32, index: u16){
        //similar to remove data but keep the required bits. If they are larger than the original
        //data then keep going and replacing data until end is reached. Then shift all data to an
        //overflow page and keep going. (All indexes must also be updated...)
        //
        //if modifying a piece of data at the end of a page and it overflows, make a new page and
        //store the original in there to avoid fragmentation.
    }

    pub fn get_data(&self, page_num: u32, index: u16){
        //go to index and read the required bytes.
    }





}
