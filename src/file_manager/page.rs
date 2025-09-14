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

    pub fn remove_data_range(&mut self, from: u16, to: u16){
        //check if before record index end point and after the metadata bytes
        //
        // if it is ok then take a slice from (to) to record end point and move all bytes left
        // until they are at from.
        //
        // then set the record end point to -= (from - to). Dont care about the bytes after because
        // they will be overwritten in the future.
        let metadata_end_pointer = 16;
        if from < metadata_end_pointer || to + 1 > self.data_end_point{
            //out of range...
        }else{
            self.bytes.copy_within( to as usize .. ( self.data_end_point + 1) as usize , from as usize);
            self.data_end_point -= (to - from);

            
        }


    }



    pub fn get_record_index(&self) -> Vec<u16> {
        let slice = &self.bytes[(self.record_index_end_point as usize)..];

        let mut record_index: Vec<u16> = vec![];
        

        for i in (0.. slice.len() - 2 as usize).step_by(2){
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

    pub fn find_record_index(&self, value: u16) -> Option<u16>{
        let record_indexes = self.get_record_index();
        
        if value > record_indexes[0] || value < record_indexes[record_indexes.len()-1]{
            return None
        }else{
            let mut left_pointer = 0;
            let mut right_pointer = record_indexes.len() - 1;

            if value == record_indexes[right_pointer]{
                return Some(right_pointer as u16 * 2);
            }else if value == record_indexes[left_pointer]{
                return Some(left_pointer as u16 * 2);
            }

            loop{
                let middle_index = ((left_pointer + right_pointer) / 2) as usize;
                //println!("l: {}, m: {}, r: {}", left_pointer, middle_index, right_pointer);
                if value > record_indexes[middle_index]{
                    right_pointer = middle_index;

                }else if value < record_indexes[middle_index]{
                    left_pointer = middle_index;

                }else{
                    return Some(middle_index as u16 * 2)
                }

                if (left_pointer > right_pointer){
                    return None
                }else if left_pointer == right_pointer || left_pointer == right_pointer - 1{
                    
                    if record_indexes[left_pointer] == value{
                        return Some(left_pointer as u16 * 2)
                    }else if record_indexes[right_pointer] == value{
                        return Some(right_pointer as u16 * 2)
                        
                    }

                    else{
                        return None
                    }

                }
            }
        }
    }


    pub fn remove_record_index(&mut self, value: u16){
        let mut index = self.find_record_index(value).unwrap();

        // | 0, 0, | 0, 0 , | 1, 32 |
        //                       ^ index 
        //shift everything from record end point to (index - 2), 2 places forward but not to overwrite
        //the last bytes.
        //also if the index is the end index then don't shift anything.
        
        if index < 2{
            self.record_index_end_point += 2;
        }else{
            self.bytes.copy_within( (self.record_index_end_point as usize) .. ( self.record_index_end_point as usize + index  as usize) , (self.record_index_end_point + 2) as usize);
            self.record_index_end_point += 2;
        }
    }

    pub fn update_record_index_range(&mut self, from: u16, to: u16, incr: u16, positive: bool){
        let mut indexes = self.get_record_index();
        let from = from / 2;
        let to = to / 2;
        let mut slice = &indexes[from as usize .. to as usize];

        let src: Vec<u8> = if positive{
            slice
                .iter()
                .flat_map(|&n| (n + incr).to_be_bytes()) 
                .collect()
        }else{
            slice
                .iter()
                .flat_map(|&n| (n - incr).to_be_bytes())
                .collect()
        };

        let mut dst = &mut self.bytes[ (self.record_index_end_point + from) as usize .. (self.record_index_end_point + to * 2) as usize];

        dst.copy_from_slice(&src);


    }

    pub fn update_records_after(&mut self, record_value: u16, incr: u16, positive: bool){
        let location = match self.find_record_index(record_value){
            None    => return (), // UPDATE
            Some(n) => n,
        };
        self.update_record_index_range(0, location , incr, positive);

    }

    pub fn get_record_count_after(&mut self, value: u16) -> u16{
       let index =match self.find_record_index(value){
            None        => return 0,
            Some(num)   => num,
       };

       let total = self.get_record_index().len() as u16;

       return (total - (index / 2))
    }



}
