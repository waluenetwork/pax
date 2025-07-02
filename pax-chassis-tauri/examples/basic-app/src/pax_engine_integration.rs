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

    fn get_component_factory(&self, _type_id: &TypeId) -> Option<Box<dyn pax_runtime::ComponentFactory>> {
        None
    }

    fn get_main_component(&self, _id: &str) -> Rc<dyn InstanceNode> {
        Rc::new(crate::ExampleApp::default())
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
