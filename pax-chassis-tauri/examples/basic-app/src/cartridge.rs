use pax_engine::*;
use pax_runtime::*;
use pax_runtime::InstanceNode;
use pax_runtime::DefinitionToInstanceTraverser;
use pax_manifest::*;
use std::cell::RefCell;
use std::rc::Rc;

fn init_cartridge() -> Box<dyn pax_runtime::cartridge::PaxCartridge> {
    Box::new(ExampleAppCartridge {})
}

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

pub struct ExampleAppCartridge {}

impl pax_runtime::cartridge::PaxCartridge for ExampleAppCartridge {}

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
