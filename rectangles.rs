#[derive(Copy, Clone)]
struct Point {
	x: i32,
	y: i32
}

#[derive(Copy, Clone)]
struct Rectangle {
	x: i32,
	y: i32,
	x2: i32,
	y2: i32
}

impl PartialEq for Point {
  fn eq(&self, rhs: &Self) -> bool {
      self.x == rhs.x && self.y == rhs.y
  }
}

impl PartialEq for Rectangle {
  fn eq(&self, rhs: &Self) -> bool {
      self.x == rhs.x && self.y == rhs.y && self.x2 == rhs.x2 && self.y2 == rhs.y2
  }
}

pub fn count(lines: &[&str]) -> u32 {
	let mut y = 0;
	let mut result = 0;

  for line in lines {
  	for (i, c) in line.chars().enumerate() {
  		let int_i = i as i32;
  		let mut rectangles = vec![];

  		if c == '+' {
				let dots_to_right = all_right(lines, &int_i, &y);
				let dots_to_bottom = all_bottom(lines, &int_i, &y);

				for rd in &dots_to_right {
					for bd in &dots_to_bottom {
						let rr = all_right(lines, &bd.x, &bd.y);
						let bb = all_bottom(lines, &rd.x, &rd.y);

						for r in &rr {
							for b in &bb {
								if &r == &b {
			  					let r = Rectangle{x: int_i, y: y, x2: r.x, y2: b.y};
			  					if !rectangles.contains(&r) {
				  					rectangles.push(r);
					  				result += 1;
					  			}
								}
			  			}
						}
					}
				}
  		}
		}
		y += 1;
  }
  result
}

fn all_right(lines: &[&str], x: &i32, y: &i32) -> Vec<Point> {
	let line = lines[*y as usize];
	let mut all_dots = vec![];

	for (i, c) in line.chars().enumerate() {
		let int_i = i as i32;
		if int_i <= *x {
			continue;
		}

		if c != '-' && c != '+' {
			break;
		}

		if c == '+' {
			all_dots.push(Point{x: int_i, y: *y});
		}
	}

	all_dots
}

fn all_bottom(lines: &[&str], x: &i32, y: &i32) -> Vec<Point> {
	let mut all_dots = vec![];
	let e = lines.len() - 1;
	let e_int = e as i32;

	for i in (*y + 1)..=e_int {
		let line = lines[(i) as usize];
		let ch = line.chars().nth(*x as usize).unwrap();

		if ch != '|' && ch != '+' {
			break;
		}

		if ch == '+' {
			all_dots.push(Point{x: *x, y: i});
		}
	}

	all_dots
}

fn main() {
    let lines = &[
        "+---+--+----+",
        "|   +--+----+",
        "+---+--+    |",
        "|   +--+----+",
        "+---+--+--+-+",
        "+---+--+--+-+",
        "+------+  | |",
        "          +-+"
        ];


 	println!("result: {:?}", count(lines)); 
}