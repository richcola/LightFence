#![windows_subsystem = "windows"]

use windows::{
    core::w,
    Win32::{
        Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::{
            Direct2D::{
                D2D1CreateFactory,
                D2D1_FACTORY_TYPE_SINGLE_THREADED,
                ID2D1Factory,
                ID2D1HwndRenderTarget,
                D2D1_RENDER_TARGET_PROPERTIES,
                D2D1_HWND_RENDER_TARGET_PROPERTIES,
                D2D1_RENDER_TARGET_TYPE_DEFAULT,
                D2D1_PRESENT_OPTIONS,
            },
            Direct2D::Common::{
                D2D1_PIXEL_FORMAT,
                D2D1_ALPHA_MODE_PREMULTIPLIED,
                D2D1_COLOR_F,
                D2D_SIZE_U,
            },
            Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM,
            DirectWrite::{
                DWriteCreateFactory,
                IDWriteFactory,
                DWRITE_FACTORY_TYPE_SHARED,
            },
        },
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW,
            PostQuitMessage, RegisterClassW, TranslateMessage, MSG, WNDCLASSW,
            WINDOW_EX_STYLE, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
            WM_PAINT, WM_DESTROY, SetLayeredWindowAttributes,
            LWA_ALPHA, GetClientRect, WS_EX_LAYERED
        },
    },
};

static mut D2D_FACTORY: Option<ID2D1Factory> = None;
static mut DWRITE_FACTORY: Option<IDWriteFactory> = None;
static mut RENDER_TARGET: Option<ID2D1HwndRenderTarget> = None;

#[allow(dead_code)]
struct FenceConfig {
    title: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Default for FenceConfig {
    fn default() -> Self {
        Self {
            title: "my fence".to_string(),
            x: 100,
            y: 100,
            width: 300,
            height: 200,
        }
    }
}

// window procedure
unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            render_fence(hwnd);
            LRESULT(0)
        }
        WM_DESTROY => {  // WM_DESTROY
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

#[allow(static_mut_refs)]
unsafe fn create_render_target(hwnd: HWND) {
    if let Some(factory) = &D2D_FACTORY {
        let mut rect = RECT::default();
        let _ = GetClientRect(hwnd, &mut rect);

        let width = (rect.right - rect.left) as u32;
        let height = (rect.bottom - rect.top) as u32;

        let properties = D2D1_RENDER_TARGET_PROPERTIES {
            r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            ..Default::default()
        };

        let hwnd_properties = D2D1_HWND_RENDER_TARGET_PROPERTIES {
            hwnd,
            pixelSize: D2D_SIZE_U { width, height },
            presentOptions: D2D1_PRESENT_OPTIONS(0),
        };

        let rt = factory.CreateHwndRenderTarget(&properties, &hwnd_properties).expect("Failed to create render target");

        RENDER_TARGET = Some(rt);
    }
}

#[allow(static_mut_refs)]
unsafe fn render_fence(hwnd: HWND) {
    if D2D_FACTORY.is_none() {
        return;
    }

    if RENDER_TARGET.is_none() {
        create_render_target(hwnd);
    }

    // render
    if let Some(rt) = &RENDER_TARGET {
        rt.BeginDraw();

        let color = D2D1_COLOR_F {
            r: 0.2,
            g: 0.2,
            b: 0.2,
            a: 0.5,
        };
        rt.Clear(Some(&color));

        let mut rect = RECT::default();
        let _ = GetClientRect(hwnd, &mut rect);

        rt.EndDraw(None, None).unwrap();
    }
}

fn main() {
    // Init D2D
    unsafe {
        let factory = D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)
            .expect("Failed to create D2D factory");
        D2D_FACTORY = Some(factory);

        let dwrite_factory = DWriteCreateFactory::<IDWriteFactory>(DWRITE_FACTORY_TYPE_SHARED).expect("Failed to create DWrite factory");
        DWRITE_FACTORY = Some(dwrite_factory);
    }

    // Create Window class
    let class_name = w!("LightFenceWindow");
    let wnd_class = WNDCLASSW {
        lpfnWndProc: Some(wnd_proc),
        lpszClassName: class_name,
        ..Default::default()
    };

    unsafe {
        RegisterClassW(&wnd_class);
    }

    // Create Window
    let hwnd = unsafe{
        CreateWindowExW(
            WS_EX_LAYERED,
            class_name,
            w!("LightFence"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            100,
            100,
            800,
            600,
            None,
            None,
            None,
            None,
        )
        .expect("Failed to create window")
    };

    // set window transparent
    unsafe {
        let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), 200, LWA_ALPHA);
    }

    // message loop
    let mut msg = MSG::default();
    unsafe {
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
