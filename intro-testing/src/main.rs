#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new(x: i32, y: i32) -> Location {
        Location { x, y }
    }

    fn update_location(&mut self, step: char) {
        if step == '^' {
            self.y += 1;
        } else if step == 'v' {
            self.y -= 1;
        } else if step == '>' {
            self.x += 1;
        } else if step == '<' {
            self.x -= 1;
        }
    }
}

fn main(){}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_move() {
        let mut loc = Location::new(0, 0);
        loc.update_location('a');
        assert_eq!(loc, Location::new(0, 0));
    }

    #[test]
    fn test_update_location() {
        let mut loc = Location::new(0, 0);
        loc.update_location('^');
        assert_eq!(loc, Location::new(0, 1), "Failed to move up");
        loc.update_location('v');
        assert_eq!(loc, Location::new(0, 0), "Failed to move down");
        loc.update_location('>');
        assert_eq!(loc, Location::new(1, 0), "Failed to move right");
        loc.update_location('<');
        assert_eq!(loc, Location::new(0, 0), "Failed to move left");
    }
}
