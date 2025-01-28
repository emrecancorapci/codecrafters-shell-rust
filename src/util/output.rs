use std::{
    io::{Error, ErrorKind},
    process::Output,
};

pub trait SplitOutput {
    fn split_output(&self) -> (Option<Vec<u8>>, Option<Error>);
}

impl SplitOutput for Output {
    fn split_output(&self) -> (Option<Vec<u8>>, Option<Error>) {
        let mut output_array = self.stdout.to_vec();
        let mut error_array = self.stderr.to_vec();

        if output_array.last() == Some(&10) {
            output_array.pop();
        }
        if error_array.last() == Some(&10) {
            error_array.pop();
        }

        let output = if output_array.is_empty() {
            None
        } else {
            Some(output_array)
        };
        let error = if error_array.is_empty() {
            None
        } else {
            Some(Error::new(
                ErrorKind::Other,
                String::from_utf8(error_array).unwrap(),
            ))
        };

        return (output, error);
    }
}
