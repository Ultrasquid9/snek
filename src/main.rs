use macroquad::prelude::*;

/// Stores a snek
struct Snek {
    body: Vec<Vec2>,
    dir: SnekDir,
    score: usize
}

struct YastyFruit {
    pos: Vec2,
    texture: Texture2D,
    params: DrawTextureParams
}

impl Snek {
    /// Gets a new position for the snek
    fn get_new_pos(&self, dir: &SnekDir) -> Vec2 {
        let mut new_pos = Vec2 {
            x: self.body[0].x.clone(),
            y: self.body[0].y.clone()
        };

        match dir {
            SnekDir::Up => new_pos.y -= 2.0,
            SnekDir::Down => new_pos.y += 2.0,
            SnekDir::Left => new_pos.x -= 2.0,
            SnekDir::Right => new_pos.x += 2.0,
            SnekDir::None => ()
        }

        return new_pos;
    }

    /// Draws the snek
    fn draw_snek(&self) {
        for i in &self.body {
            draw_circle(i.x, i.y, 14.0, ORANGE);
        }
    }

    /// Increases the size of the snek by 10
    fn expand_snek(&mut self) {
        for _i in 1..16 {
            self.body.push(Vec2 {
                x: self.body[self.body.len() - 1].x.clone(),
                y: self.body[self.body.len() - 1].y.clone(),
            })
        }
    }

    /// Creates a new snek
    fn new_snek() -> Snek {
        let mut snek = Snek {
            body: vec![Vec2 {
                x: screen_width() / 2.0, 
                y: screen_height() / 2.0
            }],
            dir: SnekDir::None,
            score: 0
        };
        snek.expand_snek();
        return snek;
    }
}

impl YastyFruit {
    /// Creates a new Vec2 with a random position
    fn random_pos() -> Vec2 {
        Vec2::new(
            rand::gen_range(16.0, screen_width() - 16.0), 
            rand::gen_range(16.0, screen_height() - 16.0)
        )
    }

    fn clone(&self) -> YastyFruit {
        YastyFruit {
            pos: self.pos,
            texture: self.texture.clone(),
            params: self.params.clone()
        }
    }

    // Creates a new YastyFruit
    fn new_yasty_fruit(texture: Texture2D) -> YastyFruit {
        let mut yasty_fruit = YastyFruit {
            pos: YastyFruit::random_pos(),
            texture: texture,
            params: DrawTextureParams::default()
        };
        yasty_fruit.params.dest_size = Some(Vec2 {
            x: screen_height() / 16.0,
            y: screen_height() / 16.0
        });
        return yasty_fruit;
    }
}

/// Directions the snek could be travelling
#[derive(PartialEq)]
enum SnekDir {
    Up,
    Down,
    Left,
    Right,
    None
}

#[macroquad::main("Snek")]
async fn main() {
    let fruit_textures = [
        Texture2D::from_file_with_format(include_bytes!("../assets/textures/apple.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/textures/orange.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/textures/strawberry.png"), None)
    ];
    let mut snek = Snek::new_snek();
    let mut yasty_fruit: Vec<YastyFruit> = vec![YastyFruit::new_yasty_fruit(get_random_texture(&fruit_textures))];
    let mut has_died = false;

    loop {
        // Handles input
        if is_key_down(KeyCode::Right) && snek.dir != SnekDir::Left {
            snek.dir = SnekDir::Right;
        }
        if is_key_down(KeyCode::Left) && snek.dir != SnekDir::Right {
            snek.dir = SnekDir::Left;
        }
        if is_key_down(KeyCode::Down) && snek.dir != SnekDir::Up {
            snek.dir = SnekDir::Down;
        }
        if is_key_down(KeyCode::Up) && snek.dir != SnekDir::Down {
            snek.dir = SnekDir::Up;
        }

        // Checking if the player is dead, and drawing to the screen accordingly
        if has_died && snek.dir == SnekDir::None {
            clear_background(LIGHTGRAY);

            draw_text("you died lmao", (screen_width() / 2.0) - 300.0, screen_height() / 2.0, 100.0, RED);
            draw_text("press the arrow keys to restart", (screen_width() / 2.0) - 380.0, (screen_height() / 2.0) + 50.0, 60.0, RED);
        } else {
            draw(&yasty_fruit, &snek);
        }

        // Moves the snek based on the current direction it is travelling 
        snek.body.insert(0, snek.get_new_pos(&snek.dir));

        // Checks if the snek should eat a fruit
        if try_eat_fruit(&snek, &mut yasty_fruit, &fruit_textures) {
            snek.expand_snek(); // Expands the snek 
            snek.score += 1; // Expands the score

            // Adds another fruit
            if yasty_fruit.len() <= 8 && snek.score % 5 == 0 {
                let mut new_fruit = yasty_fruit[0].clone();
                new_fruit.pos = YastyFruit::random_pos();
                yasty_fruit.push(YastyFruit {
                    pos: YastyFruit::random_pos(),
                    texture: get_random_texture(&fruit_textures),
                    params: yasty_fruit[0].params.clone()
                });
            }
        } else {
            snek.body.remove(snek.body.len() - 1); // Removes the end of the snek
        }

        // Checks to see if the snek should be killed
        if try_kill_snek(&snek) {
            kill_snek(&mut snek, &mut yasty_fruit);
            has_died = true;
        }

        next_frame().await
    }
}

/// Draws the game elements
fn draw(yasty_fruit: &Vec<YastyFruit>, snek: &Snek) {
    clear_background(Color::new(0.35, 0.7, 0.1, 1.0));

    for i in yasty_fruit {
        draw_texture_ex(&i.texture, i.pos.x - 20.0, i.pos.y - 20.0, WHITE, i.params.clone());
    }
    snek.draw_snek();

    draw_text("move the snek with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
    draw_text(&format!("score: {}", snek.score), 20.0, 40.0, 20.0, DARKGRAY);
}

fn get_random_texture(textures: &[Texture2D]) -> Texture2D {
    textures[rand::gen_range(0, textures.len())].clone()
}

/// Checks to see if the snek should eat a fruit
fn try_eat_fruit(snek: &Snek, yasty_fruit: &mut Vec<YastyFruit>, textures: &[Texture2D]) -> bool {
    for i in yasty_fruit {
        if snek.body[0].distance(i.pos) < 20.0 {
            i.pos = YastyFruit::random_pos();
            i.texture = get_random_texture(textures);
            return true;
        }
    }
    return false;
}

/// Checks to see if the snek should be killed
fn try_kill_snek(snek: &Snek) -> bool {
    if snek.body[0].x > screen_width() - 2.0
    || snek.body[0].y > screen_height() - 2.0
    || snek.body[0].x < 2.0 
    || snek.body[0].y < 2.0 
        {
            return true;
        }

    for i in snek.body.iter().enumerate() {
        if i.0 < 18 {
            continue;
        }

        if i.1.distance(snek.body[0]) < 18.0 {
            return true;
        }
    }

    return false;
}

/// Resets the provided values
fn kill_snek(snek: &mut Snek, yasty_fruit: &mut Vec<YastyFruit>) {
    *snek = Snek::new_snek();
    yasty_fruit.drain(1..yasty_fruit.len());
}
