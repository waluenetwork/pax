use pax_engine::*;
use pax_engine::api::*;
use pax_std::*;
use pax_runtime::InstanceNode;

pub mod pax_engine_integration;

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

impl InstanceNode for ExampleApp {
    fn instantiate(_args: pax_runtime::InstantiationArgs) -> Rc<Self> {
        Rc::new(ExampleApp::default())
    }
    
    fn update(_self: Rc<Self>, _expanded_node: &Rc<pax_runtime::ExpandedNode>, _context: &Rc<pax_runtime::RuntimeContext>) {}
    
    fn render(&self, _expanded_node: &pax_runtime::ExpandedNode, _context: &Rc<pax_runtime::RuntimeContext>, rc: &mut dyn pax_runtime_api::RenderContext) {
        use kurbo::Rect;
        use pax_runtime::api::Fill;
        
        rc.clear(0);
        
        let rect = Rect::new(50.0, 50.0, 150.0 + self.rect_width.get().to_pixels(), 100.0);
        let color = self.rect_color.get();
        let fill = Fill::Solid(pax_runtime::api::Color::rgba(
            color.red as f64 / 255.0,
            color.green as f64 / 255.0,
            color.blue as f64 / 255.0,
            1.0
        ));
        rc.fill(0, rect.into(), &fill);
        
        let text_rect = Rect::new(50.0, 120.0, 300.0, 150.0);
        let text_fill = Fill::Solid(pax_runtime::api::Color::rgba(0.0, 0.0, 0.0, 1.0));
        rc.fill(0, text_rect.into(), &text_fill);
    }
    
    fn handle_mount(_self: Rc<Self>, _expanded_node: &Rc<pax_runtime::ExpandedNode>, _context: &Rc<pax_runtime::RuntimeContext>) {}
    
    fn handle_unmount(&self, _expanded_node: &Rc<pax_runtime::ExpandedNode>, _context: &Rc<pax_runtime::RuntimeContext>) {}
    
    fn resolve_debug(&self, _f: &mut std::fmt::Formatter, _expanded_node: Option<&pax_runtime::ExpandedNode>) -> std::fmt::Result {
        Ok(())
    }
    
    fn base(&self) -> &pax_runtime::BaseInstance {
        use std::sync::LazyLock;
        static BASE: LazyLock<pax_runtime::BaseInstance> = LazyLock::new(|| {
            let args = pax_runtime::InstantiationArgs::default();
            let flags = pax_runtime::InstanceFlags::new();
            pax_runtime::BaseInstance::new(args, flags)
        });
        &BASE
    }
    
    fn base_mut(&mut self) -> &mut pax_runtime::BaseInstance {
        use std::sync::LazyLock;
        static mut BASE: LazyLock<pax_runtime::BaseInstance> = LazyLock::new(|| {
            let args = pax_runtime::InstantiationArgs::default();
            let flags = pax_runtime::InstanceFlags::new();
            pax_runtime::BaseInstance::new(args, flags)
        });
        unsafe { &mut BASE }
    }
}
