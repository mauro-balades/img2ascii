/*
| MIT License
| 
| Copyright (c) 2022 Mauro Baladés
| 
| Permission is hereby granted, free of charge, to any person obtaining a copy
| of this software and associated documentation files (the "Software"), to deal
| in the Software without restriction, including without limitation the rights
| to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
| copies of the Software, and to permit persons to whom the Software is
| furnished to do so, subject to the following conditions:
| 
| The above copyright notice and this permission notice shall be included in all
| copies or substantial portions of the Software.
| 
| THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
| IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
| FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
| AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
| LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
| OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
| SOFTWARE.
| 
*/

// -------------------
// IMG TO ASCII
// -------------------
// 
// How does it work?
//  1. We get image bits as a parameter.
//  2. An iteration is made for every pixel in 
//     the image. Note that we have a set of
//     character.
//
//     darkest -> brightest
//
//     We decide the brightness of a pixel by
//     making the average of the RGB value. RGB
//     values are store at numbers in the range
//     of 0 to 255. So, if we have rgb(255, 255, 255)
//     it is white and if we have rgb(0, 0, 0) it will
//     be black.
//
//     example pixel:
//
//        255 --------- | --------- 0
//                ^
//               137
//
//        We can see that this pixel has a brighter
//        value as it is closer to 255. We get the
//        average of those 3 numbers (ignoring Alpha)
//        and get the character index depending on the
//        average.

extern crate image;
use image::DynamicImage;
use image::GenericImageView;
use image::imageops::FilterType;


pub fn img2ascii(img: DynamicImage) -> () {
  // TODO: options struct
  // Options (default)
  let resolution = 5;

  // Create a new instance of a string.
  // Here is where the ascii art will
  // be stored as a result.
  let mut last_y = 0;
  let mut ascii_art = String::new();

  // Resize the image to a smaller version of it.
  let sm = img.resize(img.width() / resolution, img.height() / resolution, FilterType::Nearest);

  // Iterate for every pixel on the resized
  // image.
  for pixel in sm.pixels() {
    if last_y != pixel.1 {
      ascii_art.push_str("\n");
      last_y = pixel.1;
    }

    let pixel_data = pixel.2;
    let brightness:f64 = ((pixel_data[0] as u64 + pixel_data[1] as u64 + pixel_data[2] as u64) / 3) as f64;
  }
}

// - TESTS -

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn transparent_dog() {
        let img = image::open("./test/transparent_dog.jpg").unwrap();
        img2ascii(img);
        assert_eq!(3, 3);
    }
}
