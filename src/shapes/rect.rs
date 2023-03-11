use super::area::Area;
use std::fmt::Display;


pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
};

impl Rectangle {
    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        return x >= self.x && x <= self.x + self.width &&
            y >= self.y && y <= self.y + self.height;
    }
}

// impl Collidable<Circle> for Rectangle {
//     fn collide(&self, other: &Circle) -> bool {
//         return other.collide(self);
//     }
// }

// impl Collidable<Rectangle> for Rectangle {
//     fn collide(&self, other: &Rectangle) -> bool {
//         for point in other {
//             if self.contains_point(point) {
//                 return true;
//             }
//         }
//         return false;
//     }
// }


impl Area for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Default for Rect {
    fn default() -> Self {
        return Rect {
        x: 0f64, 
        y: 0f64,
        width: 10f64,
        height: 20f64,
        }
    }
};

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Rectangle({}, {}), {}x{}",
            self.x, self.y, self.width, self.height
        );
    }
};

// struct RectIter {
//     points: Vec<(f64, f64)>,
//     idx: usize,
// };

// impl Iterator for RectIter {
//     type Item = (f64, f64);

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.idx >= self.points.len() {
//             return None;
//         }

//         let point = self.points[self.idx];
//         self.idx += 1;

//         return Some(point);
//     }
// }

// impl IntoIterator for Rectangle {
//     type Item = (f64, f64);

//     type IntoIter = RectIter;

//     fn into_iter(self) -> Self::IntoIter {
//         return RectIter {
//             points: vec![
//                 (self.x, self.y),
//                 (self.x + self.width, self.y),
//                 (self.x, self.y + self.height),
//                 (self.x + self.width, self.y + self.height),
//             ],
//             idx: 0,
//         }
//     }
// }

//! \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
// Iteration
// pub struct RectIter {
//     points: [(f64, f64); 4],
//     idx: usize,
// }

// impl From<&Rectangle> for RectIter {
//     fn from(rect: &Rectangle) -> Self {
//         return RectIter {
//             points: vec![
//                 (rect.x, rect.y),
//                 (rect.x + rect.width, rect.y),
//                 (rect.x, rect.y + rect.height),
//                 (rect.x + rect.width, rect.y + rect.height),
//             ],
//             idx: 0,
//         }
//     }
// }

// impl IntoIterator for Rectangle {
//     type Item = (f64, f64);

//     type IntoIter = RectIter;

//     fn into_iter(self) -> Self::IntoIter {
//         return (&self).into();
//     }
// }

// impl IntoIterator for &Rectangle {
//     type Item = (f64, f64);

//     type IntoIter = RectIter;

//     fn into_iter(self) -> Self::IntoIter {
//         return self.into();
//     }
// }

impl Points for Rectangle {
    fn points(&self) -> super::collisions::PointIter {
        return super::collisions::PointIter {
            points: vec![
                (self.x, self.y),
                (self.x + self.width, self.y),
                (self.x, self.y + self.height),
                (self.x + self.width, self.y + self.height),
            ],
            idx: 0,
        };
    }
}

impl Contains for Rectangle {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        return x >= self.x && x <= self.x + self.width &&
            y >= self.y && y <= self.y + self.height;
    }
}

impl FromStr for Rectangle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() != 4 {
            return Err(anyhow::anyhow!("Invalid number of parts"));
        }

        return Ok(Rectangle {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            width: parts[2].parse()?,
            height: parts[3].parse()?,
        });
    }
}