#![cfg(feature = "wasm-backend")]

use cursive_core::{
    event::Event,
    Vec2,
    theme,
};
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::prelude::*;
use crate::backend;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
struct TextColorPairs {
    data: Vec<TextColorPair>,
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TextColorPair  {
    text: String ,
    color: ColorPair,
}

impl TextColorPair {
    pub fn new(text: String, color: ColorPair) -> Self {
        Self {
            text,
            color,
        }
    }
}

impl Clone for TextColorPair {
    fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            color: self.color.clone(),
        }
    }
}


#[wasm_bindgen(module = "/src/backends/canvas.js")]
extern "C" {
    fn paint(buffer: JsValue);
}

/// Backend using wasm.
pub struct Backend {
    canvas: HtmlCanvasElement,
    color: RefCell<ColorPair>,
    events: Rc<RefCell<VecDeque<Event>>>,
    buffer: RefCell<Vec<TextColorPair>>,
}
impl Backend {
    /// Creates a new Cursive root using a wasm backend.
    pub fn init() -> std::io::Result<Box<dyn backend::Backend>> {
        let document = web_sys::window()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get window",
            ))?
            .document()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get document",
            ))?;
        let canvas = document.get_element_by_id("cursive-wasm-canvas")
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get window",
            ))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to cast canvas",
            ))?;
        canvas.set_width(1000);
        canvas.set_height(1000);

        let color = cursive_to_color_pair(theme::ColorPair {
            front: theme::Color::Light(theme::BaseColor::Black),
            back:theme::Color::Dark(theme::BaseColor::Green),
        });

        let events = Rc::new(RefCell::new(VecDeque::new()));
         let cloned = events.clone();
         let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
             for c in event.key().chars() {
                cloned.borrow_mut().push_back(Event::Char(c));
             }
         }) as Box<dyn FnMut(_)>);
         canvas.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .map_err(|_| std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to add event listener",
            ))?;
         closure.forget();

        let buffer = vec![TextColorPair::new(' '.to_string(), color.clone()); 1_000_000];

        let c = Backend {
            canvas,
            color: RefCell::new(color),
            events,     
            buffer: RefCell::new(buffer),
         };
        Ok(Box::new(c))
    }
}

impl cursive_core::backend::Backend for Backend {
    fn poll_event(self: &mut Backend) -> Option<Event> {
        self.events.borrow_mut().pop_front()
    }

    fn set_title(self: &mut Backend, title: String) {
        self.canvas.set_title(&title);
    }

    fn refresh(self: &mut Backend) {
        let data = self.buffer.borrow().clone();
        let pairs = TextColorPairs { data };
        paint(serde_wasm_bindgen::to_value(&pairs).unwrap());
    }

    fn has_colors(self: &Backend) -> bool {
        true
    }

    fn screen_size(self: &Backend) -> Vec2 {
        Vec2::new(self.canvas.width() as usize, self.canvas.height() as usize)
    }

    fn print_at(self: &Backend, pos: Vec2, text: &str) {
        let color = (*self.color.borrow()).clone();
        let mut buffer = self.buffer.borrow_mut();
        for (i, c) in text.chars().enumerate() {
            let x = pos.x + i;
            buffer[1000 * pos.y + x] = TextColorPair::new(c.to_string(), color.clone());
        }
    }

    fn clear(self: &Backend, _color: cursive_core::theme::Color) {
    }

    fn set_color(self: &Backend, color_pair: cursive_core::theme::ColorPair) -> cursive_core::theme::ColorPair {
        let mut color = self.color.borrow_mut();
        *color = cursive_to_color_pair(color_pair);
        color_pair
    }

    fn set_effect(self: &Backend, _: cursive_core::theme::Effect) {
    }

    fn unset_effect(self: &Backend, _: cursive_core::theme::Effect) {
    }

    fn name(&self) -> &str {
        "cursive-wasm-backend"
    }
}


/// Type of hex color which starts with #.
pub type Color = String;

/// Type of color pair.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)] 
pub struct ColorPair {
    /// Foreground text color.
    pub front: Color,
    /// Background color.
    pub back: Color,
}

/// Convert cursive color to hex color.
pub fn cursive_to_color(color: theme::Color) -> Color {
    match color {
        theme::Color::Dark(theme::BaseColor::Black) => "#000000".to_string(),
        theme::Color::Dark(theme::BaseColor::Red) => "#800000".to_string(),
        theme::Color::Dark(theme::BaseColor::Green) => "#008000".to_string(),
        theme::Color::Dark(theme::BaseColor::Yellow) => "#808000".to_string(),
        theme::Color::Dark(theme::BaseColor::Blue) => "#000080".to_string(),
        theme::Color::Dark(theme::BaseColor::Magenta) => "#800080".to_string(),
        theme::Color::Dark(theme::BaseColor::Cyan) => "#008080".to_string(),
        theme::Color::Dark(theme::BaseColor::White) => "#c0c0c0".to_string(),
        theme::Color::Light(theme::BaseColor::Black) => "#808080".to_string(),
        theme::Color::Light(theme::BaseColor::Red) => "#ff0000".to_string(),
        theme::Color::Light(theme::BaseColor::Green) => "#00ff00".to_string(),
        theme::Color::Light(theme::BaseColor::Yellow) => "#ffff00".to_string(),
        theme::Color::Light(theme::BaseColor::Blue) => "#0000ff".to_string(),
        theme::Color::Light(theme::BaseColor::Magenta) => "#ff00ff".to_string(),
        theme::Color::Light(theme::BaseColor::Cyan) => "#00ffff".to_string(),
        theme::Color::Light(theme::BaseColor::White) => "#ffffff".to_string(),
        theme::Color::Rgb(r, g, b) => format!("#{:02x}{:02x}{:02x}", r, g, b).to_string(),
        theme::Color::RgbLowRes(r,g ,b ) => format!("#{:01x}{:01x}{:01x}", r, g, b).to_string(),
        theme::Color::TerminalDefault => "#00ff00".to_string(),
    }
}

/// Convert cursive color pair to hex color pair.
pub fn cursive_to_color_pair(c: theme::ColorPair) -> ColorPair {
    ColorPair {
        front: cursive_to_color(c.front),
        back: cursive_to_color(c.back),
    }
}
