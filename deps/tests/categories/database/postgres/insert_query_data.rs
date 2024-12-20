// ANCHOR: example
use std::collections::HashMap;

use postgres::Client;
use postgres::Error;
use postgres::NoTls;

struct Author {
    _id: i32,
    name: String,
    country: String,
}

pub fn main() -> Result<(), Error> {
    // The connection URL is formatted as
    // postgresql://<user>:<password>@<host>/<db>, for example postgresql://
    // postgres:postgres@localhost/library
    let mut client = Client::connect(
        "postgresql://postgres:mysecretpassword@rust_howto_dev-postgres-1/library",
        NoTls,
    )?;

    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    for (key, value) in &authors {
        let author = Author {
            _id: 0,
            name: key.to_string(),
            country: value.to_string(),
        };

        client.execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            &[&author.name, &author.country],
        )?;
    }

    for row in client.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }

    Ok(())
}
// ANCHOR_END: example
