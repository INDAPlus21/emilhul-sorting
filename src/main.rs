use ggez::{event, conf, timer, Context, ContextBuilder, GameResult, GameError, graphics::{self, Color as EzColor}};
use colorgrad::*;

use rand::{thread_rng, seq::SliceRandom};


/// Size of grid in cells. (X, Y)
const GRID_SIZE: (f32, f32) = (120.0, 80.0);

/// Defines the lenght of a side for the square grid cell in pixels.
const GRID_CELL_SIZE: f32 = 10.0;

/// Size of the application screen
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 * GRID_CELL_SIZE,
    GRID_SIZE.1 * GRID_CELL_SIZE
);

pub fn get_color(value: usize, gradient: &mut Gradient) -> EzColor {
    let position: f64 = value as f64 / 100.0;
    let color: Color = gradient.at(position);
    let rgba: (f64, f64, f64, f64) = color.rgba();
    EzColor::new(rgba.0 as f32, rgba.1 as f32, rgba.2 as f32, rgba.3 as f32)
}

enum Algorithm {
    InsertionSort,
    PancakeSort
}

struct AppState {
    gradient: Gradient,
    target_fps: u32,
    array: Vec<usize>,

    
    algorithm: Algorithm,
    sorting: bool,
    setup: bool,
    outer_step: usize,
    inner_step: usize,
}

impl AppState {
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        Ok(AppState {
            gradient: {
                CustomGradient::new()
                .colors(&[
                    Color::from_rgba_u8(252, 70, 107, 1*255),
                    Color::from_rgba_u8(63, 94, 251, 1*255)
                ])
                .build()
                .expect("Failed to build gradient")
            },
            target_fps: 60,
            array: (1..101).collect(),
            algorithm: Algorithm::PancakeSort,
            sorting: false,
            setup: false,
            outer_step: 0,
            inner_step: 0,
        })
    }

    fn shuffle(&mut self) {
        self.array.shuffle(&mut thread_rng());
    }

    fn pancake_sort_setup(&mut self) {
        self.outer_step = self.array.len();
        self.sorting = true;
        self.setup = false;
    }

    fn pancake_sort(&mut self) {
        if self.outer_step > 1 {
            let mut max_index = 0;
            for i in 0..self.outer_step {
                if self.array[i] > self.array[max_index] {
                    max_index = i;
                }
            }
            if max_index != self.outer_step - 1 {
                self.pancake_flipper(max_index);

                self.pancake_flipper(self.outer_step-1);
            }
            self.outer_step -= 1;
        } else {
            self.sorting = false;
        }
    }

    fn pancake_flipper(&mut self, mut upper_index: usize) {
        let mut lower_index = 0;
        while lower_index < upper_index {
            self.array.swap(lower_index, upper_index);
            lower_index += 1;
            upper_index -= 1;
        }
    }

    fn insertion_sort_setup(&mut self) {
        self.outer_step = 1;
        self.sorting = true;
        self.setup = false;
    }

    fn insertion_sort(&mut self) {
        if self.outer_step < self.array.len() {
            let value = self.array[self.outer_step];
        
            let mut j = self.outer_step;
            while j >= 1 && value < self.array[j-1] {
                self.array[j] = self.array[j-1];
                j -= 1;
            }
            self.array[j] = value;

            self.outer_step += 1;
        } else {
            self.sorting = false;
        }
    }
}

impl event::EventHandler<GameError> for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, self.target_fps) {
            if self.setup {
                match self.algorithm {
                    Algorithm::InsertionSort => self.insertion_sort_setup(),
                    Algorithm::PancakeSort => self.pancake_sort_setup(),
                }
            } else if self.sorting {
                match self.algorithm {
                    Algorithm::InsertionSort => self.insertion_sort(),
                    Algorithm::PancakeSort => self.pancake_sort(),
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, EzColor::BLACK);

        for i in 0..self.array.len() {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx, 
                graphics::DrawMode::fill(), 
                graphics::Rect::new(
                    10.0 * GRID_CELL_SIZE + i as f32 * GRID_CELL_SIZE,
                    40.0 * GRID_CELL_SIZE - self.array[i] as f32 * (30.0 * GRID_CELL_SIZE)/100.0,
                    GRID_CELL_SIZE-5.0,
                    self.array[i] as f32 * (30.0 * GRID_CELL_SIZE)/100.0),
                get_color(self.array[i], &mut self.gradient)
            )
            .expect("Failed to create bar");
            graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                .expect("Failed to draw bar");
        } 

        graphics::present(ctx).expect("Failed to update graphics");

        Ok(())
    }

    /// Update game on mouse click
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == event::MouseButton::Left {
            if !self.sorting {
                if x <= SCREEN_SIZE.0/3.0 {
                    println!{"Shuffle!"};
                    self.shuffle();
                } else if x > SCREEN_SIZE.0/3.0 && x < 2.0*SCREEN_SIZE.0/3.0 {
                    println!{"Insertion Sort!"};
                    self.algorithm = Algorithm::InsertionSort;
                    self.setup = true;
                } else if x >= 2.0*SCREEN_SIZE.0/3.0 {
                    println!{"Pancake Sort!"}
                    self.algorithm = Algorithm::PancakeSort;
                    self.setup = true;
                }
            }
        } 
    }
}

pub fn main() {
    let context_builder = ContextBuilder::new("sorting_visualizer", "Emil Hultcrantz")
        .window_setup(
            conf::WindowSetup::default()
                .title("Sorting Visualizer")
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1)
                .resizable(false)
        );
    let (mut context, event_loop) = context_builder.build().expect("Failed to build context.");
    let state = AppState::new(&mut context).expect("Failed to create state");
    event::run(context, event_loop, state)
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert_eq!(2+2,4);
    }
}