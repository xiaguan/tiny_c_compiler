use log::info;

// The 'Stream' is input stream.
pub(crate) trait Stream {
    // Get next string from stream, if the stream is empty, return None
    fn next(&mut self) -> Option<String>;
}

/// `BasicStream` is for test right now.
/// It just wrap `Option<String>.
#[derive(Debug)]
pub struct BasicStream {
    // current string
    cur: Option<String>,
}

impl BasicStream {
    /// Create with a input string
    pub fn new_with_string(input: String) -> BasicStream {
        info!("create a stream with a input ${}$", input);
        BasicStream { cur: Some(input) }
    }
}

impl Stream for BasicStream {
    fn next(&mut self) -> Option<String> {
        self.cur.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_basic() {
        // Create a BasicStream with an input string
        let input_string = "Hello, World!".to_string();
        let mut stream = BasicStream::new_with_string(input_string.clone());

        // Test if the next method returns the correct string
        assert_eq!(stream.next(), Some(input_string));
        assert_eq!(stream.next(), None); // Stream should be empty now
    }
}
