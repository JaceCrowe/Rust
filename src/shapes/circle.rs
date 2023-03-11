use super::area::Area;
use std::f64::consts::PI;


pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Circle {
    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;

        return dx * dx + dy * dy <= self.radius * self.radius;
    }
}

// impl Collidable<Rect> for Circle {
//     fn collide(&self, other: &Rect) -> bool {
//         for point in other {
//             if self.contains_point(point) {
//                 return true;
//             }
//         }
//         return false;
//     }
// }

// impl Collidable<Circle> for Circle {
//     fn collide(&self, other: &Circle) -> bool {
//         return self.contains_point((other.x, other.y)) ||
//             other.contains_point((self.x, self.y));
//     }
// }

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI
    }
}

impl Contains for Circle {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;

        return dx * dx + dy * dy <= self.radius * self.radius;
    }
}

impl Points for Circle {
    fn points(&self) -> super::collisions::PointIter {
        return vec![
            (self.x, self.y),
        ].into();
    }
}

impl FromStr for Circle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("Invalid number of parts"));
        }

        return Ok(Circle {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            radius: parts[2].parse()?,
        });
    }
}