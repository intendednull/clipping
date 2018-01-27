extern crate clipping;

use clipping::CPolygon;

fn main() {
  // two polygons
  let poly_a: Vec<[f64; 2]> = vec![[40., 34.], [200., 66.], [106., 80.], [120., 175.]];
  let poly_b = vec![[133., 120.], [80., 146.], [26., 106.], [40., 90.], [0., 53.], [80., 66.], [146., 0.]];

  // Get the clipping polygons
  let mut cp_a = CPolygon::from_vec(&poly_a);
  let mut cp_b = CPolygon::from_vec(&poly_b);

  // clip operation (intersection, union, difference)
  let cp_ab = cp_a.intersection(&mut cp_b);

  // handle the new polygons
  for poly_c in cp_ab{
    println!("Cliped polygon : {:?}", poly_c);
  }
}
