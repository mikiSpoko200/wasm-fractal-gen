#[allow(dead_code)]

use std::io::Write;

/// Struct that represents a 32 bit RGBA pixel.
#[derive(Copy, Clone, Debug, std::cmp::PartialEq)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alfa: u8,
}

impl Pixel {
    /// Creates Pixel with specified values of red, green, blue.
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue, alfa: 255 }
    }

    /// Creates Pixel from tuple representation.
    ///
    /// This function treats 3 element tuple as sequence of as values. red, green, blue.
    pub fn from_rgb_tuple(rgb: (u8, u8, u8)) -> Self {
        let (red, green, blue) = rgb;
        Self::new(red, green, blue)
    }

    /// Creates Pixel from string representation.
    ///
    /// Function expects a string with three numeric values convertible to u8
    /// String can have arbitrary number of leading and trailing whitespaces as well as any number
    /// of spaces in between the numbers themselves.
    pub fn from_string(rgb: &str) -> Self {
        let numbers = rgb
            .split_whitespace()
            .map(|number| number.parse().unwrap()).collect::<Vec<u8>>();
        match &numbers[..] {
            &[red, green, blue] => { Pixel::new(red, green, blue) },
            _ => {
                panic!("Too many values passed in Pixel string representation. Expected 3 got {}", numbers.len());
            },
        }
    }

    
    /// Returns Pixel representation used in primitive plain ppm format.
    pub fn plain_ppm_pixel_format(&self) -> String {
        format!(" {} {} {} ", self.red, self.green, self.blue)
    }
}

/// Test suite for Pixel Struct.

impl Default for Pixel {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

#[cfg(test)]
mod test_pixel {
    use super::*;

    /// Test Pixel::from_string
    #[test]
    fn pixel_from_string() {
        assert_eq!(Pixel::from_string("0   0 0"), Pixel::new(0, 0, 0));
        assert_eq!(Pixel::from_string("   1 2 3 "), Pixel::new(1, 2, 3));
        assert_eq!(Pixel::from_string("0 0  0   "), Pixel::new(0, 0, 0));
        assert_eq!(Pixel::from_string("  255 255   255   "), Pixel::new(255, 255, 255));
    }

    #[test]
    #[should_panic]
    fn pixel_value_out_of_range() {
        Pixel::from_string("256 0 0");
        Pixel::from_string("-1 42 13");
    }


    #[test]
    #[should_panic]
    fn pixel_too_few_numbers() {
        Pixel::from_string("0 0");
    }

    #[test]
    #[should_panic]
    fn pixel_too_many_numbers() {
        Pixel::from_string("0 0 0 0");
    }
}

/// Struct that represents a 2D image.
///
/// Image can have a width, height and buffer of pixels that create the image.
pub struct Image {
    pub width:  usize,
    pub height: usize,
    buffer: Vec<Pixel>,
}


impl Image {
    /// Creates Image with dimensions specified.
    pub fn new(width: usize, height: usize) -> Self {
        println!("width: {}, height: {}", width, height);
        Self {
            width, height,
            buffer: vec![Pixel::default(); width * height],
        }
    }

    /// Setter for the red bit of pixel with specified index.
    pub fn set_red(&mut self, index: u64, value: u8) {
        self.buffer[index as usize].red = value;
    }

    /// Setter for the green bit of pixel with specified index.
    pub fn set_green(&mut self, index: u64, value: u8) {
        self.buffer[index as usize].green = value;
    }

    /// Setter for the blue bit of pixel with specified index.
    pub fn set_blue(&mut self, index: u64, value: u8) {
        self.buffer[index as usize].blue = value;
    }

    /// Setter for specific pixel.
    pub fn set_pixel(&mut self, index: usize, pixel: Pixel) {
        self.buffer[index] = pixel;
    }

    /// Returns Image representation in plain ppm format.
    pub fn plain_ppm_format(&self) -> String {
        let pixel_repr_len =
            Pixel::new(255, 255, 255)
            .plain_ppm_pixel_format()
            .len();
        let pixels_per_line = 70 / pixel_repr_len;
        let formatted_pixels = self
            .buffer.iter().enumerate()
            .map(|(index, pixel)|
                pixel.plain_ppm_pixel_format() +
                    if index % pixels_per_line == pixels_per_line - 1 {"\n"} else {""} )
            .collect::<String>();
        String::from(format!("P3\n{} {}\n{}\n{}", self.width, self.height, u8::MAX, formatted_pixels))
    }

    /// Returns a pointer to image buffer.
    pub fn raw_pixels(&self) -> *const Pixel {
        self.buffer.as_ptr()
    }

    /// Clears the pixel buffer.
    pub fn clear_buffer(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = Pixel::default();
        }
    }

    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let mut file_handle = std::fs::File::create(path)?;
        write!(file_handle, "{}", &self.plain_ppm_format()[..])?;
        Ok(())
    }
}
