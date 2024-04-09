use std::{
    ffi::c_void,
    mem::size_of,
    sync::{Arc, Mutex, OnceLock},
};

use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::{
            BitBlt, CreateCompatibleDC, CreateDIBSection, DeleteObject, GetDC, ReleaseDC,
            SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBITMAP, HDC,
            SRCCOPY,
        },
        System::{LibraryLoader::GetModuleHandleW, SystemServices::*},
        UI::{
            HiDpi::{SetProcessDpiAwareness, PROCESS_DPI_AWARENESS},
            Input::{
                Ime::{
                    ImmGetContext, ImmReleaseContext, ImmSetCandidateWindow, CANDIDATEFORM,
                    CFS_FORCE_POSITION, CFS_POINT,
                },
                KeyboardAndMouse::{ReleaseCapture, SetCapture, SetFocus},
            },
            WindowsAndMessaging::*,
        },
    },
};

use wke_sys::*;

unsafe impl Send for Render {}
static RENDER: OnceLock<Arc<Mutex<Render>>> = OnceLock::new();

fn main() -> Result<()> {
    unsafe {
        let _ = SetProcessDpiAwareness(PROCESS_DPI_AWARENESS(2));

        wkeInit();
        let wv = wkeCreateWebView();
        wkeResize(wv, 600, 400);
        wkeLoadHTML(wv, c"<html><head><meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\" /></head> 你好<input type='text'></html>".as_ptr());
        // wkeLoadURL(wv, c"https://www.4399.com".as_ptr());
        wkeSetZoomFactor(wv, 1.5);
        wkeSetEditable(wv, 0);
        wkeFocus(wv);

        let instance = GetModuleHandleW(None)?;
        debug_assert!(instance.0 != 0);

        let window_class = w!("window");

        let wc = WNDCLASSW {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance.into(),
            lpszClassName: window_class,

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassW(&wc);
        debug_assert!(atom != 0);

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            window_class,
            w!("WKE"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_CLIPSIBLINGS,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            600,
            400,
            None,
            None,
            instance,
            None,
        );

        let _ = RENDER.set(Arc::new(Mutex::new(Render::init(hwnd.clone(), wv.clone()))));

        // SetWindowSubclass(
        //     hwnd,
        //     Some(parent_subclass_proc),
        //     (WM_USER + 0x64) as _,
        //     data as _,
        // );

        let mut message = MSG::default();

        while GetMessageW(&mut message, None, 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }

        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_SIZE => {
                if let Some(render) = RENDER.get() {
                    let mut render = render.lock().unwrap();
                    let mut rect = RECT::default();
                    GetClientRect(window, &mut rect).unwrap();
                    render.resize(rect.right - rect.left, rect.bottom - rect.top);
                }
                LRESULT(0)
            }
            WM_PAINT => {
                if let Some(render) = RENDER.get() {
                    let mut render = render.lock().unwrap();
                    render.render();
                    std::thread::sleep(std::time::Duration::from_millis(15));
                }
                LRESULT(0)
            }
            WM_LBUTTONDOWN | WM_MBUTTONDOWN | WM_RBUTTONDOWN | WM_LBUTTONDBLCLK
            | WM_MBUTTONDBLCLK | WM_RBUTTONDBLCLK | WM_LBUTTONUP | WM_MBUTTONUP | WM_RBUTTONUP
            | WM_MOUSEMOVE => {
                if let Some(render) = RENDER.get() {
                    let render = render.lock().unwrap();
                    if message == WM_LBUTTONDOWN
                        || message == WM_MBUTTONDOWN
                        || message == WM_RBUTTONDOWN
                    {
                        SetFocus(window);
                        SetCapture(window);
                    } else if message == WM_LBUTTONUP
                        || message == WM_MBUTTONUP
                        || message == WM_RBUTTONUP
                    {
                        let _ = ReleaseCapture();
                    }

                    let x = lparam.0 & 0xFFFF;
                    let y = lparam.0 >> 16 & 0xFFFF;

                    let mut flags = 0;

                    if wparam.0 as u32 & MK_CONTROL.0 > 0 {
                        flags |= wkeMouseFlags_WKE_CONTROL
                    }
                    if wparam.0 as u32 & MK_SHIFT.0 > 0 {
                        flags |= wkeMouseFlags_WKE_SHIFT
                    }
                    if wparam.0 as u32 & MK_LBUTTON.0 > 0 {
                        flags |= wkeMouseFlags_WKE_LBUTTON
                    }
                    if wparam.0 as u32 & MK_MBUTTON.0 > 0 {
                        flags |= wkeMouseFlags_WKE_MBUTTON
                    }
                    if wparam.0 as u32 & MK_RBUTTON.0 > 0 {
                        flags |= wkeMouseFlags_WKE_RBUTTON
                    }

                    wkeMouseEvent(render.webview, message, x as _, y as _, flags as u32);
                }
                LRESULT(0)
            }
            WM_IME_STARTCOMPOSITION => {
                if let Some(render) = RENDER.get() {
                    let render = render.lock().unwrap();

                    let caret = wkeGetCaret(render.webview);

                    let form = CANDIDATEFORM {
                        dwIndex: 0,
                        dwStyle: CFS_FORCE_POSITION | CFS_POINT,
                        ptCurrentPos: POINT { x: caret.x, y: caret.y},
                        ..Default::default()
                    };

                    let ime_context = ImmGetContext(HWND::default());
                    ImmSetCandidateWindow(ime_context, &form);
                    ImmReleaseContext(window, ime_context);
                }
                LRESULT(0)
            }
            WM_CHAR => {
                if let Some(render) = RENDER.get() {
                    let render = render.lock().unwrap();
                    let char_code = wparam.0 as u32;
                    let mut flags = 0;
                    if lparam.0 >> 16 & 0xffff & KF_REPEAT as isize > 0 {
                        flags |= wkeKeyFlags_WKE_REPEAT;
                    }
                    if lparam.0 >> 16 & 0xffff & KF_EXTENDED as isize > 0 {
                        flags |= wkeKeyFlags_WKE_EXTENDED;
                    }

                    wkeKeyPress(render.webview, char_code, flags as _, 0);
                }
                LRESULT(0)
            }
            WM_KEYDOWN => {
                if let Some(render) = RENDER.get() {
                    let render = render.lock().unwrap();
                    let virtual_key_code = wparam;
                    let mut flags = 0;

                    if lparam.0 >> 16 & 0xffff & KF_REPEAT as isize > 0 {
                        flags |= wkeKeyFlags_WKE_REPEAT;
                    }
                    if lparam.0 >> 16 & 0xffff & KF_EXTENDED as isize > 0 {
                        flags |= wkeKeyFlags_WKE_EXTENDED;
                    }

                    let _ = wkeKeyDown(
                        render.webview,
                        virtual_key_code.0 as _,
                        flags.try_into().unwrap(),
                        0,
                    );
                }

                LRESULT(0)
            }
            WM_KEYUP => {
                if let Some(render) = RENDER.get() {
                    let render = render.lock().unwrap();
                    let virtual_key_code = wparam;
                    let mut flags = 0;

                    if lparam.0 >> 16 & 0xffff & KF_REPEAT as isize > 0 {
                        flags |= wkeKeyFlags_WKE_REPEAT;
                    }
                    if lparam.0 >> 16 & 0xffff & KF_EXTENDED as isize > 0 {
                        flags |= wkeKeyFlags_WKE_EXTENDED;
                    }

                    wkeKeyUp(
                        render.webview,
                        virtual_key_code.0 as _,
                        flags.try_into().unwrap(),
                        0,
                    );
                }
                LRESULT(0)
            }
            WM_SETFOCUS => {
                if let Some(render) = RENDER.get() {
                    let render = render.lock().unwrap();
                    wkeFocus(render.webview);
                }
                LRESULT(0)
            }
            WM_KILLFOCUS => {
                if let Some(render) = RENDER.get() {
                    let render = render.lock().unwrap();
                    wkeUnfocus(render.webview);
                }
                LRESULT(0)
            }
            WM_DESTROY => {
                std::process::exit(0);
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
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
        let render = Self {
            window,
            cdc: unsafe { CreateCompatibleDC(HDC::default()) },
            bitmap: HBITMAP::default(),
            webview,
            width: unsafe { wkeWidth(webview) },
            height: unsafe { wkeHeight(webview) },
            pixels: std::ptr::null_mut(),
        };

        render
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
