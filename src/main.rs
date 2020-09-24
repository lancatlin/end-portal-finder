use std::io::{self, Write};

fn main() {
    println!("Welcome to End Portal Finder.");
    println!("Please input the coordinates.");
    println!("syntax: a.x a.y b.x b.y");
    println!("Press Ctrl+C to exit.");
    let mut vectors: Vec<Vector> = Vec::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let line = read();
        if let Some(v) = parse(line) {
            let eq = solve_line(v);
            println!("y = {:.1}x + {:.1}", eq.x, eq.y);
            vectors.push(eq);
        } else {
            continue;
        }
        if vectors.len() > 1 {
            let len = vectors.len();
            let point = solve_intersection(&vectors[len - 1], &vectors[len - 2]);
            println!("The intersection is at ({:.1}, {:.1}).", point.x, point.y);
        }
    }
}

fn read() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn parse(line: String) -> Option<(Vector, Vector)> {
    let mut nums: Vec<f64> = Vec::new();
    for word in line.trim().split(" ") {
        match word.parse() {
            Ok(n) => nums.push(n),
            Err(_) => (),
        }
    }
    if nums.len() != 4 {
        println!("Please input exactly 4 number");
        return None;
    }
    Some((
        Vector {
            x: nums[0],
            y: nums[1],
        },
        Vector {
            x: nums[2],
            y: nums[3],
        },
    ))
}

fn solve_line(vectors: (Vector, Vector)) -> Vector {
    let (a, b) = vectors;
    let m = (a.y - b.y) / (a.x - b.x);
    let h = a.y - m * a.x;
    Vector { x: m, y: h }
}

fn solve_intersection(a: &Vector, b: &Vector) -> Vector {
    let m = -(a.y - b.y) / (a.x - b.x);
    let h = a.y + m * a.x;
    Vector { x: m, y: h }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_line() {
        let a = Vector { x: 0.0, y: 0.0 };
        let b = Vector { x: 1.0, y: 1.0 };
        let ans = Vector { x: 1.0, y: 0.0 };
        assert_eq!(solve_line((a, b)), ans);
    }

    #[test]
    fn test_solve_intersection() {
        let a = Vector { x: 1.0, y: 1.0 };
        let b = Vector { x: -1.0, y: -1.0 };
        let ans = Vector { x: -1.0, y: 0.0 };
        assert_eq!(solve_intersection(&a, &b), ans);
    }

    #[test]
    fn test_parse() {
        let t = vec![
            (
                "20 8 2 3\n",
                Some((Vector { x: 20.0, y: 8.0 }, Vector { x: 2.0, y: 3.0 })),
            ),
            (
                "  1 2 3 4  \n",
                Some((Vector { x: 1.0, y: 2.0 }, Vector { x: 3.0, y: 4.0 })),
            ),
            (
                "1  2  3  4  \n",
                Some((Vector { x: 1.0, y: 2.0 }, Vector { x: 3.0, y: 4.0 })),
            ),
            ("1 2 3 4 5 \n", None),
        ];
        for (q, ans) in t {
            assert_eq!(parse(q.to_owned()), ans);
        }
    }
}
