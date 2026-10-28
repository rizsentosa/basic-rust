#![allow(dead_code)]

#[derive(Debug)]
pub struct InformationThirtedLibrary {
    title: String,
    version: u32,
    description: String,
    author: String,
    license: String,
}

impl InformationThirtedLibrary {
    pub fn new() -> Self {
        InformationThirtedLibrary {
            title: String::from("third-library"),
            version: 1,
            description: String::from(
                "Eiusmod eu est id nulla enim reprehenderit mollit consequat.",
            ),
            author: String::from("john le"),
            license: String::from("GPL"),
        }
    }

    pub fn third_library(&self) {
        println!("this is third library");
        println!("{:?}", self);
    }
}
