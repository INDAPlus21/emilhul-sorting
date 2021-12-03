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
}

impl AppState {
    fn new(_ctx: &mut Context) -> GameResult<AppState> {
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
        })
    }

    /// Shuffles self.array
    fn shuffle(&mut self) {
        self.array.shuffle(&mut thread_rng());
    }

    /// Gets color for a value 0 <= x <= 100 from self.grardient
    fn get_color(&mut self, value: usize) -> EzColor {
        if value <= 100 {
            let position: f64 = value as f64 / 100.0;
            let rgba: (f64, f64, f64, f64) = self.gradient.at(position).rgba();
            EzColor::new(rgba.0 as f32, rgba.1 as f32, rgba.2 as f32, rgba.3 as f32)
        } else {
            EzColor::WHITE
        }
    }

    // #region Pancake sort
    /// Set-up for self.pancake_sort()
    fn pancake_sort_setup(&mut self) {
        self.outer_step = self.array.len();
        self.sorting = true;
        self.setup = false;
    }

    /// One step of pancake_sort
    /// 
    /// While sorting runs once every frame
    /// 
    /// See https://en.wikipedia.org/wiki/Pancake_sorting
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

    /// Reverses self.array from index 0 to upper_index
    /// 
    /// Helper function for self.pancake_sort()
    fn pancake_flipper(&mut self, mut upper_index: usize) {
        let mut lower_index = 0;
        while lower_index < upper_index {
            self.array.swap(lower_index, upper_index);
            lower_index += 1;
            upper_index -= 1;
        }
    }
    // #endregion

    // #region Insertion sort
    /// Set-up for self.insertion_sort()
    fn insertion_sort_setup(&mut self) {
        self.outer_step = 1;
        self.sorting = true;
        self.setup = false;
    }

    /// One step of insertion_sort
    ///  
    /// While sorting runs once every frame
    /// 
    /// To avoid subtraction overflow:
    /// * j index is equal to outer_step (i), instead of i - 1 
    /// * stops at j = 1, instead of 0
    /// * indexes array with j-1, instead of j ( ( j-1 ) + 1 = j )
    /// 
    /// See https://en.wikipedia.org/wiki/Insertion_sort
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
    // #endregion
}

impl event::EventHandler<GameError> for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, self.target_fps) { // Updates at most target_fps frames per second. Can be used to change speed of algorithm
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
                self.get_color(self.array[i])
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
        _y: f32,
    ) {
        if button == event::MouseButton::Left {
            if !self.sorting { // Lock input if sorting
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

/// Sets-up and launches application
pub fn main() -> GameResult {
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
    use crate::AppState;
    use crate::Algorithm;
    use colorgrad::*;

    #[test]
    fn it_works() {
        assert_eq!(2+2,4)
    }
    
    #[test]
    fn pancake_sort_test() {
        print!("Pancake sort: Empty array ... ");
        let mut test_state = AppState {
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
            array: Vec::new(),
            algorithm: Algorithm::PancakeSort,
            sorting: false,
            setup: false,
            outer_step: 0,
        };

        test_state.shuffle();
        test_state.pancake_sort_setup();
        while test_state.sorting {
            test_state.pancake_sort();
        }
        assert_eq!(test_state.array, []);
        println!("ok");

        print!("Pancake sort: Repetion of elements ... ");
        test_state.array = Vec::from([4,2,1,3,5,2,6]);
        test_state.shuffle();
        test_state.pancake_sort_setup();
        while test_state.sorting {
            test_state.pancake_sort();
        }
        assert_eq!(test_state.array, [1,2,2,3,4,5,6]);
        println!("ok");

        print!("Pancake sort: Reverse input ... ");
        test_state.array = (1..101).rev().collect();
        test_state.pancake_sort_setup();
        while test_state.sorting {
            test_state.pancake_sort();
        }
        let eq: Vec<usize> = (1..101).collect();
        assert_eq!(test_state.array, eq);
        println!(" ok");
    }

    #[test]
    fn insertion_sort_test() {
        print!("Insertion sort: Empty array ... ");
        let mut test_state = AppState {
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
            array: Vec::new(),
            algorithm: Algorithm::InsertionSort,
            sorting: false,
            setup: false,
            outer_step: 0,
        };

        test_state.shuffle();
        test_state.pancake_sort_setup();
        while test_state.sorting {
            test_state.pancake_sort();
        }
        assert_eq!(test_state.array, []);
        println!("ok");

        print!("Insertion sort: Repetion of elements ... ");
        test_state.array = Vec::from([4,2,1,3,5,2,6]);
        test_state.shuffle();
        test_state.pancake_sort_setup();
        while test_state.sorting {
            test_state.pancake_sort();
        }
        assert_eq!(test_state.array, [1,2,2,3,4,5,6]);
        println!("ok");

        print!("Insertion sort: Reverse input ... ");
        test_state.array = (1..101).rev().collect();
        test_state.pancake_sort_setup();
        while test_state.sorting {
            test_state.pancake_sort();
        }
        let eq: Vec<usize> = (1..101).collect();
        assert_eq!(test_state.array, eq);
        println!(" ok");
    }
}