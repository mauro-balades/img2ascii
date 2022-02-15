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
use image::imageops::FilterType;

   
pub fn img2ascii(DynamicImage img) -> String {
  // TODO: options struct
  // Options (default)
  let resolution = 5;

  // Create a new instance of a string.
  // Here is where the ascii art will
  // be stored as a result.
  let ascii_art = String::new()


  let sm = img.resize(img.width() / resolution, img.height() / resolution, FilterType::Nearest);
}
