use std::fs::File;

use log::info;

const BUFFER_SIZE: usize = 4096;

#[derive(Debug)]
pub(crate) struct StreamBuffer {
    pub(crate) buffer: [char; BUFFER_SIZE],
    // the index of the next char to be read
    pub(crate) read_index: usize,
    // the number of chars in the buffer
    pub(crate) count: usize,
}

// The trait 'StringStream' means we can get strings(in fact is char) like a stream
pub(crate) trait StringStream {
    // get next char from stream, if the stream is empty, return None
    // Because the scanner just read it , so we can return a reference to the char
    fn next_buffer(&mut self) -> Option<StreamBuffer>;
    // open a stream with a string , usuallly we use this method for test
    fn open_with_string(&mut self, string: String);
    // open a stream with a file, we can read data from file asynchoronously(maybe)
    fn open_with_file(&mut self, file: String);
}

/// Double buffer easy string stream
/// It has two buffer(BUFFER_SIZE) we can switch between them
/// When one buffer is empty, we can read data from file to the other buffer
/// and let read data to the empty buffer asynchoronously
#[derive(Debug)]
pub struct DoubleBufferStringStream {
    buffers: [Vec<char>; 2],
    consume_index: usize,
    // For the oepn_with_file method
    file: Option<File>,
    // For the open_with_string method
    string: Option<String>,
}

impl DoubleBufferStringStream {
    pub(crate) fn new() -> DoubleBufferStringStream {
        DoubleBufferStringStream {
            buffers: [Vec::new(), Vec::new()],
            consume_index: 0,
            file: None,
            string: None,
        }
    }

    /// new a stream with a string
    pub fn new_with_string(string: String) -> DoubleBufferStringStream {
        info!("create a stream with a string ${}$", string);
        let mut stream = DoubleBufferStringStream::new();
        stream.open_with_string(string);
        stream
    }
}

// impl the trait 'StringStream' for 'DoubleBufferStringStream'
impl StringStream for DoubleBufferStringStream {
    fn next_buffer(&mut self) -> Option<StreamBuffer> {
        // just support string method currently for test
        if let Some(string) = &self.string {
            let mut buffer = StreamBuffer {
                buffer: ['\0'; BUFFER_SIZE],
                read_index: 0,
                count: 0,
            };
            for (i, c) in string.chars().enumerate() {
                buffer.buffer[i] = c;
                buffer.count += 1;
            }
            Some(buffer)
        } else {
            None
        }
    }

    fn open_with_string(&mut self, string: String) {
        self.string = Some(string);
    }

    fn open_with_file(&mut self, file: String) {
        self.string = Some(file);
    }
}
