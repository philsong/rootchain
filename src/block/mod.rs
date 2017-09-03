pub struct Block{
    pub index: i64,
    pub nonce: i64,
    pub timestamp: String,
    pub data: String,
    pub hash: String,
    pub previous_hash: String,
}

impl Block{
    pub fn print_block(&self){
        println!("Index: {}", self.index);
        println!("Nonce: {}", self.nonce);
        println!("Timestamp: {}", self.timestamp);
        println!("Data: {}", self.data);
        println!("Hash: {}", self.hash);
        println!("Previous hash: {}", self.previous_hash);
        println!("----------------------------------------")
    }
}
