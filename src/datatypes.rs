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
pub fn process_command(command: &[u8]) -> Result<DataType, String> {
    let token = command[0] as char;
    let rest = std::str::from_utf8(&command[1..]);

    match (token, rest) {
        ('+', Ok(body)) => to_string(body),
        (':', Ok(body)) => to_integer(body),
        _ => Err(format!("unknown command: {}", token)),
    }
}

fn to_string(body: &str) -> Result<DataType, String> {
    Ok(DataType::SimpleString(body.trim().to_string()))
}

fn to_integer(body: &str) -> Result<DataType, String> {
    let trimmed_body = body.trim();

    match trimmed_body.parse::<usize>() {
        Ok(integer) => Ok(DataType::Integer(integer)),
        Err(_) => Err(format!("unparsable integer: {}", trimmed_body)),
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

    #[test]
    fn it_handles_unparsable_numbers() {
        let input = format!(":one{}", CRLF);
        match process_command(input.as_bytes()) {
            Ok(_) => panic!("expected error"),
            Err(msg) => assert_eq!(msg, "unparsable integer: one"),
        }
    }
}
