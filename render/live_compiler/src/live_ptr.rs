#![allow(dead_code)]

use{
    makepad_live_tokenizer::LiveId,
    std::fmt,
};
 
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialOrd, Hash, PartialEq)]
pub struct LiveFileId(pub u16);

impl LiveFileId {
    pub fn index(index: usize) -> LiveFileId {LiveFileId(index as u16)}
    pub fn to_index(&self) -> usize {self.0 as usize}
}

//TODO FIX THIS THING TO BE N LEVELS OF MODULES
#[derive(Default, Clone, Eq, Hash, Debug, Copy, PartialEq)]
pub struct LiveModuleId(pub LiveId, pub LiveId);

impl LiveModuleId {
    pub fn from_str(module_path: &str) -> Result<Self,
    String> {
        // ok lets split off the first 2 things from module_path
        let bytes = module_path.as_bytes();
        let len = bytes.len();
        // we have to find the first :
        let mut crate_id = LiveId(0);
        let mut i = 0;
        while i < len {
            if bytes[i] == ':' as u8 {
                crate_id = LiveId::from_str(std::str::from_utf8(&bytes[0..i]).unwrap()) ?;
                i += 2;
                break
            }
            i += 1;
        }
        if i == len { // module_path is only one thing
            return Ok(LiveModuleId(LiveId(0), LiveId::from_str(std::str::from_utf8(&bytes[0..len]).unwrap()) ?));
        }
        return Ok(LiveModuleId(crate_id, LiveId::from_str(std::str::from_utf8(&bytes[i..len]).unwrap()) ?));
    }

}

impl fmt::Display for LiveModuleId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}", self.0, self.1)
    }
}
/*
#[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd, Copy, PartialEq)]
pub struct LocalPtr(pub usize);
*/
#[derive(Clone, Debug, Eq, Hash, Copy, Ord, PartialOrd, PartialEq)]
pub struct LivePtr {
    pub file_id: LiveFileId,
    pub index: u32,
}

impl LivePtr{
    pub fn node_index(&self)->usize{
        self.index as usize
    }
    
    pub fn with_index(&self, index:usize)->Self{
        Self{file_id:self.file_id, index:index as u32}
    }

    pub fn from_index(file_id:LiveFileId, index:usize)->Self{
        Self{file_id, index:index as u32}
    }

}

impl fmt::Display for LivePtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.file_id.0, self.index)
    }
}
