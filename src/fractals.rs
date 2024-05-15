use crate::image::{Image, Pixel};
use crate::complex::Complex;
use crate::linspace::Linspace;
use wasm_bindgen::prelude::*;


/// For some ungodly reason I cannot use 
// where F:
//     F: Fn(Complex) -> Option<u16>
// I really don't like this
#[wasm_bindgen]
pub struct FractalGenerator {
    rstart: f64,
    rend:   f64,
    istart: f64,
    iend:   f64,
    max_iter: u16,
    image_buffer: Image,
    dirty: bool,
}


#[wasm_bindgen]
impl FractalGenerator {
    pub fn new(
            rstart: f64, rend: f64, istart: f64, iend: f64,     // dimensions on the complex plain
            max_iter: u16, xpcount: usize, ypcount: usize) -> Self {
        let image_buffer = Image::new(xpcount, ypcount);
        Self { rstart, rend, istart, iend, max_iter, image_buffer, dirty: false }
    }

    pub fn move_view(&mut self, rstart: f64, rend: f64, istart: f64, iend: f64) {
        self.rstart = rstart;
        self.rend = rend;
        self.istart = istart;
        self.iend = iend;
    }

    /// Creates an iterator over the real axis.
    fn reals(&self) -> Linspace {
        Linspace::new(self.rstart, self.rend, self.image_buffer.width)
    }

    /// Creates an iterator over the imaginary axis.
    fn imags(&self) -> Linspace {
        Linspace::new(self.istart, self.iend, self.image_buffer.height)
    }

    /// Generates the fractal.
    ///
    /// Generates color value for each pixel by calling self.formula for each pixel.
    /// Pease note there is some wired shit happening with the axis. 
    /// Linspace indexes axis accordingly to the Euclidian coordinate system.
    /// Buffer however for max cache efficiency needs to be index by increasing reals and DECREASING imags.
    /// This is the reason why imag iterator needs to be reversed()
    pub fn generate(&mut self) {
        if self.dirty {
            self.image_buffer.clear_buffer();
        }
        self.dirty = true;
        let mut comp = Complex::new(0.0, 0.0);
        for (yindex, imag) in self.imags().rev().enumerate() {
            for (xindex, real) in self.reals().enumerate() {
                comp.real = real;
                comp.imag = imag;
                if let Some(iter_count) = mandelbrot_formula(comp, self.max_iter) {
                    self.image_buffer.set_pixel(
                        xindex + yindex * self.image_buffer.width,
                        Pixel::from_rgb_tuple(hsl_to_rgb(iter_count))
                    );
                }
            }
        }
    }

    pub fn raw_pixels(&self) -> *const Pixel {
        self.image_buffer.raw_pixels()
    }

    pub fn save_to_file(&self, path: &str) {
        self.image_buffer.save(path).expect("saving to file failed");
    }
}


pub fn mandelbrot_formula(c: Complex, max_iter: u16) -> Option<u16> {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..max_iter {
        if z.modulus() >= 2.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}


/// Coloring function.
///
/// Takes a hue value and calculates rgb tuple for luminosity and saturation values of 1.
pub fn hsl_to_rgb(hue: u16) -> (u8, u8, u8){
    let hue = hue % 360;
    let x = 1.0 - (hue as f64 / 60.0 % 2.0 - 1.0).abs();
    let (rp, gp, bp) = match hue {
          0..=59  => (1.0, x  , 0.0),
         60..=119 => (  x, 1.0, 0.0),
        120..=179 => (0.0, 1.0, x  ),
        180..=239 => (0.0, x  , 1.0),
        240..=299 => (x  , 0.0, 1.0),
        300..=359 => (1.0, 0.0, x  ),
        _ => (1.0, 0.0, 0.0)
    };
    const INVERSE: bool = false;

    if INVERSE {
        return ((rp * 255.0) as u8, (gp * 255.0) as u8, (bp * 255.0) as u8);
    }
    (255 - (rp * 255.0) as u8, 255 - (gp * 255.0) as u8, 255 - (bp * 255.0) as u8)
}
