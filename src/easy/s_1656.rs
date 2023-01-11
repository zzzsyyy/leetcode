struct OrderedStream {
    stream: Vec<String>,
    ptr: usize,
}

impl OrderedStream {

    fn new(n: i32) -> Self {
        Self{
            stream: vec!["".to_string();n as usize +1],
            ptr: 1,
        }
    }
    
    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let id_key: usize = id_key as usize;
        self.stream[id_key] = value;
        let s = self.ptr;
        while self.ptr<self.stream.len() && self.stream[self.ptr]!=""{
            self.ptr+=1;
        }
        return self.stream[s..self.ptr].to_vec()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_1656(){
        assert!(OrderedStream::new(5).insert(3,"ccccc".to_string()).is_empty());
        assert_eq!(OrderedStream::new(5).insert(1,"aaaaa".to_string()),vec!["aaaaa".to_string()]);
    }
}
