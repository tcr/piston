
//! An implementation of Window that runs without a window at all.
//!
//! It saves just enough information to implement the window interface,
//! but otherwise does nothing.
//!
//! Often used in servers as an event loop.

extern crate input;

use {
	Window,
	WindowSettings,
	BuildFromWindowSettings,
	AdvancedWindow,
	Size
};

use self::input::Input;

/// The tiny NoWindow structure.
///
/// This structure holds just enough state to be able to return
/// values that were set.
pub struct NoWindow {
    should_close: bool,
    title: String,
    size: Size
}

impl NoWindow {
	/// Creates a new `NoWindow`.
    pub fn new(settings: WindowSettings) -> NoWindow {
        NoWindow {
            should_close: false,
            title: settings.title,
            size: settings.size,
        }
    }
}

impl Window for NoWindow {
    type Event = Input;
    
    fn should_close(&self) -> bool { self.should_close }
    
    fn set_should_close(&mut self, value: bool) { self.should_close = value; }
    
    fn size(&self) -> Size { self.size }
    
    fn swap_buffers(&mut self) {}
    
    fn poll_event(&mut self) -> Option<Input> { None }
    
    fn draw_size(&self) -> Size { self.size() }
}

impl BuildFromWindowSettings for NoWindow {
	/// # Errors
	///
	/// This function will always return without error.
    fn build_from_window_settings(settings: WindowSettings)
    -> Result<Self, String> {
        Ok(NoWindow::new(settings))
    }
}

impl AdvancedWindow for NoWindow {
    fn get_title(&self) -> String { self.title.clone() }
    
    fn set_title(&mut self, value: String) { self.title = value; }
    
    fn get_exit_on_esc(&self) -> bool { false }
    
    fn set_exit_on_esc(&mut self, _value: bool) {}
    
    fn set_capture_cursor(&mut self, _value: bool) {}
}