use std::io::Error;
use std::io::ErrorKind;
use super::block::Block_ID;

#[derive(Debug)]
pub struct Page{
    pub bytes:                  Vec<u8>,
    pub page_num:               u32,            //4 bytes
    pub page_type:              Page_type,      //1 byte
    pub previous_index:         Option<u32>,    //4 bytes
    pub next_index:             Option<u32>,    //4 bytes
    pub data_end_point:         u16,            //2 bytes
    pub record_index_end_point: u16,            //2 bytes   
                                                //
                                                //in total 17 bytes for page meta data

}

pub fn build_page(size: u16, page_num: u32, page_type: Page_type) -> Page{
    let x = match page_type{
        Page_type::Data            =>  size - 4,
        Page_type::Table_structure =>  size - 2,
    };

    return Page{
        bytes:                  vec![0; usize::from(size)],
        page_num:               0,
        page_type:              page_type,
        previous_index:         None,
        next_index:             None,
        data_end_point:         17, // meta data ends at byte 17 (index 16)
        record_index_end_point: x,


    }

}

#[derive(Debug)]
pub enum Page_type{
    Data,
    Table_structure,
    //...
}

impl Page{

    pub fn new(size: u16, page_num: u32, page_type: Page_type) -> Page{

        let x = match page_type{

            Page_type::Data            =>  size - 4,
            Page_type::Table_structure =>  size - 2,
        };

        return Page{
            bytes:                  vec![0; usize::from(size)],
            page_num:               0,
            page_type:              page_type,
            previous_index:         None,
            next_index:             None,
            data_end_point:         17, // meta data ends at byte 17 (index 16)
            record_index_end_point: x,


        }

        
        
        

    }

    pub fn write(&mut self, offset: u16, data: Vec<u8>) -> Result<u8, std::io::Error> {

        if  usize::from(offset) + data.len()     >   self.size(){
            return Err(Error::new(ErrorKind::Other, "Page size is too small for this data."))
        }

        self.bytes[ usize::from(offset) .. data.len() + usize::from(offset) ]
            .copy_from_slice(&data);

        return Ok(1);
    }


    pub fn read(&self, offset: u16, dst: &mut Vec<u8>) -> Result<u8, std::io::Error> {

        let end_point = dst.len();

        if( usize::from(offset) > self.size()){
            return Err(Error::new(ErrorKind::Other, "Offest exceeds page size.")) 

        }else if ( usize::from(offset)  +  end_point   >  self.size()  ){
            let end_point = self.size();
        }


        dst.copy_from_slice( &self.bytes[ usize::from(offset) .. end_point + usize::from(offset) ] );
        return Ok(1)
    }


    pub fn byte(&self) -> &Vec<u8>{
        return &self.bytes
    }


    pub fn size(&self) -> usize{
        return self.bytes.len()
    }




    pub fn get_record_index(&self) -> Vec<u16> {
        let slice = &self.bytes[(self.record_index_end_point as usize)..];

        let mut record_index: Vec<u16> = vec![];
        

        for i in (self.record_index_end_point as usize )..slice.len(){
            let mut big_endian = slice[i];
            let mut little_endian = slice[i+1];
            let result: u16 = ( (big_endian as u16) << 8) | little_endian as u16;
            record_index.push(result);
        }

        return record_index
    }





    pub fn add_record_index(&mut self, entry: u16){
        //add error checking
        //
        //thoughts: either shuffle all bytes to fill empty space or in this function find next
        //empty slot.

        self.record_index_end_point -= 2;
        let bytes: [u8; 2] = [(entry >> 8) as u8, entry as u8];

        self.bytes[self.record_index_end_point as usize] = bytes[0];
        self.bytes[(self.record_index_end_point + 1) as usize] = bytes[1];
    }

}
