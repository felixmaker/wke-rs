use pixels::{wgpu::TextureFormat, PixelsBuilder, SurfaceTexture};
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{MapVirtualKeyW, MAPVK_VSC_TO_VK},
    WindowsAndMessaging::{
        WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEMOVE, WM_MOUSEWHEEL,
        WM_RBUTTONDOWN, WM_RBUTTONUP,
    },
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, Event, Ime, Modifiers, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    platform::scancode::PhysicalKeyExtScancode,
    window::Window,
};
use wke_sys::*;

fn main() {
    unsafe {
        let event_loop = EventLoop::new().unwrap();
        let window = Window::new(&event_loop).unwrap();

        let mut pixels = {
            let PhysicalSize { width, height } = window.inner_size();
            let surface_texture = SurfaceTexture::new(width, height, &window);
            PixelsBuilder::new(width, height, surface_texture)
                .texture_format(TextureFormat::Bgra8UnormSrgb)
                .build()
                .unwrap()
        };

        window.set_ime_allowed(true);

        wkeInit();
        let wv = wkeCreateWebView();
        // wkeLoadHTML(wv, c"<html><head><meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\" /></head> 你好<input type='text'></html>".as_ptr());
        wkeLoadURL(wv, c"https://www.baidu.com".as_ptr());
        wkeSetZoomFactor(wv, 1.5);
        wkeSetEditable(wv, 0);
        wkeFocus(wv);

        let mut last_position = PhysicalPosition::default();
        let mut last_index: Option<(usize, usize)> = None;
        let mut modifier_state = Modifiers::default();
        let mut ime_enabled = false;

        event_loop
            .run(move |event, event_loop| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        event_loop.exit();
                    }
                    WindowEvent::Resized(size) => {
                        // render.resize(size.width as i32, size.height as i32)
                        let PhysicalSize { width, height } = size;
                        let _ = pixels.resize_surface(width, height);
                        let _ = pixels.resize_buffer(width, height);
                        wkeResize(wv, width as i32, height as i32);
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
                                wv,
                                message,
                                last_position.x as i32,
                                last_position.y as i32,
                                0,
                            );
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => match delta {
                        MouseScrollDelta::LineDelta(_, y) => {
                            wkeMouseWheel(wv, 0, 0, (y * 120.0) as i32, 0);
                        }
                        _ => {}
                    },
                    WindowEvent::Touch(touch) => {
                        println!("{:?}", touch);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        last_position = position;
                        wkeMouseEvent(wv, WM_MOUSEMOVE, position.x as i32, position.y as i32, 0);
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        if !ime_enabled {
                            if let Some(scancode) = event.physical_key.to_scancode() {
                                let code = MapVirtualKeyW(scancode, MAPVK_VSC_TO_VK);
                                let flag = if event.repeat {
                                    wkeKeyFlags_WKE_REPEAT
                                } else {
                                    wkeKeyFlags_WKE_EXTENDED
                                };
                                match event.state {
                                    ElementState::Pressed => {
                                        wkeKeyDown(wv, code, flag as u32, 0);
                                        if modifier_state.state().is_empty() {
                                            if let Some(text) = event.text {
                                                let text = widestring::U16String::from_str(&text);
                                                for c in text.as_slice() {
                                                    wkeKeyPress(
                                                        wv,
                                                        *c as u32,
                                                        wkeKeyFlags_WKE_EXTENDED as u32,
                                                        0,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    ElementState::Released => {
                                        wkeKeyUp(wv, code, flag as u32, 0);
                                    }
                                }
                            }
                        }
                    }
                    WindowEvent::ModifiersChanged(modifier) => {
                        modifier_state = modifier;
                    }
                    WindowEvent::Ime(Ime::Enabled) => ime_enabled = true,
                    WindowEvent::Ime(Ime::Disabled) => ime_enabled = false,
                    WindowEvent::Ime(Ime::Preedit(text, current_index)) => {
                        let text = widestring::U16String::from_str(&text);
                        let last = last_index.map(|x| x.0).unwrap_or(0);
                        for _ in 0..last {
                            wkeKeyDown(wv, 0x08, wkeKeyFlags_WKE_EXTENDED as u32, 0);
                        }
                        for c in text.as_slice() {
                            wkeKeyPress(wv, *c as u32, wkeKeyFlags_WKE_EXTENDED as u32, 0);
                        }
                        let rect = wkeGetCaret(wv);
                        window.set_ime_cursor_area(
                            PhysicalPosition::new(rect.x, rect.y),
                            PhysicalSize::new(rect.x + rect.w, rect.y + rect.h),
                        );
                        last_index = current_index;
                    }
                    WindowEvent::Ime(Ime::Commit(text)) => {
                        let text = widestring::U16String::from_str(&text);
                        for c in text.as_slice() {
                            wkeKeyPress(wv, *c as u32, wkeKeyFlags_WKE_EXTENDED as u32, 0);
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        wkePaint(wv, pixels.frame_mut().as_mut_ptr() as _, 0);
                        let _ = pixels.render();
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    std::thread::sleep(std::time::Duration::from_millis(15));
                    window.request_redraw();
                }
                _ => {}
            })
            .unwrap();
    }
}
