use std::{ffi::c_void, num::NonZeroU32, rc::Rc};

use softbuffer::{Context, Surface};
use windows_sys::Win32::UI::{
    Input::KeyboardAndMouse::{MapVirtualKeyW, MAPVK_VSC_TO_VK},
    WindowsAndMessaging::{
        WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEMOVE, WM_RBUTTONDOWN,
        WM_RBUTTONUP,
    },
};
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, Ime, Modifiers, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    platform::scancode::PhysicalKeyExtScancode,
    window::Window,
};
use wke_sys::*;

#[derive(Default)]
struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    webview: Option<Rc<*mut c_void>>,
    last_position: PhysicalPosition<f64>,
    last_index: Option<(usize, usize)>,
    modifier_state: Modifiers,
    ime_enabled: bool,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Rc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        window.set_ime_allowed(true);
        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();
        self.window = Some(window);
        self.webview = Some(Rc::new(unsafe {
            wkeInit();
            let wv = wkeCreateWebView();
            // wkeLoadHTML(wv, c"<html><head><meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\" /></head> 你好<input type='text'></html>".as_ptr());
            wkeLoadURL(wv, c"https://www.baidu.com".as_ptr());
            wkeSetZoomFactor(wv, 1.5);
            wkeSetEditable(wv, 0);
            wkeFocus(wv);
            wv
        }));
        self.surface = Some(surface);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        unsafe {
            let surface = self.surface.as_mut().unwrap();
            let wv = self.webview.as_mut().unwrap().clone();
            let window = self.window.as_mut().unwrap().clone();
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(size) => {
                    let PhysicalSize { width, height } = size;
                    wkeResize(*wv, width as i32, height as i32);
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    let message = match (button, state) {
                        (MouseButton::Left, ElementState::Pressed) => Some(WM_LBUTTONDOWN),
                        (MouseButton::Left, ElementState::Released) => Some(WM_LBUTTONUP),
                        (MouseButton::Right, ElementState::Pressed) => Some(WM_RBUTTONDOWN),
                        (MouseButton::Right, ElementState::Released) => Some(WM_RBUTTONUP),
                        (MouseButton::Middle, ElementState::Pressed) => Some(WM_MBUTTONDOWN),
                        (MouseButton::Middle, ElementState::Released) => Some(WM_MBUTTONUP),
                        _ => None,
                    };
                    if let Some(message) = message {
                        wkeMouseEvent(
                            *wv,
                            message,
                            self.last_position.x as i32,
                            self.last_position.y as i32,
                            0,
                        );
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        wkeMouseWheel(*wv, 0, 0, (y * 120.0) as i32, 0);
                    }
                    _ => {}
                },
                WindowEvent::Touch(touch) => {
                    println!("{:?}", touch);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    self.last_position = position;
                    wkeMouseEvent(*wv, WM_MOUSEMOVE, position.x as i32, position.y as i32, 0);
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if !self.ime_enabled {
                        if let Some(scancode) = event.physical_key.to_scancode() {
                            let code = MapVirtualKeyW(scancode, MAPVK_VSC_TO_VK);
                            let flag = if event.repeat {
                                wkeKeyFlags_WKE_REPEAT
                            } else {
                                wkeKeyFlags_WKE_EXTENDED
                            };
                            match event.state {
                                ElementState::Pressed => {
                                    wkeKeyDown(*wv, code, flag as u32, 0);
                                    if self.modifier_state.state().is_empty() {
                                        if let Some(text) = event.text {
                                            let text = widestring::U16String::from_str(&text);
                                            for c in text.as_slice() {
                                                wkeKeyPress(
                                                    *wv,
                                                    *c as u32,
                                                    wkeKeyFlags_WKE_EXTENDED as u32,
                                                    0,
                                                );
                                            }
                                        }
                                    }
                                }
                                ElementState::Released => {
                                    wkeKeyUp(*wv, code, flag as u32, 0);
                                }
                            }
                        }
                    }
                }
                WindowEvent::ModifiersChanged(modifier) => {
                    self.modifier_state = modifier;
                }
                WindowEvent::Ime(Ime::Enabled) => self.ime_enabled = true,
                WindowEvent::Ime(Ime::Disabled) => self.ime_enabled = false,
                WindowEvent::Ime(Ime::Preedit(text, current_index)) => {
                    let text = widestring::U16String::from_str(&text);
                    let last = self.last_index.map(|x| x.0).unwrap_or(0);
                    for _ in 0..last {
                        wkeKeyDown(*wv, 0x08, wkeKeyFlags_WKE_EXTENDED as u32, 0);
                    }
                    for c in text.as_slice() {
                        wkeKeyPress(*wv, *c as u32, wkeKeyFlags_WKE_EXTENDED as u32, 0);
                    }
                    let rect = wkeGetCaret(*wv);
                    window.set_ime_cursor_area(
                        PhysicalPosition::new(rect.x, rect.y),
                        PhysicalSize::new(rect.x + rect.w, rect.y + rect.h),
                    );
                    self.last_index = current_index;
                }
                WindowEvent::Ime(Ime::Commit(text)) => {
                    let text = widestring::U16String::from_str(&text);
                    for c in text.as_slice() {
                        wkeKeyPress(*wv, *c as u32, wkeKeyFlags_WKE_EXTENDED as u32, 0);
                    }
                }
                WindowEvent::RedrawRequested => {
                    let PhysicalSize { width, height } = window.inner_size();
                    let _ = surface.resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    );
                    let mut buffer = surface.buffer_mut().unwrap();
                    wkePaint(*wv, buffer.as_mut_ptr() as _, 0);
                    let _ = buffer.present();
                }
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = self.window.as_mut().unwrap().clone();

        std::thread::sleep(std::time::Duration::from_millis(15));
        window.request_redraw();
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
