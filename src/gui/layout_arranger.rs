use std::cell::UnsafeCell;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::kernel::decl::SysResult;
use crate::msg::wm;
use crate::prelude::{Handle, user_Hdwp, user_Hwnd};
use crate::user::decl::{HDWP, HWND, HwndPlace, POINT, RECT, SIZE};

/// Specifies the horizontal behavior of the control when the parent window is
/// resized.
///
/// The values are analog to [`gui::Vert`](crate::gui::Vert).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Horz {
	/// Nothing will be done when parent window is resized.
	None,
	/// When parent window resizes, the control will move anchored at right.
	/// Size of the control will remain fixed.
	Repos,
	/// When parent window resizes, the control width will stretch/shrink
	/// accordingly. Position will remain fixed.
	Resize,
}

/// Specifies the vertical behavior of the control when the parent window is
/// resized.
///
/// The values are analog to [`gui::Horz`](crate::gui::Horz).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vert {
	/// Nothing will be done when parent window is resized.
	None,
	/// When parent window resizes, the control will move anchored at bottom.
	/// Size of the control will remain fixed.
	Repos,
	/// When parent window resizes, the control height will stretch/shrink
	/// accordingly. Position will remain fixed.
	Resize,
}

struct ChildInfo {
	hchild: HWND,
	rc_orig: RECT, // original coordinates relative to parent
	horz: Horz,
	vert: Vert,
}

struct Obj { // actual fields of LayoutArranger
	ctrls: UnsafeCell<Vec<ChildInfo>>,
	sz_parent_orig: UnsafeCell<SIZE>, // original parent client area
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Rearranges the stored controls according to their predefined rules.
#[derive(Clone)]
pub(in crate::gui) struct LayoutArranger(Pin<Arc<Obj>>);

impl LayoutArranger {
	pub(in crate::gui) fn new() -> Self {
		Self(
			Arc::pin(
				Obj {
					ctrls: UnsafeCell::new(Vec::with_capacity(10)), // arbitrary
					sz_parent_orig: UnsafeCell::new(SIZE::default()),
					_pin: PhantomPinned,
				},
			),
		)
	}

	/// Adds a new child control to the internal list, so this control will have
	/// its position and size rearranged when requested.
	pub(in crate::gui) fn add_child(&self,
		hparent: &HWND, hchild: &HWND, horz: Horz, vert: Vert) -> SysResult<()>
	{
		if *hparent == HWND::NULL || *hchild == HWND::NULL {
			panic!("Cannot add resizer entries before window/control creation.");
		}

		if horz == Horz::None && vert == Vert::None {
			return Ok(()); // nothing to do, don't even add it
		}

		let ctrls = unsafe { &mut *self.0.ctrls.get() };
		if ctrls.is_empty() { // first control being added?
			let rc_parent = hparent.GetClientRect()?;
			*unsafe { &mut *self.0.sz_parent_orig.get() } =
				SIZE::new(rc_parent.right, rc_parent.bottom); // save original parent size
		}

		let mut rc_orig = hchild.GetWindowRect()?;
		hparent.ScreenToClientRc(&mut rc_orig)?; // control client coordinates relative to parent

		ctrls.push(
			ChildInfo {
				hchild: unsafe { hchild.raw_copy() },
				rc_orig,
				horz,
				vert,
			},
		);
		Ok(())
	}

	/// Rearranges all child controls to fit the new width/height of parent
	/// window.
	pub(in crate::gui) fn rearrange(&self, p: &wm::Size) -> SysResult<()> {
		let ctrls = unsafe { &mut *self.0.ctrls.get() };
		if ctrls.is_empty() // no controls
			|| p.request == co::SIZE_R::MINIMIZED { // we're minimized
			return Ok(());
		}

		let mut hdwp = HDWP::BeginDeferWindowPos(ctrls.len() as _)?;

		for ctrl in ctrls.iter() {
			let mut uflags = co::SWP::NOZORDER;
			if ctrl.horz == Horz::Repos && ctrl.vert == Vert::Repos { // reposition both vert & horz
				uflags |= co::SWP::NOSIZE;
			} else if ctrl.horz == Horz::Resize && ctrl.vert == Vert::Resize { // resize both vert & horz
				uflags |= co::SWP::NOMOVE;
			}

			let sz_parent_orig = unsafe { &mut *self.0.sz_parent_orig.get() };

			hdwp.DeferWindowPos(
				&ctrl.hchild,
				HwndPlace::None,
				POINT::new(
					match ctrl.horz {
						Horz::Repos => p.client_area.cx - sz_parent_orig.cx + ctrl.rc_orig.left,
						_ => ctrl.rc_orig.left // keep original x pos
					},
					match ctrl.vert {
						Vert::Repos => p.client_area.cy - sz_parent_orig.cy + ctrl.rc_orig.top,
						_ => ctrl.rc_orig.top // keep original y pos
					},
				),
				SIZE::new(
					match ctrl.horz {
						Horz::Resize => p.client_area.cx - sz_parent_orig.cx + ctrl.rc_orig.right - ctrl.rc_orig.left,
						_ => ctrl.rc_orig.right - ctrl.rc_orig.left // keep original width
					},
					match ctrl.vert {
						Vert::Resize => p.client_area.cy - sz_parent_orig.cy + ctrl.rc_orig.bottom - ctrl.rc_orig.top,
						_ =>ctrl.rc_orig.bottom - ctrl.rc_orig.top // keep original height
					},
				),
				uflags,
			)?;
		}

		Ok(())
	}
}
