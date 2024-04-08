/* automatically generated by rust-bindgen 0.69.4 */

pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub type utf8 = ::std::os::raw::c_char;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wkeRect {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub w: ::std::os::raw::c_int,
    pub h: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_wkeRect() {
    const UNINIT: ::std::mem::MaybeUninit<wkeRect> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<wkeRect>(),
        16usize,
        concat!("Size of: ", stringify!(wkeRect))
    );
    assert_eq!(
        ::std::mem::align_of::<wkeRect>(),
        4usize,
        concat!("Alignment of ", stringify!(wkeRect))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wkeRect),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(wkeRect),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).w) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(wkeRect),
            "::",
            stringify!(w)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).h) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(wkeRect),
            "::",
            stringify!(h)
        )
    );
}
pub const wkeMouseFlags_WKE_LBUTTON: wkeMouseFlags = 1;
pub const wkeMouseFlags_WKE_RBUTTON: wkeMouseFlags = 2;
pub const wkeMouseFlags_WKE_SHIFT: wkeMouseFlags = 4;
pub const wkeMouseFlags_WKE_CONTROL: wkeMouseFlags = 8;
pub const wkeMouseFlags_WKE_MBUTTON: wkeMouseFlags = 16;
pub type wkeMouseFlags = ::std::os::raw::c_int;
pub const wkeKeyFlags_WKE_EXTENDED: wkeKeyFlags = 256;
pub const wkeKeyFlags_WKE_REPEAT: wkeKeyFlags = 16384;
pub type wkeKeyFlags = ::std::os::raw::c_int;
pub const wkeMouseMsg_WKE_MSG_MOUSEMOVE: wkeMouseMsg = 512;
pub const wkeMouseMsg_WKE_MSG_LBUTTONDOWN: wkeMouseMsg = 513;
pub const wkeMouseMsg_WKE_MSG_LBUTTONUP: wkeMouseMsg = 514;
pub const wkeMouseMsg_WKE_MSG_LBUTTONDBLCLK: wkeMouseMsg = 515;
pub const wkeMouseMsg_WKE_MSG_RBUTTONDOWN: wkeMouseMsg = 516;
pub const wkeMouseMsg_WKE_MSG_RBUTTONUP: wkeMouseMsg = 517;
pub const wkeMouseMsg_WKE_MSG_RBUTTONDBLCLK: wkeMouseMsg = 518;
pub const wkeMouseMsg_WKE_MSG_MBUTTONDOWN: wkeMouseMsg = 519;
pub const wkeMouseMsg_WKE_MSG_MBUTTONUP: wkeMouseMsg = 520;
pub const wkeMouseMsg_WKE_MSG_MBUTTONDBLCLK: wkeMouseMsg = 521;
pub const wkeMouseMsg_WKE_MSG_MOUSEWHEEL: wkeMouseMsg = 522;
pub type wkeMouseMsg = ::std::os::raw::c_int;
pub type jsExecState = *mut ::std::os::raw::c_void;
pub type jsValue = ::std::os::raw::c_longlong;
pub type wkeWebView = *mut ::std::os::raw::c_void;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type bool_ = ::std::os::raw::c_uchar;
extern "C" {
    pub fn wkeInit();
}
extern "C" {
    pub fn wkeShutdown();
}
extern "C" {
    pub fn wkeUpdate();
}
extern "C" {
    pub fn wkeVersion() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn wkeVersionString() -> *const utf8;
}
pub type FILE_OPEN = ::std::option::Option<
    unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void,
>;
pub type FILE_CLOSE =
    ::std::option::Option<unsafe extern "C" fn(handle: *mut ::std::os::raw::c_void)>;
pub type FILE_SIZE =
    ::std::option::Option<unsafe extern "C" fn(handle: *mut ::std::os::raw::c_void) -> usize>;
pub type FILE_READ = ::std::option::Option<
    unsafe extern "C" fn(
        handle: *mut ::std::os::raw::c_void,
        buffer: *mut ::std::os::raw::c_void,
        size: usize,
    ) -> ::std::os::raw::c_int,
