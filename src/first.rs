#![allow(dead_code)]

use crate::third::InformationThirtedLibrary;

#[derive(Debug)]
struct InformationSecondLibrary {
    title: String,
    version: u32,
    description: String,
    author: String,
    license: String,
}

pub fn first_library() {
    println!("this is second lib");
    let lib3 = InformationThirtedLibrary::new();
    lib3.third_library();
}

pub mod seconded {
    pub mod thirded {
        pub fn call_library() {
            let info = super::super::InformationSecondLibrary {
                title: String::from("second-library"),
                version: 1,
                description: String::from(
                    "Eiusmod eu est id nulla enim reprehenderit mollit consequat.",
                ),
                author: String::from("john le"),
                license: String::from("GPL"),
            };
            println!("{:?}", info);
        }
    }
}
