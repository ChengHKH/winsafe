use std::cell::UnsafeCell;
use std::sync::Arc;

use crate::co;
use crate::gui::events::{ButtonEvents, MsgEvents};
use crate::gui::native_control_base::NativeControlBase;
use crate::gui::parent::Parent;
use crate::handles::HWND;
use crate::structs::{POINT, SIZE};

/// Native
/// [button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#push-buttons)
/// control.
#[derive(Clone)]
pub struct Button {
	obj: Arc<UnsafeCell<Obj>>,
}

struct Obj { // actual fields of Button
	base: NativeControlBase,
	parent_events: ButtonEvents,
}

unsafe impl Send for Button {}
unsafe impl Sync for Button {}

cref_mref!(Button);

impl Button {
	/// Creates a new Button object.
	pub fn new<T: Parent>(parent: T) -> Button {
		Self::new_with_id(parent, NativeControlBase::auto_ctrl_id())
	}

	/// Creates a new Button object with a specific control ID.
	pub fn new_with_id<T: Parent>(parent: T, ctrl_id: u16) -> Button {
		Self {
			obj: Arc::new(UnsafeCell::new(
				Obj {
					base: NativeControlBase::new_with_id(ctrl_id, parent.hwnd()),
					parent_events: ButtonEvents::new(parent, ctrl_id),
				}
			)),
		}
	}

	/// Returns a reference to the underlying handle for this control.
	pub fn hwnd(&self) -> &HWND {
		&self.cref().base.hwnd()
	}

	/// Returns the control ID.
	pub fn ctrl_id(&self) -> u16 {
		self.cref().base.ctrl_id()
	}

	/// Exposes the button events.
	///
	/// # Panics
	///
	/// Panics if the parent window is already created.
	pub fn on(&self) -> &ButtonEvents {
		if self.cref().base.is_parent_created() {
			panic!("Cannot add events after the button parent was created.");
		}
		&self.cref().parent_events
	}

	/// Exposes the subclass events. If at least one event exists, the control
	/// will be
	/// [subclassed](https://docs.microsoft.com/en-us/windows/win32/controls/subclassing-overview).
	///
	/// # Panics
	///
	/// Panics if the control is already created. Closures must be attached to
	/// events before control creation.
	pub fn on_subclass(&self) -> &MsgEvents {
		self.cref().base.on_subclass()
	}

	/// Physically creates the control within the parent window.
	///
	/// # Panics
	///
	/// Panics if the control is already created.
	pub fn create(&self, opts: ButtonOpts) -> Result<(), co::ERROR> {
		if !self.cref().base.hwnd().is_null() {
			panic!("Cannot create Button twice.");
		}

		self.mref().base.create_window(
			"BUTTON", Some(&opts.text), opts.pos,
			SIZE{ cx: opts.width as i32, cy: opts.height as i32 },
			opts.ex_window_style,
			opts.window_style | opts.button_style.into(),
		).map(|_| ())
	}
}

//------------------------------------------------------------------------------

/// Options for [`Button::create`](crate::gui::Button::create).
pub struct ButtonOpts {
	/// Text of the button to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
	/// Button position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to 0x0.
	pub pos: POINT,
	/// Button width, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to 80.
	pub width: u32,
	/// Button height, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to 23.
	pub height: u32,
	/// Button styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `co::BS::PUSHBUTTON`.
	///
	/// Suggestion:
	/// * `co::BS::DEFPUSHBUTTON` for the default button of the window.
	/// * `co::BS::NOTIFY` to receive notifications other than the simple click.
	pub button_style: co::BS,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `co::WS_CHILD | co::WS_VISIBLE | co::WS_TABSTOP | co::WS_GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `co::WS_EX::LEFT`.
	pub ex_window_style: co::WS_EX,
}

impl Default for ButtonOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			pos: POINT { x: 0, y: 0 },
			width: 80,
			height: 23,
			button_style: co::BS::PUSHBUTTON,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			ex_window_style: co::WS_EX::LEFT,
		}
	}
}