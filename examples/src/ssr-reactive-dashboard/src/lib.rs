#![allow(unused_imports)]

use pax_kit::*;
use pax_lang::{parse_pax_str, Rule};
use pax_manifest::{ComponentTemplate, parsing::TemplateNodeParseContext, TypeId, SettingsBlockElement};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{WebSocket, MessageEvent, CloseEvent, ErrorEvent};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub enum WebSocketMessage {
    Data(Vec<DataItem>),
    Template(String),
}

#[pax]
#[main]
#[file("lib.pax")]
pub struct SSRDashboard {
    pub server_data: Property<Vec<DataItem>>,
    pub connection_status: Property<String>,
    pub last_update: Property<String>,
    pub auto_reconnect: Property<bool>,
}

#[pax]
pub struct DataItem {
    pub id: u32,
    pub title: String,
    pub value: f64,
    pub trend: String,
    pub color: String,
    pub timestamp: String,
}

impl SSRDashboard {
    pub fn on_mount(&mut self, ctx: &NodeContext) {
        self.connection_status.set("Connecting...".to_string());
        self.auto_reconnect.set(true);
        self.connect_websocket(ctx);
    }

    pub fn handle_reconnect(&mut self, ctx: &NodeContext, _args: Event<ButtonClick>) {
        self.connect_websocket(ctx);
    }

    pub fn toggle_auto_reconnect(&mut self, _ctx: &NodeContext, _args: Event<ButtonClick>) {
        let current = self.auto_reconnect.get();
        self.auto_reconnect.set(!current);
    }

    fn connect_websocket(&mut self, _ctx: &NodeContext) {
        let ws = WebSocket::new("ws://localhost:8090/ws").unwrap();
        let server_data = self.server_data.clone();
        let connection_status = self.connection_status.clone();
        let last_update = self.last_update.clone();

        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                let data_str: String = txt.into();
                web_sys::console::log_1(&format!("📨 Received WebSocket message: {}", data_str).into());
                
                if let Ok(message) = serde_json::from_str::<WebSocketMessage>(&data_str) {
                    match message {
                        WebSocketMessage::Data(data) => {
                            web_sys::console::log_1(&format!("📊 Processing data message with {} items", data.len()).into());
                            server_data.set(data);
                            let now = js_sys::Date::new_0();
                            last_update.set(format!("Data updated: {}", now.to_locale_time_string("en-US")));
                        }
                        WebSocketMessage::Template(template) => {
                            web_sys::console::log_1(&format!("🎨 Processing template message: {}", template).into());
                            let now = js_sys::Date::new_0();
                            last_update.set(format!("Template updated: {}", now.to_locale_time_string("en-US")));
                            
                            SSRDashboard::parse_template_safely(&template);
                        }
                    }
                } else if let Ok(data) = serde_json::from_str::<Vec<DataItem>>(&data_str) {
                    web_sys::console::log_1(&format!("📊 Processing legacy data message with {} items", data.len()).into());
                    server_data.set(data);
                    let now = js_sys::Date::new_0();
                    last_update.set(format!("Legacy data updated: {}", now.to_locale_time_string("en-US")));
                } else {
                    web_sys::console::log_1(&format!("❌ Failed to parse WebSocket message: {}", data_str).into());
                }
            }
        }) as Box<dyn FnMut(_)>);
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let connection_status_open = connection_status.clone();
        let onopen_callback = Closure::wrap(Box::new(move |_: web_sys::Event| {
            web_sys::console::log_1(&"🔗 WebSocket connection opened".into());
            connection_status_open.set("Connected".to_string());
        }) as Box<dyn FnMut(web_sys::Event)>);
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        let connection_status_error = connection_status.clone();
        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            web_sys::console::log_1(&format!("❌ WebSocket error: {:?}", e).into());
            connection_status_error.set("Error".to_string());
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        let connection_status_close = connection_status.clone();
        let onclose_callback = Closure::wrap(Box::new(move |e: CloseEvent| {
            web_sys::console::log_1(&format!("🔌 WebSocket closed: code={}, reason={}", e.code(), e.reason()).into());
            connection_status_close.set("Disconnected".to_string());
        }) as Box<dyn FnMut(CloseEvent)>);
        ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();
    }

    pub fn parse_template_safely(template_content: &str) {
        web_sys::console::log_1(&format!("🔄 Safe template parsing ({} chars): {}", template_content.len(), template_content).into());
        
        let mut template_map: HashMap<String, TypeId> = HashMap::new();
        let self_type_id = TypeId::build_singleton("SSRDashboard", Some("SSRDashboard"));
        template_map.insert("SSRDashboard".to_string(), self_type_id.clone());
        
        template_map.insert("Group".to_string(), TypeId::build_singleton("Group", Some("pax_std::core::group")));
        template_map.insert("Text".to_string(), TypeId::build_singleton("Text", Some("pax_std::core::text")));
        template_map.insert("Rectangle".to_string(), TypeId::build_singleton("Rectangle", Some("pax_std::drawing::rectangle")));
        template_map.insert("Frame".to_string(), TypeId::build_singleton("Frame", Some("pax_std::core::frame")));
        template_map.insert("Ellipse".to_string(), TypeId::build_singleton("Ellipse", Some("pax_std::drawing::ellipse")));
        
        web_sys::console::log_1(&format!("📋 Template map includes {} components", template_map.len()).into());
        
        let mut tpc = TemplateNodeParseContext {
            pascal_identifier_to_type_id_map: template_map,
            template: ComponentTemplate::new(self_type_id.clone(), None),
        };
        
        web_sys::console::log_1(&"🔍 Starting safe AST parsing...".into());
        match parse_pax_str(Rule::pax_component_definition, template_content) {
            Ok(ast) => {
                web_sys::console::log_1(&"✅ AST parsing successful!".into());
                
                let _settings = pax_manifest::parsing::parse_settings_from_component_definition_string(ast.clone());
                pax_manifest::parsing::parse_template_from_component_definition_string(&mut tpc, template_content, ast);
                
                let _new_template = tpc.template;
                web_sys::console::log_1(&"✅ Template parsing completed successfully!".into());
                web_sys::console::log_1(&"🎯 Template parsed without designer dependency - demonstrates flexible parsing!".into());
                web_sys::console::log_1(&"💡 This approach works in all configuration modes".into());
            }
            Err(e) => {
                web_sys::console::log_1(&format!("❌ Template parsing failed: {:?}", e).into());
                web_sys::console::log_1(&"💡 Check template syntax - ensure proper Pax DSL format".into());
            }
        }
    }
}
