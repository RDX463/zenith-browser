use serde::{Deserialize, Serialize};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

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
}

enum UserEvent {
    Ipc(IpcMsg),
    UrlChanged(String),
}

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::<UserEvent>::with_user_event();
    let proxy = event_loop.create_proxy();
    
    let window = WindowBuilder::new()
        .with_title("Zenith Browser")
        .with_inner_size(tao::dpi::LogicalSize::new(1200.0, 800.0))
        .build(&event_loop)
        .unwrap();

    let toolbar_height = 44;

    // UI Assets
    let ui_html = include_str!("ui/ui.html");
    let ui_css = include_str!("ui/ui.css");
    
    // Inject CSS into HTML
    let final_ui_html = ui_html.replace(
        "<link rel=\"stylesheet\" href=\"ui.css\">",
        &format!("<style>{}</style>", ui_css),
    );

    // 1. UI WebView (Address Bar)
    let proxy_ui = proxy.clone();
    let ui_webview = WebViewBuilder::new(&window)
        .with_bounds(wry::Rect {
            x: 0,
            y: 0,
            width: window.inner_size().width,
            height: toolbar_height as u32,
        })
        .with_html(final_ui_html)?
        .with_ipc_handler(move |msg| {
            if let Ok(ipc_msg) = serde_json::from_str::<IpcMsg>(&msg) {
                let _ = proxy_ui.send_event(UserEvent::Ipc(ipc_msg));
            }
        })
        .build()?;

    // 2. Content WebView
    let proxy_content = proxy.clone();
    let content_webview = WebViewBuilder::new(&window)
        .with_bounds(wry::Rect {
            x: 0,
            y: toolbar_height,
            width: window.inner_size().width,
            height: window.inner_size().height - toolbar_height as u32,
        })
        .with_url("https://www.google.com")?
        .with_on_page_load_handler(move |_event, url| {
            let _ = proxy_content.send_event(UserEvent::UrlChanged(url));
        })
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(UserEvent::Ipc(msg)) => match msg {
                IpcMsg::LoadUrl(url) => {
                    let _ = content_webview.load_url(&url);
                }
                IpcMsg::Back => {
                    // Logic for back (requires history or just javascript call)
                    let _ = content_webview.evaluate_script("window.history.back()");
                }
                IpcMsg::Forward => {
                    let _ = content_webview.evaluate_script("window.history.forward()");
                }
                IpcMsg::Reload => {
                    let _ = content_webview.evaluate_script("window.location.reload()");
                }
            },

            Event::UserEvent(UserEvent::UrlChanged(url)) => {
                // Update Address Bar in UI
                let script = format!("window.postMessage(JSON.stringify({{ type: 'update_url', payload: '{}' }}), '*')", url);
                let _ = ui_webview.evaluate_script(&script);
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
                let _ = ui_webview.set_bounds(wry::Rect {
                    x: 0,
                    y: 0,
                    width: size.width,
                    height: toolbar_height as u32,
                });
                
                let _ = content_webview.set_bounds(wry::Rect {
                    x: 0,
                    y: toolbar_height,
                    width: size.width,
                    height: size.height - toolbar_height as u32,
                });
            }

            _ => (),
        }
    });
}
