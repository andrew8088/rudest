#[derive(Debug, PartialEq, Eq)]
pub enum DataType {
    SimpleString(String), // +
    RedisError,           // -
    Integer(usize),       // :
    BulkString,           // $
    Array,                // *
}

pub const CRLF: &str = "\r\n";

// impl TryFrom<char> for DataType {
//     type Error = String;

//     fn try_from(value: char) -> Result<Self, Self::Error> {
//         match value {
//             '+' => Ok(DataType::SimpleString),
//             '-' => Ok(DataType::RedisError),
//             ':' => Ok(DataType::Integer),
//             '$' => Ok(DataType::BulkString),
//             '*' => Ok(DataType::Array),
//             _ => Err(format!("char [{}] is not a valid redis data type", value)),
//         }
//     }
// }

pub fn process_command(command: &[u8]) -> Option<DataType> {
    let token = command[0] as char;
    let rest = std::str::from_utf8(&command[1..]);

    match (token, rest) {
        ('+', Ok(body)) => Some(DataType::SimpleString(body.trim().to_string())),
        ('-', Ok(body)) => Some(DataType::SimpleString(body.trim().to_string())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::datatypes::{process_command, DataType, CRLF};

    #[test]
    fn it_parses_strings() {
        let input = format!("+OK{}", CRLF);
        let out = process_command(input.as_bytes()).unwrap();
        if let DataType::SimpleString(body) = out {
            assert_eq!(body, "OK");
        } else {
            panic!("{:?} did not contain \"{}\"", out, "OK");
        }
    }

    #[test]
    fn it_parses_numbers() {
        let input = format!(":1000{}", CRLF);
        let out = process_command(input.as_bytes()).unwrap();
        if let DataType::Integer(body) = out {
            assert_eq!(body, 1000);
        } else {
            panic!("{:?} did not contain \"{}\"", out, "OK");
        }
    }
}
