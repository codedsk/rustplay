// https://mobiarch.wordpress.com/2015/06/29/understanding-lifetime-in-rust-part-i

pub struct TextEditor {
    text: String, // Private member variable
}

impl TextEditor {

    // Constuctor
    pub fn new() -> TextEditor {
        TextEditor{text: String::new()}
    }

    // Modify text
    pub fn add_char(&mut self, ch: char) {
        self.text.push(ch);
    }

    // Read text - Cloning
    pub fn get_text_clone(&self) -> String {
        return self.text.clone();
    }

    // Read text - Reference (implicit lifetime)
    pub fn get_text_ref1(&self) -> &String {
        return &self.text;
    }

    // Read text - Reference (explicit lifetime)
    pub fn get_text_ref2<'a>(&'a self) -> &'a String {
        return &self.text;
    }

    // Reset
    pub fn reset(&mut self) {
        self.text = String::new();
    }
}

fn main() {
        let mut editor = TextEditor::new();

        editor.add_char('a');
        editor.add_char('b');
        editor.add_char('c');

        let my_clone = editor.get_text_clone();
        println!("{}",my_clone);

        let my_ref1 = editor.get_text_ref1();
        println!("{}",my_ref1);

        let my_ref2 = editor.get_text_ref2();
        println!("{}",my_ref2);
}
