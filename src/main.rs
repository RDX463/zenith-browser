use serde::{Deserialize, Serialize};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
    dpi::{LogicalPosition, LogicalSize},
};
use wry::{WebViewBuilder, Rect};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TabInfo {
    id: u32,
    title: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
enum IpcMsg {
    #[serde(rename = "load_url")]
    LoadUrl(String),
    #[serde(rename = "back")]
    Back,
    #[serde(rename = "forward")]
    Forward,
    #[serde(rename = "reload")]
    Reload,
    #[serde(rename = "new_tab")]
    NewTab,
    #[serde(rename = "switch_tab")]
    SwitchTab(u32),
    #[serde(rename = "close_tab")]
    CloseTab(u32),
}

enum UserEvent {
    Ipc(IpcMsg),
    UrlChanged(u32, String),
    TitleChanged(u32, String),
}

fn main() -> wry::Result<()> {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    
    let window = WindowBuilder::new()
        .with_title("Zenith Browser")
        .with_inner_size(LogicalSize::new(1200.0, 800.0))
        .build(&event_loop)
        .unwrap();

    let ui_height = 76; // 38 (tabs) + 38 (toolbar)

    // UI Assets
    let ui_html = include_str!("ui/ui.html");
    let ui_css = include_str!("ui/ui.css");
    let home_html = include_str!("ui/home.html");
    
    // Inject CSS into HTML
    let final_ui_html = ui_html.replace(
        "<link rel=\"stylesheet\" href=\"ui.css\">",
        &format!("<style>{}</style>", ui_css),
    );

    // 1. UI WebView (Tabs + Address Bar)
    let proxy_ui = proxy.clone();
    let ui_webview = WebViewBuilder::new()
        .with_bounds(Rect {
            position: LogicalPosition::new(0.0, 0.0).into(),
            size: LogicalSize::new(window.inner_size().width as f64, ui_height as f64).into(),
        })
        .with_html(final_ui_html)
        .with_ipc_handler(move |request| {
            let msg = request.body();
            if let Ok(ipc_msg) = serde_json::from_str::<IpcMsg>(msg) {
                let _ = proxy_ui.send_event(UserEvent::Ipc(ipc_msg));
            }
        })
        .build(&window)?;

    // Tab Management State
    let mut tabs: HashMap<u32, wry::WebView> = HashMap::new();
    let mut tab_infos: Vec<TabInfo> = Vec::new();
    let mut active_tab_id: u32 = 0;
    let mut next_tab_id: u32 = 0;

    // Initial Tab creation logic (manual here to avoid closure borrow issues)
    {
        let id = next_tab_id;
        let p_url = proxy.clone();
        let p_title = proxy.clone();
        
        let webview = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0.0, ui_height as f64).into(),
                size: LogicalSize::new(window.inner_size().width as f64, (window.inner_size().height - ui_height as u32) as f64).into(),
            })
            .with_on_page_load_handler(move |_, url| {
                let _ = p_url.send_event(UserEvent::UrlChanged(id, url));
            })
            .with_document_title_changed_handler(move |title| {
                let _ = p_title.send_event(UserEvent::TitleChanged(id, title));
            })
            .with_html(home_html)
            .build(&window)?;

        tabs.insert(id, webview);
        tab_infos.push(TabInfo {
            id,
            title: "New Tab".to_string(),
            url: "zenith://home".to_string(),
        });
        active_tab_id = id;
        next_tab_id += 1;
    }

    // Helper to update UI tabs
    fn update_ui_tabs(ui: &wry::WebView, infos: &Vec<TabInfo>, active: u32) {
        let json = serde_json::json!({
            "type": "update_tabs",
            "payload": {
                "tabs": infos,
                "active_id": active
            }
        });
        let _ = ui.evaluate_script(&format!("window.postMessage(JSON.stringify({}), '*')", json));
    }

    update_ui_tabs(&ui_webview, &tab_infos, active_tab_id);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(UserEvent::Ipc(msg)) => match msg {
                IpcMsg::LoadUrl(url) => {
                    if let Some(wv) = tabs.get(&active_tab_id) {
                        let _ = wv.load_url(&url);
                    }
                }
                IpcMsg::Back => {
                    if let Some(wv) = tabs.get(&active_tab_id) {
                        let _ = wv.evaluate_script("window.history.back()");
                    }
                }
                IpcMsg::Forward => {
                    if let Some(wv) = tabs.get(&active_tab_id) {
                        let _ = wv.evaluate_script("window.history.forward()");
                    }
                }
                IpcMsg::Reload => {
                    if let Some(wv) = tabs.get(&active_tab_id) {
                        let _ = wv.evaluate_script("window.location.reload()");
                    }
                }
                IpcMsg::NewTab => {
                    let id = next_tab_id;
                    next_tab_id += 1;
                    
                    let p_url = proxy.clone();
                    let p_title = proxy.clone();
                    
                    let res = WebViewBuilder::new()
                        .with_bounds(Rect {
                            position: LogicalPosition::new(0.0, ui_height as f64).into(),
                            size: LogicalSize::new(window.inner_size().width as f64, (window.inner_size().height - ui_height as u32) as f64).into(),
                        })
                        .with_on_page_load_handler(move |_, url| {
                            let _ = p_url.send_event(UserEvent::UrlChanged(id, url));
                        })
                        .with_document_title_changed_handler(move |title| {
                            let _ = p_title.send_event(UserEvent::TitleChanged(id, title));
                        })
                        .with_html(home_html)
                        .build(&window);

                    if let Ok(webview) = res {
                        if let Some(wv) = tabs.get(&active_tab_id) {
                            let _ = wv.set_visible(false);
                        }
                        tabs.insert(id, webview);
                        tab_infos.push(TabInfo {
                            id,
                            title: "New Tab".to_string(),
                            url: "zenith://home".to_string(),
                        });
                        active_tab_id = id;
                        update_ui_tabs(&ui_webview, &tab_infos, active_tab_id);
                    }
                }
                IpcMsg::SwitchTab(id) => {
                    if id != active_tab_id && tabs.contains_key(&id) {
                        if let Some(wv) = tabs.get(&active_tab_id) {
                            let _ = wv.set_visible(false);
                        }
                        active_tab_id = id;
                        if let Some(wv) = tabs.get(&active_tab_id) {
                            let _ = wv.set_visible(true);
                            let _ = wv.focus();
                        }
                        update_ui_tabs(&ui_webview, &tab_infos, active_tab_id);
                    }
                }
                IpcMsg::CloseTab(id) => {
                    if tabs.len() > 1 {
                        tabs.remove(&id);
                        tab_infos.retain(|t| t.id != id);
                        if active_tab_id == id {
                            active_tab_id = tab_infos[0].id;
                            if let Some(wv) = tabs.get(&active_tab_id) {
                                let _ = wv.set_visible(true);
                            }
                        }
                        update_ui_tabs(&ui_webview, &tab_infos, active_tab_id);
                    }
                }
            },

            Event::UserEvent(UserEvent::UrlChanged(id, url)) => {
                if let Some(info) = tab_infos.iter_mut().find(|t| t.id == id) {
                    info.url = url.clone();
                    if id == active_tab_id {
                        let script = format!("window.postMessage(JSON.stringify({{ type: 'update_url', payload: '{}' }}), '*')", url);
                        let _ = ui_webview.evaluate_script(&script);
                    }
                }
            }
            
            Event::UserEvent(UserEvent::TitleChanged(id, title)) => {
                if let Some(info) = tab_infos.iter_mut().find(|t| t.id == id) {
                    info.title = title;
                    update_ui_tabs(&ui_webview, &tab_infos, active_tab_id);
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                let size = window.inner_size();
                let _ = ui_webview.set_bounds(Rect {
                    position: LogicalPosition::new(0.0, 0.0).into(),
                    size: LogicalSize::new(size.width as f64, ui_height as f64).into(),
                });
                
                for wv in tabs.values() {
                    let _ = wv.set_bounds(Rect {
                        position: LogicalPosition::new(0.0, ui_height as f64).into(),
                        size: LogicalSize::new(size.width as f64, (size.height - ui_height as u32) as f64).into(),
                    });
                }
            }

            _ => (),
        }
    });

    // Note: Ok(()) here is technically unreachable because event_loop.run never returns.
    #[allow(unreachable_code)]
    Ok(())
}