>;
pub type FILE_SEEK = ::std::option::Option<
    unsafe extern "C" fn(
        handle: *mut ::std::os::raw::c_void,
        offset: ::std::os::raw::c_int,
        origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn wkeSetFileSystem(
        pfn_open: FILE_OPEN,
        pfn_close: FILE_CLOSE,
        pfn_size: FILE_SIZE,
        pfn_read: FILE_READ,
        pfn_seek: FILE_SEEK,
    );
}
extern "C" {
    pub fn wkeCreateWebView() -> wkeWebView;
}
extern "C" {
    pub fn wkeGetWebView(name: *const ::std::os::raw::c_char) -> wkeWebView;
}
extern "C" {
    pub fn wkeDestroyWebView(webView: wkeWebView);
}
extern "C" {
    pub fn wkeWebViewName(webView: wkeWebView) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn wkeSetWebViewName(webView: wkeWebView, name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn wkeIsTransparent(webView: wkeWebView) -> bool_;
}
extern "C" {
    pub fn wkeSetTransparent(webView: wkeWebView, transparent: bool_);
}
extern "C" {
    pub fn wkeLoadURL(webView: wkeWebView, url: *const utf8);
}
extern "C" {
    pub fn wkeLoadURLW(webView: wkeWebView, url: *const wchar_t);
}
extern "C" {
    pub fn wkeLoadHTML(webView: wkeWebView, html: *const utf8);
}
extern "C" {
    pub fn wkeLoadHTMLW(webView: wkeWebView, html: *const wchar_t);
}
extern "C" {
    pub fn wkeLoadFile(webView: wkeWebView, filename: *const utf8);
}
extern "C" {
    pub fn wkeLoadFileW(webView: wkeWebView, filename: *const wchar_t);
}
extern "C" {
    pub fn wkeIsLoaded(webView: wkeWebView) -> bool_;
}
extern "C" {
    pub fn wkeIsLoadFailed(webView: wkeWebView) -> bool_;
}
extern "C" {
    pub fn wkeIsLoadComplete(webView: wkeWebView) -> bool_;
}
extern "C" {
    pub fn wkeIsDocumentReady(webView: wkeWebView) -> bool_;
}
extern "C" {
    pub fn wkeIsLoading(webView: wkeWebView) -> bool_;
}
extern "C" {
    pub fn wkeStopLoading(webView: wkeWebView);
}
extern "C" {
    pub fn wkeReload(webView: wkeWebView);
}
extern "C" {
    pub fn wkeTitle(webView: wkeWebView) -> *const utf8;
}
extern "C" {
    pub fn wkeTitleW(webView: wkeWebView) -> *const wchar_t;
}
extern "C" {
    pub fn wkeResize(webView: wkeWebView, w: ::std::os::raw::c_int, h: ::std::os::raw::c_int);
}
extern "C" {
    pub fn wkeWidth(webView: wkeWebView) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wkeHeight(webView: wkeWebView) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wkeContentsWidth(webView: wkeWebView) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wkeContentsHeight(webView: wkeWebView) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wkeSetDirty(webView: wkeWebView, dirty: bool_);
}
extern "C" {
    pub fn wkeIsDirty(webView: wkeWebView) -> bool_;
}
extern "C" {
    pub fn wkeAddDirtyArea(
        webView: wkeWebView,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn wkeLayoutIfNeeded(webView: wkeWebView);
}
extern "C" {
    pub fn wkePaint(
        webView: wkeWebView,
        bits: *mut ::std::os::raw::c_void,
        pitch: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn wkeCanGoBack(webView: wkeWebView) -> bool_;
}
extern "C" {
    pub fn wkeGoBack(webView: wkeWebView) -> bool_;
}
extern "C" {
    pub fn wkeCanGoForward(webView: wkeWebView) -> bool_;
}
extern "C" {
    pub fn wkeGoForward(webView: wkeWebView) -> bool_;
}
extern "C" {
    pub fn wkeSelectAll(webView: wkeWebView);
}
extern "C" {
    pub fn wkeCopy(webView: wkeWebView);
}
extern "C" {
    pub fn wkeCut(webView: wkeWebView);
}
extern "C" {
    pub fn wkePaste(webView: wkeWebView);
}
extern "C" {
    pub fn wkeDelete(webView: wkeWebView);
}
extern "C" {
    pub fn wkeSetCookieEnabled(webView: wkeWebView, enable: bool_);
}
extern "C" {
    pub fn wkeCookieEnabled(webView: wkeWebView) -> bool_;
}
extern "C" {
    pub fn wkeSetMediaVolume(webView: wkeWebView, volume: f32);
}
extern "C" {
    pub fn wkeMediaVolume(webView: wkeWebView) -> f32;
}
extern "C" {
    pub fn wkeMouseEvent(
        webView: wkeWebView,
        message: ::std::os::raw::c_uint,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> bool_;
}
extern "C" {
    pub fn wkeContextMenuEvent(
        webView: wkeWebView,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> bool_;
}
extern "C" {
    pub fn wkeMouseWheel(
        webView: wkeWebView,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        delta: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> bool_;
}
extern "C" {
    pub fn wkeKeyUp(
        webView: wkeWebView,
        virtualKeyCode: ::std::os::raw::c_uint,
        flags: ::std::os::raw::c_uint,
        systemKey: bool_,
    ) -> bool_;
}
extern "C" {
    pub fn wkeKeyDown(
        webView: wkeWebView,
        virtualKeyCode: ::std::os::raw::c_uint,
        flags: ::std::os::raw::c_uint,
        systemKey: bool_,
    ) -> bool_;
}
extern "C" {
    pub fn wkeKeyPress(
        webView: wkeWebView,
        charCode: ::std::os::raw::c_uint,
        flags: ::std::os::raw::c_uint,
        systemKey: bool_,
    ) -> bool_;
}
extern "C" {
    pub fn wkeFocus(webView: wkeWebView);
}
extern "C" {
    pub fn wkeUnfocus(webView: wkeWebView);
}
extern "C" {
    pub fn wkeGetCaret(webView: wkeWebView) -> wkeRect;
}
extern "C" {
    pub fn wkeRunJS(webView: wkeWebView, script: *const utf8) -> jsValue;
}
extern "C" {
    pub fn wkeRunJSW(webView: wkeWebView, script: *const wchar_t) -> jsValue;
}
extern "C" {
    pub fn wkeGlobalExec(webView: wkeWebView) -> jsExecState;
}
extern "C" {
    pub fn wkeSleep(webView: wkeWebView);
}
extern "C" {
    pub fn wkeAwaken(webView: wkeWebView);
}
extern "C" {
    pub fn wkeIsAwake(webView: wkeWebView) -> bool_;
}
extern "C" {
    pub fn wkeSetZoomFactor(webView: wkeWebView, factor: f32);
}
extern "C" {
    pub fn wkeZoomFactor(webView: wkeWebView) -> f32;
}
extern "C" {
    pub fn wkeSetEditable(webView: wkeWebView, editable: bool_);
}
pub type jsNativeFunction =
    ::std::option::Option<unsafe extern "fastcall" fn(es: jsExecState) -> jsValue>;
pub const jsType_JSTYPE_NUMBER: jsType = 0;
pub const jsType_JSTYPE_STRING: jsType = 1;
pub const jsType_JSTYPE_BOOLEAN: jsType = 2;
pub const jsType_JSTYPE_OBJECT: jsType = 3;
pub const jsType_JSTYPE_FUNCTION: jsType = 4;
pub const jsType_JSTYPE_UNDEFINED: jsType = 5;
pub type jsType = ::std::os::raw::c_int;
extern "C" {
    pub fn jsBindFunction(
        name: *const ::std::os::raw::c_char,
        fn_: jsNativeFunction,
        argCount: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn jsBindGetter(name: *const ::std::os::raw::c_char, fn_: jsNativeFunction);
}
extern "C" {
    pub fn jsBindSetter(name: *const ::std::os::raw::c_char, fn_: jsNativeFunction);
}
extern "C" {
    pub fn jsArgCount(es: jsExecState) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jsArgType(es: jsExecState, argIdx: ::std::os::raw::c_int) -> jsType;
}
extern "C" {
    pub fn jsArg(es: jsExecState, argIdx: ::std::os::raw::c_int) -> jsValue;
}
extern "C" {
    pub fn jsTypeOf(v: jsValue) -> jsType;
}
extern "C" {
    pub fn jsIsNumber(v: jsValue) -> bool_;
}
extern "C" {
    pub fn jsIsString(v: jsValue) -> bool_;
}
extern "C" {
    pub fn jsIsBoolean(v: jsValue) -> bool_;
}
extern "C" {
    pub fn jsIsObject(v: jsValue) -> bool_;
}
extern "C" {
    pub fn jsIsFunction(v: jsValue) -> bool_;
}
extern "C" {
    pub fn jsIsUndefined(v: jsValue) -> bool_;
}
extern "C" {
    pub fn jsIsNull(v: jsValue) -> bool_;
}
extern "C" {
    pub fn jsIsArray(v: jsValue) -> bool_;
}
extern "C" {
    pub fn jsIsTrue(v: jsValue) -> bool_;
}
extern "C" {
    pub fn jsIsFalse(v: jsValue) -> bool_;
}
extern "C" {
    pub fn jsToInt(es: jsExecState, v: jsValue) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jsToFloat(es: jsExecState, v: jsValue) -> f32;
}
extern "C" {
    pub fn jsToDouble(es: jsExecState, v: jsValue) -> f64;
}
extern "C" {
    pub fn jsToBoolean(es: jsExecState, v: jsValue) -> bool_;
}
extern "C" {
    pub fn jsToString(es: jsExecState, v: jsValue) -> *const utf8;
}
extern "C" {
    pub fn jsToStringW(es: jsExecState, v: jsValue) -> *const wchar_t;
}
extern "C" {
    pub fn jsInt(n: ::std::os::raw::c_int) -> jsValue;
}
extern "C" {
    pub fn jsFloat(f: f32) -> jsValue;
}
extern "C" {
    pub fn jsDouble(d: f64) -> jsValue;
}
extern "C" {
    pub fn jsBoolean(b: bool_) -> jsValue;
}
extern "C" {
    pub fn jsUndefined() -> jsValue;
}
extern "C" {
    pub fn jsNull() -> jsValue;
}
extern "C" {
    pub fn jsTrue() -> jsValue;
}
extern "C" {
    pub fn jsFalse() -> jsValue;
}
extern "C" {
    pub fn jsString(es: jsExecState, str_: *const utf8) -> jsValue;
}
extern "C" {
    pub fn jsStringW(es: jsExecState, str_: *const wchar_t) -> jsValue;
}
extern "C" {
    pub fn jsObject(es: jsExecState) -> jsValue;
}
extern "C" {
    pub fn jsArray(es: jsExecState) -> jsValue;
}
extern "C" {
    pub fn jsFunction(
        es: jsExecState,
        fn_: jsNativeFunction,
        argCount: ::std::os::raw::c_uint,
    ) -> jsValue;
}
extern "C" {
    pub fn jsGlobalObject(es: jsExecState) -> jsValue;
}
extern "C" {
    pub fn jsEval(es: jsExecState, str_: *const utf8) -> jsValue;
}
extern "C" {
    pub fn jsEvalW(es: jsExecState, str_: *const wchar_t) -> jsValue;
}
extern "C" {
    pub fn jsCall(
        es: jsExecState,
        func: jsValue,
        thisObject: jsValue,
        args: *mut jsValue,
        argCount: ::std::os::raw::c_int,
    ) -> jsValue;
}
extern "C" {
    pub fn jsCallGlobal(
        es: jsExecState,
        func: jsValue,
        args: *mut jsValue,
        argCount: ::std::os::raw::c_int,
    ) -> jsValue;
}
extern "C" {
    pub fn jsGet(es: jsExecState, object: jsValue, prop: *const ::std::os::raw::c_char) -> jsValue;
}
extern "C" {
    pub fn jsSet(es: jsExecState, object: jsValue, prop: *const ::std::os::raw::c_char, v: jsValue);
}
extern "C" {
    pub fn jsGetGlobal(es: jsExecState, prop: *const ::std::os::raw::c_char) -> jsValue;
}
extern "C" {
    pub fn jsSetGlobal(es: jsExecState, prop: *const ::std::os::raw::c_char, v: jsValue);
}
extern "C" {
    pub fn jsGetAt(es: jsExecState, object: jsValue, index: ::std::os::raw::c_int) -> jsValue;
}
extern "C" {
    pub fn jsSetAt(es: jsExecState, object: jsValue, index: ::std::os::raw::c_int, v: jsValue);
}
extern "C" {
    pub fn jsGetLength(es: jsExecState, object: jsValue) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jsSetLength(es: jsExecState, object: jsValue, length: ::std::os::raw::c_int);
}
extern "C" {
    pub fn jsGetWebView(es: jsExecState) -> wkeWebView;
}
extern "C" {
    pub fn jsGC();
}