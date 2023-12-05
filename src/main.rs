use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]

struct Paragraph {
    name:String
}
#[derive(Serialize, Deserialize)]

struct Components {
    article: String,
    author: String,
    paragraph : Vec<Paragraph>
}

fn main(){

    let chai = r#"
    {
        "article" : " We fell in love in a hopeless place",
        "author": "Ginger",
        "paragraph": [
            {
                "name" : "Chukwuma"
            },
            {
                "name": "ginger ginger"
            }
        ]
    }"#;


    let read = the_reader(chai);

    println!("\n\n this is to find the truth: {}", read.paragraph[0].name);

}

fn the_reader(chai : &str)-> Components{

    let parsed:Components = serde_json::from_str(chai).expect("there is a mistake somewhere");
    return parsed
}