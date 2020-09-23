use std::io::{self, Write};

fn main() {
    println!("Please input the coordinate!");
	println!("syntax: a.x a.y b.x b.y");
	println!("Press Ctrl+C to exit.");
    let mut vectors: Vec<(Vector, Vector)> = Vec::new();
	loop {
		print!("> ");
		io::stdout().flush().unwrap();
		let line = read();
		vectors.push(parse(line));
		if vectors.len() > 1 {
			let point = intersection(&vectors);
			println!("The intersection is at ({:.1}, {:.1}).", point.x, point.y);
		}
	}
}

fn intersection(vectors: &Vec<(Vector, Vector)>) -> Vector {
	let len = vectors.len();
	let l1 = solve_line(&vectors[len-1]);
	let l2 = solve_line(&vectors[len-2]);
	solve_intersection(&(l1, l2))
}

fn read() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn parse(line: String) -> (Vector, Vector) {
    let mut nums: Vec<f64> = Vec::new();
    for word in line.trim().split(" ") {
        nums.push(word.parse().unwrap());
    }
    (Vector{x: nums[0], y: nums[1]}, Vector{x: nums[2], y: nums[3]})
}

#[derive(Debug)] struct Vector {
    x: f64,
    y: f64,
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn solve_line(vectors: &(Vector, Vector)) -> Vector {
	let (a, b) = vectors;
    let m = (a.y - b.y) / (a.x - b.x);
    let h = a.y - m * a.x;
    Vector{x: m, y: h}
}

fn solve_intersection(vectors: &(Vector, Vector)) -> Vector {
	let (a, b) = vectors;
    let m = - (a.y - b.y) / (a.x - b.x);
    let h = a.y + m * a.x;
    Vector{x: m, y: h}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_line() {
        let a = Vector{x: 0.0, y: 0.0};
        let b = Vector{x: 1.0, y: 1.0};
        let ans = Vector{x: 1.0, y: 0.0};
        assert_eq!(solve_line(&(a, b)), ans);
    }

    #[test]
    fn test_solve_intersection() {
        let a = Vector{x: 1.0, y: 1.0};
        let b = Vector{x: -1.0, y: -1.0};
        let ans = Vector{x: -1.0, y: 0.0};
        assert_eq!(solve_intersection(&(a, b)), ans);
    }

    #[test]
    fn test_parse() {
        let s = "20 8 2 3\n".to_owned();
        assert_eq!(parse(s), (Vector{x: 20.0, y: 8.0}, Vector{x: 2.0, y: 3.0}));
    }

	#[test]
	fn test_intersection() {
		let vectors = vec![
			(Vector{x: 1.0, y: 1.0}, Vector{x: 3.0, y: 1.0}), // y = 1
			(Vector{x: 2.0, y: 2.0}, Vector{x: 4.0, y: 0.0}), // y = -x + 4
		];
		assert_eq!(intersection(&vectors), Vector{x: 3.0, y: 1.0});
	}
}
