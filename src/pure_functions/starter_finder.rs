use std::error::Error;

use serde_yaml::{to_string, Value};

pub fn get_starter_data_from_list_by_name<'a>(
    starters: &'a Value,
    name_to_find: &'a str,
) -> Result<&'a Value, Box<dyn Error>> {

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

    let error_msg = format!("Couldn't find a starter named: {}", name_to_find);

    Err(error_msg.into())
}

#[cfg(test)]
mod finds_starter_from_list {
    use serde_yaml::{from_str, to_string, Value};
    use std::error::Error;

    #[test]
    fn extracts_starter_obj() -> Result<(), Box<dyn Error>> {
        use crate::pure_functions::starter_finder::get_starter_data_from_list_by_name;

        let name_to_find = "third";
        let expected_other_key_value = "3";
        let mock_starters: Value = from_str(
            r#"[ { name: first, other_key: 1 }, { name: second, other_key: 2 }, { name: third, other_key: 3 }, { name: fourth, other_key: 4 } ]"#,
        )?;

        // let mock_starters_seq = mock_starters.as_sequence().unwrap();

        let result = get_starter_data_from_list_by_name(&mock_starters, name_to_find)?;

        let result_other_key = to_string(result.get("other_key").unwrap())?;

        assert_eq!(result_other_key, format!("{expected_other_key_value}\n"));

        Ok(())
    }
}
