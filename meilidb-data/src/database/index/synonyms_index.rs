use crate::RocksDbResult;

#[derive(Clone)]
pub struct SynonymsIndex(pub(crate) crate::CfTree);

impl SynonymsIndex {
    pub fn alternatives_to(&self, word: &[u8]) -> RocksDbResult<Option<fst::Set>> {
        match self.0.get(word)? {
            Some(vector) => Ok(Some(fst::Set::from_bytes(vector.to_vec()).unwrap())),
            None => Ok(None),
        }
    }

    pub fn set_alternatives_to(&self, word: &[u8], value: Vec<u8>) -> RocksDbResult<()> {
        self.0.insert(word, value).map(drop)
    }

    pub fn del_alternatives_of(&self, word: &[u8]) -> RocksDbResult<()> {
        self.0.remove(word).map(drop)
    }
}