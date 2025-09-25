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

    return Page{
        bytes:                  vec![0; usize::from(size)],
        page_num:               page_num,
        page_type:              page_type,
        previous_index:         None,
        next_index:             None,
        data_end_point:         17, // meta data ends at byte 17 (index 16)
        record_index_end_point: size-2,


    }

}

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Page_type{
    Table_structure = 1,
    Record = 2,
    B_tree = 3,
    Data = 4,
    Free_space_tracker = 5,

    
}

//Page metadata:
//
// 0 0 0 0 | 0 | 0 0 0 0 | 0 0 0 0 | 0 0 | 0 0 
// page num  |      |         |       |     |
//          type    |         |       |     |
//              prev page     |       |     |
//                        next page   |     |
//                                data end  |
//                                        record end
//
//
// when page is written to disk, page table ensures the meta data is written correctly.
impl Page{

    pub fn new(size: u16, page_num: u32, page_type: Page_type) -> Page{

        return Page{
            bytes:                  vec![0; usize::from(size)],
            page_num:               page_num,
            page_type:              page_type,
            previous_index:         None,
            next_index:             None,
            data_end_point:         17, // meta data ends at byte 17 (index 16)
            record_index_end_point: size - 2,


        }

    }

    pub fn set_data_end_point(&mut self, value: u16){
        self.data_end_point = value;
        let bytes = value.to_be_bytes();
        self.write(13, bytes.to_vec());
    }

    pub fn set_record_index_end_point(&mut self, value: u16){
        self.record_index_end_point = value;
        let bytes = value.to_be_bytes();
        self.write(15, bytes.to_vec());
    }

    pub fn set_previous_page_num(&mut self, value: u32){
        self.previous_index = Some(value);
        let bytes = value.to_be_bytes();
        self.write(5, bytes.to_vec());
    }

    pub fn set_next_page_num(&mut self, value: u32){
        self.next_index = Some(value);
        let bytes = value.to_be_bytes();
        self.write(9, bytes.to_vec());


    }




    pub fn write(&mut self, offset: u16, data: Vec<u8>) -> Result<u8, std::io::Error> {

        if  usize::from(offset) + data.len()     >   self.size(){
            return Err(Error::new(ErrorKind::Other, "Page size is too small for this data."))
        }

        self.bytes[ usize::from(offset) .. data.len() + usize::from(offset) ]
            .copy_from_slice(&data);

        return Ok(1);
    }

    pub fn write_at_end(&mut self, data: Vec<u8>){
        let amount = data.len();
        if amount > (self.record_index_end_point - self.data_end_point).into(){
            return ()
        };
        self.write(self.data_end_point, data);
        self.data_end_point += amount as u16;

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

        let metadata_end_pointer = 16;

        if from < metadata_end_pointer{
            //out of range...
        }else if to >= self.data_end_point{
            self.data_end_point -= (to - from);


        }else{
            
            self.bytes.copy_within( to as usize .. ( self.data_end_point + 1 ) as usize , from as usize);
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
        println!("{:?}", self);
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
