use std::io::Error;
use std::io::ErrorKind;

pub struct Page{
    bytes: Vec<u8>,
}

pub fn build_page(size: u16) -> Page{
    return Page{
        bytes: vec![0; usize::from(size)],
    }

}

impl Page{
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
}
