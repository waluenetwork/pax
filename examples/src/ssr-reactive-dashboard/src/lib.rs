#![allow(unused_imports)]

use pax_kit::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{WebSocket, MessageEvent, CloseEvent, ErrorEvent};

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
    pub fn on_mount(&mut self, _ctx: &NodeContext) {
        self.connection_status.set("Connecting...".to_string());
        self.auto_reconnect.set(true);
        self.connect_websocket();
    }

    pub fn handle_reconnect(&mut self, _ctx: &NodeContext, _args: Event<ButtonClick>) {
        self.connect_websocket();
    }

    pub fn toggle_auto_reconnect(&mut self, _ctx: &NodeContext, _args: Event<ButtonClick>) {
        let current = self.auto_reconnect.get();
        self.auto_reconnect.set(!current);
    }

    fn connect_websocket(&mut self) {
        let ws = WebSocket::new("ws://localhost:8090/ws").unwrap();
        let server_data = self.server_data.clone();
        let connection_status = self.connection_status.clone();
        let last_update = self.last_update.clone();

        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                let data_str: String = txt.into();
                if let Ok(data) = serde_json::from_str::<Vec<DataItem>>(&data_str) {
                    server_data.set(data);
                    let now = js_sys::Date::new_0();
                    last_update.set(format!("Last updated: {}", now.to_locale_time_string("en-US")));
                }
            }
        }) as Box<dyn FnMut(_)>);
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let onopen_callback = Closure::wrap(Box::new(move |_: web_sys::Event| {
            connection_status.set("Connected".to_string());
        }) as Box<dyn FnMut(web_sys::Event)>);
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    }
}
