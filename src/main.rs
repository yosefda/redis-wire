use std::io::Read;
use std::str;

const SIMPLE_STRING_PREFIX: char = '+';

#[derive(Debug)]
struct SimpleStringType<'a> {
    data: &'a[u8],
}

#[derive(Debug)]
struct Error<'a> {
    data: &'a[u8],
}

#[derive(Debug)]
enum Response<'a> {
    SimpleString(SimpleStringType<'a>),
    Error(Error<'a>),
}

impl <'a> Response<'a> {
    fn from(input: &'a[u8]) -> Response<'a> {
        match input.bytes().nth(0).unwrap().unwrap() as char {
            SIMPLE_STRING_PREFIX => return Response::SimpleString(
                SimpleStringType {
                    data: input,
                }
            ) ,
            // @todo add more
            _ => return Response::Error(
                Error {
                    data: input,
                }
            ),
        }
    }

    fn parse(&self) -> String {
        match self {
            &Response::SimpleString(ref resp_type) => resp_type.parse(),
            _ => "Something".to_owned(),
        }
    }
}

impl <'a> SimpleStringType<'a> {
    fn parse(&self) -> String {
        str::from_utf8(self.data).unwrap()
            .replace('+',"").replace("\r\n","")
    }
}


fn main() {
    let response = Response::SimpleString(
        SimpleStringType {
            data: "+OK\r\n".as_bytes(),
        }
    );

    println!("{:?}", response.parse());
}




#[cfg(test)]
mod tests {
    use super::*;

    fn create_response(response: &str) -> &[u8] {
        response.as_bytes()
    }

    #[test]
    fn test_response_simplestring_from() {
        let response = Response::from(create_response("+OK\r\n"));
        assert!(
            match response {
                Response::SimpleString(_) => true,
                _ => false,
            }
        );
    }
}