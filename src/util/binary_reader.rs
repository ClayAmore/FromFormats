use std::collections::VecDeque;
use std::io::{self, Error, ErrorKind};
use std::str;
use std::string::FromUtf8Error;

pub struct BinaryReader {
    pub(crate) big_endian: bool,
    steps: VecDeque<usize>,
    pub(crate) memory: Vec<u8>,
    pub(crate) position: usize,
}

impl BinaryReader {
    pub(crate) fn new(big_endian: bool, bytes: Vec<u8>) -> BinaryReader {
        BinaryReader {
            big_endian,
            steps: VecDeque::new(),
            memory: bytes,
            position: 0
        }
    }

    pub(crate) fn len(&mut self) -> usize {
        return self.memory.len();
    }

    pub(crate) fn skip(&mut self, length: usize) {
        self.position += length;
    }

    pub(crate) fn assert_value<T>(&mut self, value: T, options: &[T]) -> T
    where
        T: PartialEq + std::fmt::Debug,
    {
        if options.contains(&value) {
            value
        } else {
            panic!("Value not found in the list of options: {:?}", value);
        }
    }

    // READ
    pub fn read<T>(&mut self) -> T
    where
        T: Default {
            let size = std::mem::size_of::<T>();
            let memory_slice = &self.memory[self.position..self.position + size];
    
            let mut value = T::default();

            unsafe {
                let value_bytes = &mut value as *mut T as *mut u8;
        
                for i in 0..size {
                    value_bytes.add(i).write(memory_slice[i]);
                }
            }
    
            self.position += size;
            value
    }

    // Read multiple values
    pub fn read_multiple<T>(&mut self, count: usize) -> Result<Vec<T>, std::io::Error>
    where
        T: Default + Clone,
    {
        let size = std::mem::size_of::<T>();
        let mut values: Vec<T> = Vec::with_capacity(count);

        for _ in 0..count {
            let memory_slice = &self.memory[self.position..self.position + size];
            let mut value = T::default();

            unsafe {
                let value_bytes = &mut value as *mut T as *mut u8;

                for i in 0..size {
                    value_bytes.add(i).write(memory_slice[i]);
                }
            }

            self.position += size;
            values.push(value);
        }

        Ok(values)
    }

    // Read a span view of data
    pub fn read_span_view<T>(&mut self, count: usize) -> Result<&[T], Error>
    where
        T: Default,
    {
        let size = std::mem::size_of::<T>() * count;

        if self.position + size > self.memory.len() {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Reached end of memory while reading",
            ));
        }

        let data = &self.memory[self.position..self.position + size];
        self.position += size;

        let data_ptr = data.as_ptr() as *const T;
        let elements = data.len() / std::mem::size_of::<T>();

        // Safety: Converting a slice of bytes into a slice of type T.
        let typed_slice = unsafe { std::slice::from_raw_parts(data_ptr, elements) };

        Ok(typed_slice)
    }

    
    pub fn get_value<T, F>(&mut self, offset: usize, read_func: F) -> T
    where
        F: FnOnce(&mut BinaryReader) -> T,
    {
        self.step_in(offset);
        let result = read_func(self);
        self.step_out().expect("Error stepping out");
        result
    }
    
    pub fn step_in(&mut self, offset: usize) {
        self.steps.push_back(self.position);
        self.position = offset;
    }

    pub fn step_out(&mut self) -> Result<(), std::io::Error> {
        if self.steps.is_empty() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Reader is already stepped all the way out."));
        }

        self.position = self.steps.pop_back().unwrap();
        Ok(())
    }


    //************ Byte **************/ 

    pub(crate) fn assert_byte(&mut self, options: &[u8]) -> u8 {
        let value = self.read_byte();
        return self.assert_value(value, options);
    }

    pub(crate) fn read_byte(&mut self) -> u8 {
        self.read::<u8>()
    }

    pub(crate) fn get_byte(&mut self, offset: usize) -> u8 {
        let read_value = |reader: &mut BinaryReader| reader.read_byte();
        self.get_value(offset, read_value)
    }

    pub(crate) fn read_bytes(&mut self, count: usize) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(count);
        for _ in 0..count {
            bytes.push(self.read_byte());
        }
        bytes
    }

    pub(crate) fn get_bytes(&mut self, offset: usize, length: usize) -> Vec<u8> {
        self.step_in(offset);
        let result = self.read_bytes(length);
        self.step_out();
        return result;
    }


    //************ String **************/ 
    pub(crate) fn assert_ascii(&mut self, values: &[&str]) -> Result<String, Error> {
        // Read an ASCII string from the binary reader.
        let s = self.read_ascii(values[0].len())?;
    
        // Check if the read string matches any of the expected values.
        if !values.contains(&s.as_str()) {
            // If there's a mismatch, return an error with an error message.
            return Err(Error::new(
                io::ErrorKind::InvalidData,
                format!("Read ASCII: {} | Expected ASCII: {}", s, values.join(", ")),
            ));
        }
    
        // If the read string matches an expected value, return it as a String in the Ok variant.
        Ok(s)
    }

    pub(crate) fn read_chars(&mut self, encoding: &str, length: usize) -> Result<String, Error> {
        let bytes = self.read_bytes(length);
        let result = match encoding.to_lowercase().as_str() {
            "utf-8" => String::from_utf8(bytes),
            _ => return Err(Error::new(ErrorKind::InvalidData, "Unsupported encoding")),
        };
        
        match result {
            Ok(s) => Ok(s),
            Err(FromUtf8Error { .. }) => Err(Error::new(ErrorKind::InvalidData, "Decoding error")),
        }
    }

    pub(crate) fn read_ascii(&mut self, length: usize) -> Result<String, io::Error> {
        let bytes = self.read_bytes(length);
        let result = String::from_utf8(bytes);
        
        match result {
            Ok(s) => Ok(s),
            Err(FromUtf8Error { .. }) => Err(io::Error::new(io::ErrorKind::InvalidData, "Decoding error")),
        }
    }

    pub(crate) fn get_ascii(&mut self, offset: usize, length: usize) -> Result<String, io::Error> {
        self.step_in(offset);
        let result = self.read_ascii(length)?;
        self.step_out()?;
        Ok(result)
    }


    //************ i32 **************/ 
    pub(crate) fn assert_i32(&mut self, options: &[i32]) -> i32 {
        let value = self.read_i32();
        return self.assert_value(value, options);
    }

    pub(crate) fn read_i32(&mut self) -> i32 {
        if self.big_endian {
            let i = self.read::<i32>();
            return i.to_be();
        }
        self.read::<i32>()
    }

    pub(crate) fn get_i32(&mut self, offset: usize) -> i32 {
        let read_value = |reader: &mut BinaryReader| reader.read_i32();
        self.get_value(offset, read_value)
    }

}