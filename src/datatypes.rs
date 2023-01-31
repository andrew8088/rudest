#[derive(Debug, PartialEq, Eq)]
pub enum DataType {
    SimpleString(String), // +
    RedisError,           // -
    Integer(usize),       // :
    BulkString(String),   // $
    Array(Vec<DataType>), // *
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
        ('$', Ok(body)) => to_bulk_string(body),
        (':', Ok(body)) => to_integer(body),
        ('*', Ok(body)) => to_array(body),
        _ => Err(format!("unknown command: {token}")),
    }
}

fn to_array(body: &str) -> Result<DataType, String> {}

fn to_string(body: &str) -> Result<DataType, String> {
    Ok(DataType::SimpleString(body.trim().to_string()))
}

fn to_bulk_string(body: &str) -> Result<DataType, String> {
    let lines = body.lines().collect::<Vec<_>>();

    let byte_count_str = lines[0];

    match byte_count_str.parse::<usize>() {
        Ok(byte_count) => {
            let bytes = lines[1];

            if bytes.len() != byte_count {
                return Err(format!("string {bytes} did not have {byte_count} bytes"));
            }

            Ok(DataType::BulkString(bytes.to_owned()))
        }
        Err(_) => Err(format!(
            "bulk array: could not parse byte count: {byte_count_str}"
        )),
    }
}

fn to_integer(body: &str) -> Result<DataType, String> {
    let trimmed_body = body.trim();

    match trimmed_body.parse::<usize>() {
        Ok(integer) => Ok(DataType::Integer(integer)),
        Err(_) => Err(format!("unparsable integer: {trimmed_body}")),
    }
}

#[cfg(test)]
mod tests {
    use crate::datatypes::{process_command, DataType, CRLF};

    #[test]
    fn it_parses_strings() {
        let input = format!("+OK{CRLF}");
        let out = process_command(input.as_bytes()).unwrap();
        if let DataType::SimpleString(body) = out {
            assert_eq!(body, "OK");
        } else {
            panic!("{:?} did not contain \"{}\"", out, "OK");
        }
    }

    #[test]
    fn it_parses_bulk_strings() {
        let input = format!("$7{CRLF}COMMAND{CRLF}");
        let out = process_command(input.as_bytes()).unwrap();
        if let DataType::BulkString(body) = out {
            assert_eq!(body, "COMMAND");
        } else {
            panic!("{:?} did not contain \"{}\"", out, "OK");
        }
    }

    #[test]
    fn it_handles_unparsable_bulk_strings_with_unparsable_byte_count() {
        let input = format!("$seven{CRLF}COMMAND{CRLF}");
        match process_command(input.as_bytes()) {
            Ok(_) => panic!("expected error"),
            Err(msg) => assert_eq!(msg, "bulk array: could not parse byte count: seven"),
        }
    }

    #[test]
    fn it_handles_unparsable_bulk_strings_with_wrong_byte_count() {
        let input = format!("$6{CRLF}COMMAND{CRLF}");
        match process_command(input.as_bytes()) {
            Ok(_) => panic!("expected error"),
            Err(msg) => assert_eq!(msg, "string COMMAND did not have 6 bytes"),
        }
    }

    #[test]
    fn it_parses_numbers() {
        let input = format!(":1000{CRLF}");
        let out = process_command(input.as_bytes()).unwrap();
        if let DataType::Integer(body) = out {
            assert_eq!(body, 1000);
        } else {
            panic!("{:?} did not contain \"{}\"", out, "OK");
        }
    }

    #[test]
    fn it_handles_unparsable_numbers() {
        let input = format!(":one{CRLF}");
        match process_command(input.as_bytes()) {
            Ok(_) => panic!("expected error"),
            Err(msg) => assert_eq!(msg, "unparsable integer: one"),
        }
    }

    #[test]
    fn it_parses_arrays() {
        let input = format!("*5{CRLF}:1{CRLF}:2{CRLF}:3{CRLF}:4{CRLF}$5{CRLF}hello{CRLF}");
        let out = process_command(input.as_bytes()).unwrap();
        if let DataType::Array(mut body) = out {
            assert_eq!(
                body.pop().unwrap(),
                DataType::BulkString("hello".to_owned())
            );
            // assert_eq!(body.pop().unwrap(), "hello");
        } else {
            panic!("{:?} did not contain \"{}\"", out, "OK");
        }
    }
}
