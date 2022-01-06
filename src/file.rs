use crate::unparse::Printer;
use syn::File;

impl Printer {
    pub fn file(&mut self, file: &File) {
        if let Some(shebang) = &file.shebang {
            self.word(shebang);
            self.hardbreak();
        }
        self.inner_attrs(&file.attrs);
        for item in &file.items {
            self.item(item);
        }
    }
}
