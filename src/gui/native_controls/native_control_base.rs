use std::ptr::NonNull;

use crate::co;
use crate::enums::{AtomStr, IdMenu};
use crate::funcs_priv::WC_DIALOG;
use crate::gui::events::{MsgEvents, ProcessResult};
use crate::gui::traits::Parent;
use crate::handles::HWND;
use crate::msg::Wm;
use crate::structs::{POINT, SIZE};
use crate::WString;

static mut BASE_SUBCLASS_ID: usize = 0;

/// Base to all native child controls.
pub struct NativeControlBase<Ev> {
	hwnd: HWND,
	parent_events: Ev, // specific control events, which delegate to parent events
	subclass_events: MsgEvents, // for control subclassing
	ptr_parent_hwnd: NonNull<HWND>, // used only in control creation
}

impl<Ev> NativeControlBase<Ev> {
	pub fn new(parent: &dyn Parent, parent_events: Ev) -> NativeControlBase<Ev> {
		Self {
			hwnd: unsafe { HWND::null_handle() },
			parent_events,
			subclass_events: MsgEvents::new(),
			ptr_parent_hwnd: NonNull::from(parent.hwnd_ref()), // ref implicitly converted to pointer
		}
	}

	pub fn is_parent_created(&self) -> bool {
		let parent_hwnd = unsafe { self.ptr_parent_hwnd.as_ref() };
		!parent_hwnd.is_null()
	}

	pub fn hwnd(&self) -> &HWND {
		&self.hwnd
	}

	pub fn on(&self) -> &Ev {
		if !self.hwnd().is_null() {
			panic!("Cannot add events after the control is created.");
		} else if self.is_parent_created() {
			panic!("Cannot add events after the parent window is created.");
		}
		&self.parent_events
	}

	pub fn on_subclass(&self) -> &MsgEvents {
		if !self.hwnd.is_null() {
			panic!("Cannot add subclass events after the control is created.");
		} else if self.is_parent_created() {
			panic!("Cannot add subclass events after the parent window is created.");
		}
		&self.subclass_events
	}

	pub fn create_window(
		&mut self,
		class_name: &str,
		title: Option<&str>,
		pos: POINT, sz: SIZE,
		ctrl_id: u16,
		ex_styles: co::WS_EX,
		styles: co::WS) -> Result<HWND, co::ERROR>
	{
		if !self.hwnd.is_null() {
			panic!("Cannot create control twice.");
		} else if !self.is_parent_created() {
			panic!("Cannot create control before parent window is created.");
		}

		let parent_hwnd = unsafe { self.ptr_parent_hwnd.as_ref() };

		self.hwnd = HWND::CreateWindowEx(
			ex_styles,
			AtomStr::Str(WString::from_str(class_name)),
			title, styles,
			pos.x, pos.y, sz.cx, sz.cy,
			Some(*parent_hwnd),
			IdMenu::Id(ctrl_id),
			parent_hwnd.hinstance(),
			None,
		)?;

		self.install_subclass_if_needed()?;
		Ok(self.hwnd)
	}

	pub fn create_dlg(&mut self, ctrl_id: u16) -> Result<HWND, co::ERROR> {
		if !self.hwnd.is_null() {
			panic!("Cannot create control twice.");
		} else if !self.is_parent_created() {
			panic!("Cannot create control before parent window is created.");
		}

		let parent_hwnd = unsafe { self.ptr_parent_hwnd.as_ref() };

		let parent_atom = parent_hwnd.GetClassLongPtr(co::GCLP::ATOM);
		if parent_atom as u16 != WC_DIALOG { // https://stackoverflow.com/a/64437627/6923555
			panic!("Parent window is not a dialog, cannot create control.");
		}

		self.hwnd = parent_hwnd.GetDlgItem(ctrl_id as i32)?;
		self.install_subclass_if_needed()?;
		Ok(self.hwnd)
	}

	fn install_subclass_if_needed(&self) -> Result<(), co::ERROR> {
		if !self.subclass_events.is_empty() {
			let subclass_id = unsafe {
				BASE_SUBCLASS_ID += 1;
				BASE_SUBCLASS_ID
			};

			self.hwnd.SetWindowSubclass(
				Self::subclass_proc, subclass_id,
				self as *const Self as usize, // pass pointer to self
			)
		} else {
			Ok(())
		}
	}

	extern "system" fn subclass_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize,
		subclass_id: usize, ref_data: usize) -> isize
	{
		let ptr_self = ref_data as *mut Self; // retrieve
		let wm_any = Wm { msg_id: msg, wparam, lparam };
		let mut maybe_processed = ProcessResult::NotHandled;

		if !ptr_self.is_null() {
			let ref_self = unsafe { &mut *ptr_self };
			if !ref_self.hwnd.is_null() {
				maybe_processed = ref_self.subclass_events.process_message(wm_any);
			}
		}

		if msg == co::WM::NCDESTROY { // always check
			hwnd.RemoveWindowSubclass(Self::subclass_proc, subclass_id).ok();
		}

		match maybe_processed {
			ProcessResult::HandledWithRet(res) => res.into(),
			ProcessResult::HandledWithoutRet => 0,
			ProcessResult::NotHandled => hwnd.DefSubclassProc(wm_any).into(),
		}
	}
}