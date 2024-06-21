#[derive(Debug)]
pub struct PageSource {
    files: Vec<Vec<String>>,
}

impl PageSource {
    pub fn new() -> Self {
        PageSource {
            files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, name: String, contents: String){
        let file = vec![name, contents];
        self.files.push(file);
    }

    pub fn get_files(&self) -> &Vec<Vec<String>>{
        &self.files
    }

    pub fn clear(&mut self){
        self.files.clear();
    }
}