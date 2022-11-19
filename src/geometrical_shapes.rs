use rand::{distributions::range::SampleRange, Rng};
use raster::{Color, Image};

fn random_number<T: PartialOrd + SampleRange>(min: T, max: T) -> T {
    rand::thread_rng().gen_range(min, max)
}

pub trait Drawable {
    fn draw(&self, image: &mut Image);

    fn color() -> Color {
        Color {
            r: random_number(0, 255),
            g: random_number(0, 255),
            b: random_number(0, 255),
            a: 255,
        }
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color) {}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(max_w: i32, max_h: i32) -> Self {
        Self {
            x: random_number(0, max_w),
            y: random_number(0, max_h),
        }
    }

    fn distance(&self, other: &Point) -> i32 {
        let distance =
            (self.x as f64 - other.x as f64).powf(2.0) + (self.y as f64 - other.y as f64).powf(2.0);
        distance.sqrt().round() as i32
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.set_pixel(self.x, self.y, Point::color()).expect("");
    }
}

pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Self {
            start: Point::new(start.x, start.y),
            end: Point::new(end.x, end.y),
        }
    }

    pub fn random(max_w: i32, max_h: i32) -> Self {
        Self {
            start: Point::random(max_w, max_h),
            end: Point::random(max_w, max_h),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let x_abs = (self.start.x - self.end.x).abs() as usize;
        let y_abs = (self.start.y - self.end.y).abs() as usize;
        let color = Line::color();

        if x_abs < y_abs {
            let mut x_arr = vec![0; y_abs];
            let mut y_arr = vec![0; y_abs];
            let cf = x_abs as f64 / y_abs as f64;

            if self.start.y > self.end.y && self.start.x > self.end.x {
                for i in 0..y_abs {
                    y_arr[i] = self.end.y + i as i32;
                    x_arr[i] = self.end.x + (i as f64 * cf) as i32;
                }
            } else if self.start.y > self.end.y && self.start.x < self.end.x {
                for i in 0..y_abs {
                    y_arr[i] = self.end.y + i as i32;
                    x_arr[i] = self.start.x + (i as f64 * cf) as i32;
                }

                y_arr.reverse();
            } else if self.start.y < self.end.y && self.start.x < self.end.x {
                for i in 0..y_abs {
                    y_arr[i] = self.start.y + i as i32;
                    x_arr[i] = self.start.x + (i as f64 * cf) as i32;
                }
            } else {
                for i in 0..y_abs {
                    y_arr[i] = self.start.y + i as i32;
                    x_arr[i] = self.end.x + (i as f64 * cf) as i32;
                }

                x_arr.reverse();
            }

            for i in 0..y_abs {
                image
                    .set_pixel(x_arr[i], y_arr[i], color.clone())
                    .expect("");
            }
        } else {
            let mut x_arr = vec![0; x_abs];
            let mut y_arr = vec![0; x_abs];
            let cf = y_abs as f64 / x_abs as f64;

            if self.start.y > self.end.y && self.start.x > self.end.x {
                for i in 0..x_abs {
                    y_arr[i] = self.end.y + (i as f64 * cf) as i32;
                    x_arr[i] = self.end.x + i as i32;
                }
            } else if self.start.y > self.end.y && self.start.x < self.end.x {
                for i in 0..x_abs {
                    y_arr[i] = self.end.y + (i as f64 * cf) as i32;
                    x_arr[i] = self.start.x + i as i32;
                }

                y_arr.reverse();
            } else if self.start.y < self.end.y && self.start.x < self.end.x {
                for i in 0..x_abs {
                    y_arr[i] = self.start.y + (i as f64 * cf) as i32;
                    x_arr[i] = self.start.x + i as i32;
                }
            } else {
                for i in 0..x_abs {
                    y_arr[i] = self.start.y + (i as f64 * cf) as i32;
                    x_arr[i] = self.end.x + i as i32;
                }

                x_arr.reverse();
            }

            for i in 0..x_abs {
                image
                    .set_pixel(x_arr[i], y_arr[i], color.clone())
                    .expect("");
            }
        }
    }
}

pub struct Circle {
    center: Point,
    r: i32,
}

impl Circle {
    pub fn new(center: &Point, r: i32) -> Self {
        Self {
            center: Point::new(center.x, center.y),
            r,
        }
    }

    pub fn random(max_w: i32, max_h: i32) -> Self {
        Self {
            center: Point::random(max_w, max_h),
            r: rand::thread_rng().gen_range(0, max_w),
        }
    }
}

impl Drawable for Circle {
    // fn draw(&self, image: &mut Image) {
    //     // let circle_len = (2_f64 * self.r as f64 * std::f64::consts::PI) as usize;
    //     let mut points_arr: Vec<Point> = Vec::new();

