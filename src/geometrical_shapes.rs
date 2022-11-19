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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
        image.display(self.x, self.y, Point::color());
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
                image.display(x_arr[i], y_arr[i], color.clone());
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
                image.display(x_arr[i], y_arr[i], color.clone());
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

    fn loop_draw(&self, x_min: usize, x_max: usize, y_min: usize, y_max: usize) -> Vec<Point> {
        let mut v: Vec<Point> = vec![];
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let p = Point::new(x as i32, y as i32);
                if self.center.distance(&p) == self.r {
                    v.push(p);
                }
            }
        }
        return v;
    }

    fn reverse_x_axis(&self, v: &Vec<Point>) -> Vec<Point> {
        let np = v
            .iter()
            .map(|p| Point::new(self.center.x + (self.center.x - p.x), p.y))
            .collect::<Vec<Point>>();

        np
    }

    fn reverse_all(&self, v: &Vec<Point>) -> Vec<Point> {
        let np = v
            .iter()
            .map(|p| {
                Point::new(
                    self.center.x + (self.center.x - p.x),
                    self.center.y + self.center.y - p.y,
                )
            })
            .collect::<Vec<Point>>();

        np
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let color = Circle::color();

        let c_const: f64 = 2.0_f64.sqrt() / 2.0;
        let dist_const = self.r as f64 * c_const;

        // first half of quarter
        let (x_min, x_max) = (
            self.center.x as usize,
            self.center.x as usize + dist_const as usize,
        );
        let (y_min, y_max) = (
            self.center.y as usize + dist_const as usize,
            self.center.y as usize + self.r as usize,
        );

        let mut v1 = self.loop_draw(x_min, x_max, y_min, y_max);

        // second half of quarter
        let (x_min, x_max) = (
            self.center.x as usize + dist_const as usize,
            self.center.x as usize + self.r as usize,
        );
        let (y_min, y_max) = (
            self.center.y as usize,
            self.center.y as usize + dist_const as usize,
        );

        // create quarter of circle
        let v2 = self.loop_draw(x_min, x_max, y_min, y_max);
        v1.extend(v2);

        // create reversed by x axis quarter of circle
        let v2 = self.reverse_x_axis(&v1);
        v1.extend(v2);

        // create reversed by y axis quarter of circle
        let v2 = self.reverse_all(&v1);
        v1.extend(v2);

        for p in v1 {
            image.display(p.x, p.y, color.clone());
        }
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
            image.display(idx, self.x1.y, color.clone());
            image.display(idx, self.x2.y, color.clone());
        }

        for idx in if self.x1.y < self.x2.y {
            self.x1.y..self.x2.y
        } else {
            self.x2.y..self.x1.y
        } {
            image.display(self.x1.x, idx, color.clone());
            image.display(self.x2.x, idx, color.clone());
        }
    }
}
