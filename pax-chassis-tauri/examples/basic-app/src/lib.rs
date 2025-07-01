use pax_engine::*;
use pax_engine::api::*;
use pax_std::*;

#[pax]
#[engine_import_path("pax_engine")]
#[file("lib.pax")]
pub struct ExampleApp {
    pub button_text: Property<String>,
    pub rect_color: Property<Color>,
    pub rect_width: Property<Size>,
    pub click_count: Property<usize>,
}

impl ExampleApp {
    pub fn handle_button_click(&mut self, _ctx: &NodeContext, _args: Event<Click>) {
        let current_count = self.click_count.get();
        self.click_count.set(current_count + 1);
        
        self.button_text.set(format!("Clicked {} times!", current_count + 1));
        
        let new_width = Size::Pixels(50.0 + (current_count as f64 * 20.0) % 200.0);
        self.rect_width.set(new_width);
        
        let colors = [
            Color::rgb(255, 100, 100),
            Color::rgb(100, 255, 100), 
            Color::rgb(100, 100, 255),
            Color::rgb(255, 255, 100),
        ];
        let color_index = current_count % colors.len();
        self.rect_color.set(colors[color_index]);
    }
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            button_text: Property::new("Click me!".to_string()),
            rect_color: Property::new(Color::rgb(200, 200, 200)),
            rect_width: Property::new(Size::Pixels(100.0)),
            click_count: Property::new(0),
        }
    }
}
