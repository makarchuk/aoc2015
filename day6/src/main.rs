use std::io::{self, Read};

struct Lights {
    lights: [[bool; 1000]; 1000],
}

struct Corner {
    x: usize,
    y: usize,
}

impl Corner {
    fn parse(s: &str) -> Corner {
        let bounds: Vec<usize> = s.split(",").map(|x| x.parse().unwrap()).collect();
        Corner {
            x: bounds[0],
            y: bounds[1],
        }
    }
}

impl Lights {
    fn new() -> Lights {
        Lights {
            lights: [[false; 1000]; 1000],
        }
    }

    fn turn_on(_b: bool) -> bool {
        return true;
    }

    fn turn_off(_b: bool) -> bool {
        return false;
    }

    fn toggle(b: bool) -> bool {
        return !b;
    }

    fn instruction(&mut self, op: &str, from: &Corner, to: &Corner) {
        let f = match op {
            "turn on" => Lights::turn_on,
            "turn off" => Lights::turn_off,
            "toggle" => Lights::toggle,
            _ => panic!("Unknown command"),
        };
        for x in from.x..=to.x {
            for y in from.y..=to.y {
                self.lights[x][y] = f(self.lights[x][y])
            }
        }
    }

    fn count(&self) -> usize {
        self.lights
            .iter()
            .map(|row| row.iter().filter(|x| **x).count())
            .sum()
    }
}

struct AncientNordicElvishLights {
    lights: Vec<Vec<i32>>,
}

impl AncientNordicElvishLights {
    fn new() -> AncientNordicElvishLights {
        AncientNordicElvishLights {
            lights: vec![vec![0; 1000]; 1000],
        }
    }

    fn turn_on(v: i32) -> i32 {
        return v + 1;
    }

    fn turn_off(v: i32) -> i32 {
        if v > 1 {
            return v - 1;
        } else {
            return 0;
        }
    }

    fn toggle(v: i32) -> i32 {
        return v + 2;
    }

    fn instruction(&mut self, op: &str, from: &Corner, to: &Corner) {
        let f = match op {
            "turn on" => AncientNordicElvishLights::turn_on,
            "turn off" => AncientNordicElvishLights::turn_off,
            "toggle" => AncientNordicElvishLights::toggle,
            _ => panic!("Unknown command"),
        };
        for x in from.x..=to.x {
            for y in from.y..=to.y {
                self.lights[x][y] = f(self.lights[x][y])
            }
        }
    }

    fn total(&self) -> i32 {
        self.lights.iter().fold(0, |sum: i32, row| {
            sum + row.iter().fold(0, |sum, x| sum + x)
        })
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut l = Lights::new();
    let mut anel = AncientNordicElvishLights::new();
    for line in input.split("\n") {
        let partialy_splited: Vec<_> = line.split(" through ").collect();
        let left_part: Vec<_> = partialy_splited[0].split(" ").collect();
        let op = match left_part[left_part.len() - 2] {
            "on" => "turn on",
            "off" => "turn off",
            st => st,
        };
        let from = Corner::parse(left_part[left_part.len() - 1]);
        let to = Corner::parse(partialy_splited[1]);
        l.instruction(op, &from, &to);
        anel.instruction(op, &from, &to);
    }
    println!("Lights are shining: {}", l.count());
    println!("Total brightnes is : {}", anel.total());
}
