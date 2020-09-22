fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Vector {
    x: f64,
    y: f64,
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn solve(a: Vector, b: Vector) -> Vector {
    let m = (a.y - b.y) / (a.x - b.x);
    let h = a.y - m * a.x;
    Vector{x: m, y: h}
}

mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let a = Vector{x: 0.0, y: 0.0};
        let b = Vector{x: 1.0, y: 1.0};
        let ans = Vector{x: 1.0, y: 0.0};
        assert_eq!(solve(a, b), ans);
    }

    #[test]
    fn test_intersection() {
        let a = Vector{x: 1.0, y: 1.0};
        let b = Vector{x: -1.0, y: 1.0};
        let ans = Vector{x: 0.0, y: 1.0};
        assert_eq!(solve(a, b), ans);
    }
}
