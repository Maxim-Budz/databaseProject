pub struct Page{
    bytes: Vec<u8>,
}

pub fn Build_page(size: u16) -> Page{
    return Page{
        bytes: vec![0; usize::from(size)],
    }

}

impl Page{
    pub fn Write(&mut self, offset: u16, data: Vec<u8>) -> i32 {
        if  usize::from(offset) + data.len()     >   self.Size(){
            return 0
        }

        self.bytes[ usize::from(offset) .. data.len() + usize::from(offset) ]
            .copy_from_slice(&data);

        return 1
    }


    pub fn Read(&self, offset: u16, dst: &mut Vec<u8>) -> i32{
        let end_point = dst.len();
        dst.copy_from_slice( &self.bytes[ usize::from(offset) .. end_point + usize::from(offset) ] );
        return 1

    }


    pub fn Byte(&self) -> &Vec<u8>{
        return &self.bytes
    }


    pub fn Size(&self) -> usize{
        return self.bytes.len()

    }

    




}