    //     let (x_min, x_max) = (self.center.x - self.r, self.center.x + self.r);
    //     let (y_min, y_max) = (self.center.y - self.r, self.center.y + self.r);

    //     let (x_min, x_max) = (
    //         if 0 > x_min { 0 } else { x_min },
    //         if image.width < x_max {
    //             image.width
    //         } else {
    //             x_max
    //         },
    //     );

    //     let (y_min, y_max) = (
    //         if 0 > y_min { 0 } else { y_min },
    //         if image.height < y_max {
    //             image.height
    //         } else {
    //             y_max
    //         },
    //     );

    //     // loop {

    //     for x in x_min..x_max {
    //         for y in y_min..y_max {
    //             let new_point = Point::new(x, y);

    //             if new_point.distance(&self.center) != self.r {
    //                 continue;
    //             }

    //             if points_arr.contains(&new_point) {
    //                 continue;
    //             }

    //             points_arr.push(new_point);
    //         }
    //     }

    //     // let new_point = Point::new(
    //     //     rand::thread_rng().gen_range(x_min, x_max),
    //     //     rand::thread_rng().gen_range(y_min, y_max),
    //     // );

    //     // println!(
    //     //     "center - {:?}, r - {}, {:?}",
    //     //     self.center, self.r, new_point
    //     // );

    //     // if points_arr.contains(&new_point) {
    //     //     continue;
    //     // }

    //     // if new_point.distance(&self.center) <= self.r + 1
    //     //     && new_point.distance(&self.center) >= self.r - 1
    //     // {
    //     //     points_arr.push(new_point);
    //     // }

    //     // if points_arr.len() == circle_len {
    //     //     break;
    //     // }
    //     // }

    //     // println!("{:?}", points_arr);

    //     let color = Circle::color();

    //     for pp in points_arr {
    //         image
    //             .set_pixel(pp.x, pp.y, color.clone())
    //             .unwrap_or_default();
    //     }
    // }

    fn draw(&self, image: &mut Image) {
        let circle_len = 2.0 * self.r as f64 * std::f64::consts::PI;
        let color = Circle::color();

        // reverse calculus to more accurate
        let cf = 360.0 / circle_len;
        let cf = if cf > 1.0 { 1.0 } else { cf };

        println!("{cf}, {}, {}", self.r, circle_len);

        let mut degree = 0.0_f64;
        loop {
            // let radians = degree * std::f64::consts::PI / 180.0;
            let x = self.r as f64 * degree.cos() + self.center.x as f64;
            let y = self.r as f64 * degree.sin() + self.center.y as f64;

            let pp = Point::new(x as i32, y as i32);

            // let d = pp.distance(&self.center);

            // println!("{:?}\n{:?}", self.center, pp);
            // println!("{}, {}", d, self.r);
            // println!();

            image
                .set_pixel(pp.x, pp.y, color.clone());

            degree += cf;

            if degree >= 360.0 {
                break;
            }
        }

        // for pp in points_arr {
        // }
    }
}

pub struct Triangle {
    x1: Point,
    x2: Point,
    x3: Point,
}

impl Triangle {
    pub fn new(x1: &Point, x2: &Point, x3: &Point) -> Self {
        Self {
            x1: Point::new(x1.x, x1.y),
            x2: Point::new(x2.x, x2.y),
            x3: Point::new(x3.x, x3.y),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let line1 = Line::new(&self.x1, &self.x2);
        let line2 = Line::new(&self.x2, &self.x3);
        let line3 = Line::new(&self.x3, &self.x1);
        line1.draw(image);
        line2.draw(image);
        line3.draw(image);
    }
}

pub struct Rectangle {
    x1: Point,
    x2: Point,
}

impl Rectangle {
    pub fn new(x1: &Point, x2: &Point) -> Self {
        Self {
            x1: Point::new(x1.x, x1.y),
            x2: Point::new(x2.x, x2.y),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let color = Rectangle::color();
        for idx in if self.x1.x < self.x2.x {
            self.x1.x..self.x2.x
        } else {
            self.x2.x..self.x1.x
        } {
            image.set_pixel(idx, self.x1.y, color.clone()).expect("");
            image.set_pixel(idx, self.x2.y, color.clone()).expect("");
        }

        for idx in if self.x1.y < self.x2.y {
            self.x1.y..self.x2.y
        } else {
            self.x2.y..self.x1.y
        } {
            image.set_pixel(self.x1.x, idx, color.clone()).expect("");
            image.set_pixel(self.x2.x, idx, color.clone()).expect("");
        }
    }
}
