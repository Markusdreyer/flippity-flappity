//Import everything from bracket lib
use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 20.0;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

//Everytning that needs to be preserved between ticks is in this state.
//The state is a snapshot of the current game world.
struct State {
    //Tracks all of the state for the player
    player: Player,
    //Tracks the time since last frame, used to control the game speed
    frame_time: f32,
    //Tracks the current obstacle
    obstacle: Obstacle,
    //Tracks the current mode
    mode: GameMode,
    //Tracks the player score
    score: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        //Generate a random number for the gap location
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            //Place of the gap
            gap_y: random.range(10, 40),
            //Size of the gap
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        //Calculate the obstacle in screen space
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        //Draw the top half of the obstacle
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        //Draw the bottom half of the obstacle
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;

        //Check if the player is in the same column as the obstacle
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;

        //If all three of these are true, the player is in the gap
        does_x_match && (player_above_gap || player_below_gap)
    }
}

//Constructor for the player struct
impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    //Details how the player is rendered
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        //Check for terminal velocity
        //Only apply velocity up to a certain point
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        //Apply velocity to position
        self.y += self.velocity as i32;
        //Keep track of character position (No visible effect)
        self.x += 1;

        //Make sure we don't clip through the floor
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        //Confusing that we are subtracting from velocity to go up
        //Zero is at the top of the screen
        self.velocity = -2.0;
    }
}

impl State {
    //State constructur which initializes a new state with GameMode::Menu as default
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::Menu,
            score: 0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to flippity flappity");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        ctx.print(0, 0, "Press SPACE to flap");
        ctx.print(0, 1, format!("Score: {}", self.score));

        //This means the frame has been shown long enough
        //And we start applying gravity and moving the player
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        //If the player hits space, flap
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        //Draw the updated position of the player
        self.player.render(ctx);

        //If the player hits the bottom of the screen, end the game
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }

        //Render the obstacle
        self.obstacle.render(ctx, self.player.x);

        //Increment score if the player has passed the obstacle
        //And spawn a new obstacle
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        //If the player hits the obstacle the floor, end the game
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(7, format!("You earned {} points", self.score));
        ctx.print_centered(9, "(P) Play Again");
        ctx.print_centered(10, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.mode = GameMode::Playing;
        self.score = 0;
    }
}

//GameState is a trait for the State struct
//Traits in Rust are similar to interfaces in other languages
//They define a set of methods that must be implemented
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        //Match statement works as traffic police for the game
        //It checks the current game mode and calls the appropriate function
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

//The main function is the entry point for all Rust programs
//The -> BError part means that the function returns a BError, allowing us to use the ? operator
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flippity Flappity")
        .build()?;
    main_loop(context, State::new())
}
