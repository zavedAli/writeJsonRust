use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize,Deserialize)]

struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main(){
    let article: Article = Article {
        article: String::from("how to work json in rust"),
        author: String::from("akhil"),
        paragraph: vec![
            paragraph {
                name: String::from("first sentence")
            },
            paragraph {
                name: String::from("body of the paragraph")
            },
            paragraph {
                name: String::from("end of the paragraph")
            }

        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("the json is : {}",json)
}