extern crate sdl2;

pub mod my_event;

pub use self::my_event::MyEvent;

use self::sdl2::render;
use self::sdl2::video;
use self::sdl2::pixels;
use self::sdl2::rect;
use self::sdl2::event::Event;

// the width and height of the buffer (size = BUFFER_WIDTH * BUFFER_WIDTH)
const BUFFER_WIDTH: usize = 20;
// width in pixels in which one cell is rendered
const CELL_SIZE: usize = 20;

// colors that are drawn to the screen based on buffer values
static TRUE_COLOR: pixels::Color = pixels::Color::RGB(0, 0, 0);
static FALSE_COLOR: pixels::Color = pixels::Color::RGB(255, 255, 255);

pub struct MySdl<'a> {
    pub screen_buffer: [bool; BUFFER_WIDTH * BUFFER_WIDTH],
    renderer: render::Renderer<'a>,
    event_pump: sdl2::EventPump,
    pub timer: sdl2::TimerSubsystem,
}

impl<'a> MySdl<'a> {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video::WindowBuilder::new(&video_subsystem,
                                               "Output",
                                               (BUFFER_WIDTH * CELL_SIZE) as u32,
                                               (BUFFER_WIDTH * CELL_SIZE) as u32)
            .position_centered()
            .build()
            .unwrap();

        MySdl {
            screen_buffer: [false; 400],
            renderer: render::RendererBuilder::new(window).build().unwrap(),
            timer: sdl_context.timer().unwrap(),
            event_pump: sdl_context.event_pump().unwrap(),
        }
    }

    pub fn render(&mut self) {
        for iter_y in 0..BUFFER_WIDTH {
            for iter_x in 0..BUFFER_WIDTH {
                let cell_value = self.screen_buffer[iter_y * BUFFER_WIDTH + iter_x];
                let color = if cell_value {
                    TRUE_COLOR
                } else {
                    FALSE_COLOR
                };

                self.renderer.set_draw_color(color);
                self.renderer.fill_rect(rect::Rect::new((iter_x * CELL_SIZE) as i32,
                                                        (iter_y * CELL_SIZE) as i32,
                                                        CELL_SIZE as u32,
                                                        CELL_SIZE as u32));
            }
        }
        self.renderer.present();
    }

    pub fn next_event(&mut self) -> Option<MyEvent> {
        match self.event_pump.poll_event() {
            None => None,
            Some(event) => {
                Some(match event {
                    Event::Quit { .. } => MyEvent::Quit,
                    _ => MyEvent::DoNothingTest,
                })
            }
        }
    }
}