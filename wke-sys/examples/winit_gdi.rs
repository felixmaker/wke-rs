use std::{ffi::c_void, mem::size_of};

use windows::Win32::{
    Foundation::HWND,
    Graphics::Gdi::{
        BitBlt, CreateCompatibleDC, CreateDIBSection, DeleteObject, GetDC, ReleaseDC, SelectObject,
        BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBITMAP, HDC, SRCCOPY,
    },
    UI::{
        Input::KeyboardAndMouse::{MapVirtualKeyW, MAPVK_VSC_TO_VK},
        WindowsAndMessaging::{
            WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEMOVE,
            WM_RBUTTONDOWN, WM_RBUTTONUP,
        },
    },
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, Event, Ime, Modifiers, MouseButton, WindowEvent},
    event_loop::EventLoop,
    platform::scancode::PhysicalKeyExtScancode,
    raw_window_handle::{HasWindowHandle, RawWindowHandle, Win32WindowHandle},
    window::Window,
};
use wke_sys::*;

fn main() {
    unsafe {
        let event_loop = EventLoop::new().unwrap();
        let window = Window::new(&event_loop).unwrap();

        window.set_ime_allowed(true);

        wkeInit();
        let wv = wkeCreateWebView();
        // wkeLoadHTML(wv, c"<html><head><meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\" /></head> 你好<input type='text'></html>".as_ptr());
        wkeLoadURL(wv, c"https://www.4399.com".as_ptr());
        wkeSetZoomFactor(wv, 1.5);
        wkeSetEditable(wv, 0);
        wkeFocus(wv);

        let RawWindowHandle::Win32(Win32WindowHandle { hwnd, .. }) =
            window.window_handle().unwrap().as_raw()
        else {
            unimplemented!()
        };

        let hwnd = HWND(hwnd.get());
        let mut render = Render::init(hwnd, wv);
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
                        render.resize(size.width as i32, size.height as i32)
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
                        render.render();
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

struct Render {
    window: HWND,
    cdc: HDC,
    bitmap: HBITMAP,
    webview: wkeWebView,
    width: i32,
    height: i32,
    pixels: *mut c_void,
}

impl Render {
    fn init(window: HWND, webview: wkeWebView) -> Self {
        Self {
            window,
            cdc: unsafe { CreateCompatibleDC(HDC::default()) },
            bitmap: HBITMAP::default(),
            webview,
            width: unsafe { wkeWidth(webview) },
            height: unsafe { wkeHeight(webview) },
            pixels: std::ptr::null_mut(),
        }
    }

    fn resize(&mut self, w: i32, h: i32) {
        if w < 1 || h < 1 {
            return;
        }
        unsafe {
            wkeResize(self.webview, w, h);
        }
        self.width = w;
        self.height = h;
        self.pixels = std::ptr::null_mut();
    }

    fn render(&mut self) {
        if self.pixels.is_null() {
            self.create_bitmap();
        }

        unsafe {
            wkePaint(self.webview, self.pixels, 0);
            let hdc = GetDC(self.window);
            BitBlt(hdc, 0, 0, self.width, self.height, self.cdc, 0, 0, SRCCOPY).unwrap();
            ReleaseDC(self.window, hdc);
        }
    }

    fn create_bitmap(&mut self) {
        let bi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: self.width,
                biHeight: -self.height,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let hbmp = unsafe {
            CreateDIBSection(
                HDC::default(),
                &bi,
                DIB_RGB_COLORS,
                &mut self.pixels,
                HWND::default(),
                0,
            )
            .unwrap()
        };

        unsafe { SelectObject(self.cdc, hbmp) };

        if !self.bitmap.is_invalid() {
            unsafe { DeleteObject(self.bitmap) };
        }

        self.bitmap = hbmp;
    }
}
