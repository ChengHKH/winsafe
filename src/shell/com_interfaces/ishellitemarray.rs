#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;
use std::mem::ManuallyDrop;

use crate::kernel::ffi_types::{HRES, PCVOID, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::ole_IUnknown;
use crate::shell::decl::IShellItem;
use crate::vt::IUnknownVT;

/// [`IShellItemArray`](crate::IShellItemArray) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct IShellItemArrayVT {
	pub IUnknownVT: IUnknownVT,
	pub BindToHandler: fn(ComPtr, PVOID, PCVOID, PCVOID, *mut ComPtr) -> HRES,
	pub GetPropertyStore: fn(ComPtr, u32, PCVOID, *mut ComPtr) -> HRES,
	pub GetPropertyDescriptionList: fn(ComPtr, PVOID, PCVOID, *mut ComPtr) -> HRES,
	pub GetAttributes: fn(ComPtr, u32, u32, PVOID) -> HRES,
	pub GetCount: fn(ComPtr, *mut u32) -> HRES,
	pub GetItemAt: fn(ComPtr, u32, *mut ComPtr) -> HRES,
	pub EnumItems: fn(ComPtr, *mut PVOID) -> HRES,
}

com_interface! { IShellItemArray: "shell";
	"b63ea76d-1f85-456f-a19c-48159efa858b";
	/// [`IShellItemArray`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitemarray)
	/// COM interface over [`IShellItemArrayVT`](crate::vt::IShellItemArrayVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl shell_IShellItemArray for IShellItemArray {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IShellItemArray`](crate::IShellItemArray).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait shell_IShellItemArray: ole_IUnknown {
	/// Returns an iterator over the [`IShellItem`](crate::IShellItem) elements
	/// by calling
	/// [`IShellItemArray::GetCount`](crate::prelude::shell_IShellItemArray::GetCount)
	/// and
	/// [`IShellItemArray::GetItemAt`](crate::prelude::shell_IShellItemArray::GetItemAt)
	/// consecutively.
	///
	/// # Examples
	///
	/// Iterating over the [`IShellItem`](crate::IShellItem) objects:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, IShellItemArray};
	///
	/// let ish_arr: IShellItemArray; // initialized somewhere
	/// # let ish_arr = IShellItemArray::from(unsafe { winsafe::ComPtr::null() });
	///
	/// for ish_item in ish_arr.iter()? {
	///     let ish_item = ish_item?;
	///     println!("Path: {}",
	///         ish_item.GetDisplayName(co::SIGDN::FILESYSPATH)?);
	/// }
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	///
	/// Collecting the file paths into a
	/// [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html):
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HrResult, IShellItemArray};
	///
	/// let ish_arr: IShellItemArray; // initialized somewhere
	/// # let ish_arr = IShellItemArray::from(unsafe { winsafe::ComPtr::null() });
	///
	/// let paths = ish_arr.iter()?
	///     .map(|shi|
	///         shi.and_then(|shi|
	///             shi.GetDisplayName(co::SIGDN::FILESYSPATH),
	///         ),
	///     )
	///     .collect::<HrResult<Vec<_>>>()?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	#[must_use]
	fn iter<'a>(&'a self) -> HrResult<Box<dyn Iterator<Item = HrResult<IShellItem>> + 'a>> {
		Ok(Box::new(ShellItemIter::new(unsafe { self.ptr() })?))
	}

	/// [`IShellItemArray::GetCount`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemarray-getcount)
	/// method.
	#[must_use]
	fn GetCount(&self) -> HrResult<u32> {
		let mut count = u32::default();
		unsafe {
			let vt = self.vt_ref::<IShellItemArrayVT>();
			ok_to_hrresult((vt.GetCount)(self.ptr(), &mut count))
		}.map(|_| count)
	}

	/// [`IShellItemArray::GetItemAt`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemarray-getitemat)
	/// method.
	///
	/// Prefer using
	/// [`IShellItemArrayT::iter`](crate::prelude::shell_IShellItemArray::iter).
	#[must_use]
	fn GetItemAt(&self, index: u32) -> HrResult<IShellItem> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IShellItemArrayVT>();
			ok_to_hrresult((vt.GetItemAt)(self.ptr(), index, &mut ppv_queried))
				.map(|_| IShellItem::from(ppv_queried))
		}
	}
}

//------------------------------------------------------------------------------

struct ShellItemIter<'a> {
	array: ManuallyDrop<IShellItemArray>,
	count: u32,
	current: u32,
	_owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for ShellItemIter<'a> {
	type Item = HrResult<IShellItem>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		match self.array.GetItemAt(self.current) {
			Err(e) => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
			Ok(shell_item) => {
				self.current += 1;
				Some(Ok(shell_item))
			},
		}
	}
}

impl<'a> ShellItemIter<'a> {
	fn new(com_ptr: ComPtr) -> HrResult<Self> {
		let array = ManuallyDrop::new(IShellItemArray(com_ptr));
		let count = array.GetCount()?;

		Ok(Self {
			array,
			count,
			current: 0,
			_owner: PhantomData,
		})
	}
}
