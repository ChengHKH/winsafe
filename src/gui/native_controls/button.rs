use std::cell::UnsafeCell;
use std::sync::Arc;

use crate::co;
use crate::gui::events::{ButtonEvents, MsgEvents};
use crate::gui::globals::{auto_ctrl_id, ui_font};
use crate::gui::native_controls::native_control_base::NativeControlBase;
use crate::gui::native_controls::opts_id::OptsId;
use crate::gui::traits::{Child, Parent};
use crate::handles::HWND;
use crate::msg::{WmCommand, WmSetFont};
use crate::structs::{POINT, SIZE};

/// Native
/// [button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#push-buttons)
/// control.
#[derive(Clone)]
pub struct Button {
	obj: Arc<UnsafeCell<Obj>>,
}

struct Obj { // actual fields of Button
	base: NativeControlBase<ButtonEvents>,
	opts_id: OptsId<ButtonOpts>,
}

unsafe impl Send for Button {}
unsafe impl Sync for Button {}

cref_mref!(Button);

impl Child for Button {
	fn create(&self) -> Result<(), co::ERROR> {
		match &self.cref().opts_id {
			OptsId::Wnd(opts) => {
				let our_hwnd = self.mref().base.create_window( // may panic
					"BUTTON", Some(&opts.text), opts.pos,
					SIZE{ cx: opts.width as i32, cy: opts.height as i32 },
					opts.ctrl_id,
					opts.ex_window_style,
					opts.window_style | opts.button_style.into(),
				)?;

				our_hwnd.SendMessage(WmSetFont{ hfont: ui_font(), redraw: true });
				Ok(())
			},
			OptsId::Dlg(ctrl_id) => {
				self.mref().base.create_dlg(*ctrl_id) // may panic
					.map(|_| ())
			},
		}
	}
}

impl Button {
	/// Instantiates a new `Button` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: ButtonOpts) -> Button {
		let opts = ButtonOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		let opts = ButtonOpts::define_ctrl_id(opts);

		Self {
			obj: Arc::new(UnsafeCell::new(
				Obj {
					base: NativeControlBase::new(parent, ButtonEvents::new(parent, ctrl_id)),
					opts_id: OptsId::Wnd(opts),
				},
			)),
		}
	}

	/// Instantiates a new `Button` object, to be loaded from a dialog resource
	/// with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> Button {
		Self {
			obj: Arc::new(UnsafeCell::new(
				Obj {
					base: NativeControlBase::new(parent, ButtonEvents::new(parent, ctrl_id)),
					opts_id: OptsId::Dlg(ctrl_id),
				},
			)),
		}
	}

	/// Returns the underlying handle for this control.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is created.
	pub fn hwnd(&self) -> HWND {
		*self.cref().base.hwnd()
	}

	/// Returns the control ID.
	pub fn ctrl_id(&self) -> u16 {
		match &self.cref().opts_id {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}

	/// Exposes the button events.
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	///
	/// # Examples
	///
	/// When button is clicked, becomes disabled:
	///
	/// ```rust,ignore
	/// use winsafe::gui::Button;
	///
	/// let btn: Button; // initialize it somewhere...
	///
	/// btn.on().bn_clicked({
	///   let btn = btn.clone(); // pass into closure
	///   move || {
	///     btn.EnableWindow(false);
	///   }
	/// });
	/// ```
	pub fn on(&self) -> &ButtonEvents {
		&self.cref().base.on()
	}

	/// Exposes the subclass events. If at least one event exists, the control
	/// will be
	/// [subclassed](https://docs.microsoft.com/en-us/windows/win32/controls/subclassing-overview).
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	pub fn on_subclass(&self) -> &MsgEvents {
		self.cref().base.on_subclass()
	}

	/// Fires the click event for the button.
	pub fn trigger_click(&self) {
		self.hwnd().SendMessage(
			WmCommand {
				code: co::CMD::BN_CLICKED,
				ctrl_id: self.ctrl_id(),
				ctrl_hwnd: Some(self.hwnd()),
			},
		);
	}
}

//------------------------------------------------------------------------------

/// Options for [`Button::new`](crate::gui::Button::new).
pub struct ButtonOpts {
	/// Text of the control to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
	/// Control position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to 0 x 0.
	pub pos: POINT,
	/// Control width, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to 80.
	pub width: u32,
	/// Control height, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to 23.
	pub height: u32,
	/// Button styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `BS::PUSHBUTTON`.
	///
	/// Suggestions:
	/// * replace with `BS::DEFPUSHBUTTON` for the default button of the window;
	/// * add `BS::NOTIFY` to receive notifications other than the simple click.
	pub button_style: co::BS,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	pub ex_window_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
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
			ctrl_id: 0,
		}
	}
}

impl ButtonOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}