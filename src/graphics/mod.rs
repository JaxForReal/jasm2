extern crate sdl2;

pub mod my_event;

pub use self::my_event::MyEvent;

use self::sdl2::render;
use self::sdl2::video;
use self::sdl2::pixels;
use self::sdl2::rect;
use self::sdl2::event::Event;

pub struct MySdl<'a> {
    pub screen_buffer: [bool; 400],
    renderer: render::Renderer<'a>,
    event_pump: sdl2::EventPump,
    pub timer: sdl2::TimerSubsystem,
}

impl<'a> MySdl<'a> {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video::WindowBuilder::new(&video_subsystem, "Output", 400, 400)
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
        self.renderer.set_draw_color(pixels::Color::RGB(255, 0, 0));
        self.renderer.fill_rect(rect::Rect::new(10, 10, 20, 20));
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