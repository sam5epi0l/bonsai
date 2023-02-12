use std::thread;
use ::std::time::Duration;
use rand::random;

const ROWS: i32 = 35;
const COLS: i32 = 40;

struct Grid {
  grid: Vec<Vec<String>>,
}

impl Grid {
  fn new() -> Self {
    let grid =
      (0..ROWS)
        .into_iter()
              .map(|_|
                     vec![" ".to_owned(); COLS as usize]
                    )
                .collect::<Vec<Vec<String>>>();
        Self { grid }
    }

    fn show(&self) {
        let str = self.get_string();
        println!("{}", str);
    }

    fn get_string(&self) -> String {
        let mut str = String::new();
        for row in &self.grid {
            let row_str = row.join("");
            str.push_str(&row_str);
            str.push_str("\n");
        }
        str
    }
}
  //max_branches: i32,

fn grow(grid: &mut Grid, _max_branches: i32, t: i32) {
    let start = (COLS / 2) as usize;
    let x = start;
    let y = ROWS - 3;
    let life = 1;
    step(grid, x, y, life, t);
}

fn next_time_t(t: i32) -> i32 {
    t + 2
}

fn next_time(t: i32) -> i32 {
    (1 + (next_time_t(t) / 5)) as i32
}

fn step(grid: &mut Grid, x: usize, y: i32, life: i32, t: i32) {
    if life < 1 {
        println!("early deaths");
        return;
    }

    println!("Life is {}", life);
    let dy = if rand(1, 2) == 1 { -1 } else { 1 };
    let dx = rand(-2, 2);
    let mut char = if dx > 0 {
        "/"
    } else if dx == 0 {
        "|"
    } else {
        "\\"
    };
    if dy == 0 {
        //let str = "|".to_string();  
        //let vertical = "/".to_string();  
        //(*char) = (|| &str)();
        char = "|"
    }
    if life == 1 {
        char = "&"
    }

    if x < 0 || x >= ROWS as usize || y < 0 {
        return;
    }
    thread::sleep(Duration::from_millis(t as u64));
    if life > 0 {
        grid.grid[y as usize][x] = char.to_owned();
        grid.show();
        step(grid, x + dx as usize, y + dy, life - 1, t);
    }
}

fn rand(min: i32, max: i32) -> i32 {
    min + (random::<f64>() * (min.abs() + max) as f64) as i32
}

fn main() {
    let mut grid = Grid::new();
    let t = 1;
    grow(&mut grid, 32, t);
}
