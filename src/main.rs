#![windows_subsystem = "windows"]

use windows::{
    core::w,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Direct2D::{D2D1CreateFactory, D2D1_FACTORY_TYPE_SINGLE_THREADED, ID2D1Factory},
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW,
            PostQuitMessage, RegisterClassW, TranslateMessage, MSG, WNDCLASSW,
            WINDOW_EX_STYLE, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

static mut D2D_FACTORY: Option<ID2D1Factory> = None;

// window procedure
unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        0x0002 => {  // WM_DESTROY
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn main()
{
    // Init D2D
    unsafe {
        let factory = D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)
            .expect("Failed to create D2D factory");
        D2D_FACTORY = Some(factory);
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
            WINDOW_EX_STYLE::default(),
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
        .expect("Falied to create window")
    };

    // message loop
    let mut msg = MSG::default();
    unsafe {
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
