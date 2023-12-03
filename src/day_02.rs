pub struct Game {
    pub id: i32,
    red: i32,
    green: i32,
    blue: i32
}

pub struct CubeCount {
    red: i32,
    green: i32,
    blue: i32
}

fn parse_count_cube(text: &String) -> CubeCount {
    let rgbs = text.split(",");
    let mut cube = CubeCount {
        red: 0,
        green: 0,
        blue: 0,
    };
    for rgb in rgbs {
        let split: Vec<&str> = rgb.trim().split(" ").collect();
        let num = i32::from_str_radix(&split[0], 10).expect("Num should be first part of array");
        match split[1] {
            "red" => {
                cube.red = num;
            },
            "green" => {
                cube.green = num;
            },
            _ => {
                cube.blue = num;
            }
        }
    }
    cube
}

impl CubeCount {
    pub fn from(red: i32, green: i32, blue: i32) -> Self {
        CubeCount { red, green, blue }
    }
}

impl Game {
    pub fn less_than_eq_cube_count(&self, cube_count: &CubeCount) -> bool {
        self.red <= cube_count.red && self.green <= cube_count.green && self.blue <= cube_count.blue
    }

    pub fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }

    // Expect Game 1: to be gone
    pub fn from(id: i32, line: &str) -> Self {
        let mut game = Game {
            id,
            red: 0,
            green: 0,
            blue: 0
        };
        let pulls = line.split(";");
        pulls.for_each(|p| {
            let s = String::from(p);
            let cube = parse_count_cube(&s);
            if cube.red > game.red {
                game.red = cube.red;
            }
            if cube.green > game.green {
                game.green = cube.green;
            }
            if cube.blue > game.blue {
                game.blue = cube.blue;
            }
        });
        game
    }
}

#[test]
fn parse_count_cube_should_create_cube() {
    let line = String::from(" 7 blue, 4 red, 11 green");
    let cube = parse_count_cube(&line);
    let want = CubeCount {
        red: 4,
        green: 11,
        blue: 7
    };
    assert_eq!(cube.red, want.red);
    assert_eq!(cube.green, want.green);
    assert_eq!(cube.blue, want.blue);
}

#[test]
fn game_from_creates_game_from_text() {
    let line = String::from(" 7 blue, 4 red, 11 green; 2 red, 2 blue, 7 green; 2 red, 13 blue, 8 green; 18 blue, 7 green, 5 red");
    let want = Game {
        id: 1,
        red: 5,
        green: 11,
        blue: 18,
    };
    let game = Game::from(1, &line);
    assert_eq!(game.red, want.red);
    assert_eq!(game.green, want.green);
    assert_eq!(game.blue, want.blue);
}

#[test]
fn game_less_than_eq_cube_count() {
    let game = Game {
        id: 1,
        red: 5,
        green: 11,
        blue: 18,
    };
    let cube = CubeCount {
        red: 12,
        green: 13,
        blue: 14
    };
    let got = game.less_than_eq_cube_count(&cube);
    assert_eq!(got, false);
}