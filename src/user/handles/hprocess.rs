#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, user};
use crate::kernel::decl::{HPROCESS, SysResult};
use crate::kernel::privs::bool_to_sysresult;
use crate::prelude::Handle;

impl user_Hprocess for HPROCESS {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HPROCESS`](crate::HPROCESS).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hprocess: Handle {
	/// [`SetUserObjectInformation`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setuserobjectinformationw)
	/// method.
	///
	/// # Safety
	///
	/// The `pv_info` type varies according to `index`. If you set it wrong,
	/// you're likely to cause a buffer overrun.
	unsafe fn SetUserObjectInformation<T>(&self,
		index: co::UOI, pv_info: &mut T) -> SysResult<()>
	{
		bool_to_sysresult(
			user::ffi::SetUserObjectInformationW(
				self.as_ptr(),
				index.0,
				pv_info as *mut _ as _,
				std::mem::size_of::<T>() as _,
			),
		)
	}
}
