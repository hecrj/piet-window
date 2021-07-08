use piet_common::Piet;
use raw_window_handle::HasRawWindowHandle;

/// A double-buffered window surface.
pub struct Surface;

impl Surface {
    pub fn new<W: HasRawWindowHandle>(
        _window: &W,
        _width: usize,
        _height: usize,
    ) -> Surface {
        unimplemented! {}
    }

    pub fn resize(&mut self, _width: usize, _height: usize) {
        unimplemented! {}
    }

    pub fn frame(&mut self) -> Frame {
        unimplemented! {}
    }
}

pub struct Frame<'a> {
    lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a> Frame<'a> {
    pub fn renderer(&mut self) -> Piet {
        unimplemented! {}
    }
}
