use piet_common::Piet;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use std::mem::ManuallyDrop;

/// A double-buffered window surface.
pub struct Surface {
    raw: cairo::Surface,
    buffer: cairo::Surface,
    width: usize,
    height: usize,
}

impl Surface {
    pub fn new<W: HasRawWindowHandle>(
        window: &W,
        width: usize,
        height: usize,
    ) -> Surface {
        let raw = match window.raw_window_handle() {
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            ))]
            RawWindowHandle::Xlib(xlib) => unsafe {
                assert!(!xlib.display.is_null());

                cairo::Surface::from_raw_full(
                    cairo_sys::cairo_xlib_surface_create(
                        xlib.display as *mut _,
                        xlib.window,
                        x11::xlib::XDefaultVisual(xlib.display as *mut _, 0),
                        width as i32,
                        height as i32,
                    ),
                )
            },
            #[cfg(target_os = "macos")]
            RawWindowHandle::MacOS(macos) => {
                unimplemented! {}
            }
            #[cfg(target_os = "windows")]
            RawWindowHandle::Windows(windows) => {
                unimplemented! {}
            }
            _ => {
                unimplemented! {}
            }
        };

        let buffer = raw.create_similar(
            cairo::Content::ColorAlpha,
            width as i32,
            height as i32,
        );

        Surface {
            raw,
            buffer,
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        unsafe {
            cairo_sys::cairo_xlib_surface_set_size(
                self.raw.to_raw_none(),
                width as i32,
                height as i32,
            );
        }

        self.buffer = self.raw.create_similar(
            cairo::Content::ColorAlpha,
            width as i32,
            height as i32,
        );

        self.width = width;
        self.height = height;
    }

    pub fn frame(&mut self) -> Frame {
        Frame {
            surface: &self.raw,
            context: ManuallyDrop::new(cairo::Context::new(&self.buffer)),
        }
    }
}

pub struct Frame<'a> {
    surface: &'a cairo::Surface,
    context: ManuallyDrop<cairo::Context>,
}

impl<'a> Frame<'a> {
    pub fn renderer(&mut self) -> Piet {
        Piet::new(&mut self.context)
    }
}

impl<'a> Drop for Frame<'a> {
    fn drop(&mut self) {
        let buffer = self.context.get_target();

        unsafe {
            ManuallyDrop::drop(&mut self.context);
        }

        buffer.flush();

        {
            let blit = cairo::Context::new(self.surface);
            blit.set_source_surface(&buffer, 0.0, 0.0);
            blit.paint();
        }

        self.surface.flush();

        unsafe {
            x11::xlib::XSync(
                cairo_sys::cairo_xlib_surface_get_display(
                    self.surface.to_raw_none(),
                ),
                0,
            );
        }
    }
}
