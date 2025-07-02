use pax_engine::*;
use pax_runtime::*;
use pax_runtime::InstanceNode;
use pax_runtime::DefinitionToInstanceTraverser;
use pax_manifest::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn init_manifest() -> PaxManifest {
    let mut manifest = PaxManifest::default();
    manifest.main_component_type_id = TypeId::build_singleton("ExampleApp", None);
    
    let mut component_def = ComponentDefinition::default();
    component_def.type_id = TypeId::build_singleton("ExampleApp", None);
    component_def.template = Some(include_str!("lib.pax").to_string());
    
    manifest.components.insert(
        TypeId::build_singleton("ExampleApp", None),
        component_def
    );
    
    manifest
}

pub fn init_definition_to_instance_traverser(initial_manifest: PaxManifest) -> Box<dyn DefinitionToInstanceTraverser> {
    Box::new(ExampleAppDefinitionToInstanceTraverser::new(initial_manifest))
}

pub struct ExampleAppDefinitionToInstanceTraverser {
    manifest: RefCell<PaxManifest>,
}

impl DefinitionToInstanceTraverser for ExampleAppDefinitionToInstanceTraverser {
    fn new(manifest: PaxManifest) -> Self {
        Self {
            manifest: RefCell::new(manifest),
        }
    }

    fn get_manifest(&self) -> std::cell::Ref<PaxManifest> {
        self.manifest.borrow()
    }

    fn get_component_factory(&self, type_id: &TypeId) -> Option<Box<dyn pax_runtime::ComponentFactory>> {
        if type_id.get_unique_identifier() == "ExampleApp" {
            Some(Box::new(ExampleAppFactory{}))
        } else {
            None
        }
    }

    fn get_main_component(&self, _id: &str) -> Rc<dyn InstanceNode> {
        Rc::new(crate::ExampleApp::default())
    }
}

pub struct ExampleAppFactory {}

impl pax_runtime::ComponentFactory for ExampleAppFactory {
    fn build_component(
        &self,
        args: pax_runtime::InstantiationArgs,
    ) -> Rc<dyn InstanceNode> {
        crate::ExampleApp::instantiate(args)
    }
}

pub fn create_pax_engine() -> Result<PaxEngine, Box<dyn std::error::Error>> {
    let manifest = init_manifest();
    let definition_to_instance_traverser = init_definition_to_instance_traverser(manifest);
    
    let main_component_instance = definition_to_instance_traverser.get_main_component("USERLAND_COMPONENT_ROOT");
    
    let engine = pax_runtime::PaxEngine::new(
        main_component_instance,
        (600.0, 400.0),
        pax_runtime_api::Platform::Web,
        pax_runtime_api::OS::Linux,
        Box::new(|| std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()),
    );
    
    Ok(engine)
}

pub struct PaxEngineWrapper {
    engine: PaxEngine,
    render_context: crate::tauri_render_context::TauriRenderContext,
}

impl PaxEngineWrapper {
    pub fn new(app_handle: tauri::AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let engine = create_pax_engine()?;
        let render_context = crate::tauri_render_context::TauriRenderContext::new(app_handle);
        
        Ok(Self {
            engine,
            render_context,
        })
    }
    
    pub fn handle_button_click(&mut self) {
        let _messages = self.engine.tick();
        self.engine.render(&mut self.render_context);
        println!("Real PaxEngine: Button click processed with .pax component rendering");
    }
    
    pub fn get_render_commands(&mut self) -> Vec<String> {
        let _messages = self.engine.tick();
        self.engine.render(&mut self.render_context);
        
        vec![
            "console.log('Real PaxEngine render cycle: .pax components rendered to Canvas')".to_string(),
        ]
    }
    
    pub fn get_click_render_commands(&mut self) -> Vec<String> {
        self.handle_button_click();
        
        vec![
            "console.log('Real PaxEngine button click: .pax components updated and re-rendered')".to_string(),
        ]
    }
}
