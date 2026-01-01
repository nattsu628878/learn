use rand::Rng;
use rand::thread_rng;
use std::process::Command;
use core::time::Duration;

const CANVAS_WIDTH: usize = 120;
const CANVAS_HEIGHT: usize = 40;
const NRAINS: i32 = 80;
struct Rain {
    x: usize,
    y: usize,
    len: usize,
    amount_y: usize,
}

impl Rain {
    pub fn new() -> Self {
        Rain {
            x: 0,
            y: 0,
            len: 0,
            amount_y: 0,
        }
    }
}
struct World {
    canvas: Vec<char>,
    rains: Vec<Rain>,
}

impl World {
    pub fn new() -> Self {
        World {
            canvas: vec![],
            rains: vec![],
        }
    }

    pub fn init(&mut self) {
        for _ in 0..CANVAS_HEIGHT {
            for _ in 0..CANVAS_WIDTH {
                self.canvas.push(' ');
            }
        }

        let mut rng = thread_rng();

        for _ in 0..NRAINS {
            let mut rain = Rain::new();
            rain.x = rng.gen_range(0..CANVAS_WIDTH);
            rain.y = rng.gen_range(0..CANVAS_HEIGHT);
            rain.len = rng.gen_range(1..5);
            rain.amount_y = rng.gen_range(1..3);
            self.rains.push(rain);
        }
    }

    pub fn update(&mut self) {
        let mut rng = thread_rng();

        for i in 0..self.canvas.len() {
            self.canvas[i] = ' ';
        }

        for rain in self.rains.iter_mut() {
            rain.y += rain.amount_y;
            if rain.y >= CANVAS_HEIGHT {
                rain.x = rng.gen_range(0..CANVAS_WIDTH);
                rain.y = 0;
            }

            for i in 0..rain.len {
                let index = (rain.y + i) * CANVAS_WIDTH + rain.x;
                if index >= self.canvas.len() {
                    continue;
                }
                self.canvas[index] = '|';
            }
        }
    }

    pub fn draw(&mut self) {
        // let status = Command::new("cmd").args(&["/C","cls"]).status();
        let status = Command::new("clear").status();
        if let Err(err) = status {
            eprintln!("failed to clear: {}", err);
            std::process::exit(1);
        }

        for y in 0..CANVAS_HEIGHT {
            for x in 0..CANVAS_WIDTH {
                let i = y * CANVAS_WIDTH + x;
                let c = self.canvas[i];
                print!("{}", c);
            }
            println!("");
        }
    }

    pub fn run(&mut self) {
        self.init();

        loop {
            self.update();
            self.draw();
            std::thread::sleep(Duration::from_millis(33));
        }
    }
}

fn main() {
    let mut world = World::new();
    world.run();
}