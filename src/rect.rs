use serde::Deserialize;

#[derive(Debug, Deserialize,Clone)]
pub struct Rect {
    pub x1 : i32,
    pub x2 : i32,
    pub y1 : i32,
    pub y2 : i32
}

impl Rect {
    pub fn new(x:i32, y: i32, w:i32, h:i32) -> Rect {
        Rect{x1:x, y1:y, x2:x+w, y2:y+h}
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other:&Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2)/2, (self.y1 + self.y2)/2)
    }

    pub fn is_west(&self, other:&Rect) -> bool {
        self.x1.max(self.x2) < other.x1.min(other.x2)
            && (between(self.y1,(other.y1,other.y2))
                || between(self.y2,(other.y1,other.y2)))
    }

    pub fn is_east(&self, other:&Rect) -> bool {
        self.x1.min(self.x2) > other.x1.max(other.x2)
            && (between(self.y1,(other.y1,other.y2))
                || between(self.y2,(other.y1,other.y2)))
    }

    pub fn is_lined_horizontal(&self, other:&Rect) -> bool {
        between(self.y1,(other.y1,other.y2))
                || between(self.y2,(other.y1,other.y2))
    }
}

pub fn between(n: i32, bound:(i32,i32)) -> bool{
    n>=bound.0 && n<=bound.1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_westeast() {
        let r1 = Rect{x1:15,x2:18,y1:20,y2:23};
        let r2 = Rect{x1:20,x2:30,y1:20,y2:30};
        assert!(r1.is_west(&r2));
        assert!(!r2.is_west(&r1));
        assert!(r2.is_east(&r1));
        assert!(!r1.is_east(&r2));

        assert!(r1.is_lined_horizontal(&r2));
        assert!(r2.is_lined_horizontal(&r1));
        
        let r3= Rect{x1:10,x2:13,y1:22,y2:26};
        assert!(r1.is_lined_horizontal(&r3));
        assert!(r3.is_lined_horizontal(&r1));
        
    }
}