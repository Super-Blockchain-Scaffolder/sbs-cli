use std::error::Error;
use std::io::ErrorKind;

use serde_yaml::{to_string, Value};

pub fn get_starter_data_from_list_by_name<'a>(
    starters: &'a Value,
    name_to_find: &'a str,
) -> Result<&'a Value, Box<dyn Error>> {
    // println!("starters: {:?}", starters);

    let starters_seq = starters.as_sequence().unwrap();

    for s in starters_seq {
        println!("starter: {:?}", s);

        let starter_iterator_name: &str = &to_string(s.get("name").unwrap()).unwrap();
        
        println!("starter iterator name: {:?}", starter_iterator_name);
        println!("name_to_find: {:?}", name_to_find);

        if name_to_find == starter_iterator_name.trim_end_matches("\n") {
            println!("found it!");
            return Ok(s);
        }
    }

    // Err(Box<Error::new("Foo")>)
    // Ok(())
    // Err(Error::new(ErrorKind::Other, "Something went wrong"))

    let error_msg = format!("Couldn't find a starter named: {}", name_to_find);

    Err(error_msg.into())
    // err!("foo")
}
