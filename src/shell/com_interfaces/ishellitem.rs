#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::decl::WString;
use crate::kernel::ffi_types::{HRES, PCVOID, PSTR, PVOID};
use crate::ole::decl::{ComPtr, CoTaskMemFree, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{ole_IBindCtx, ole_IUnknown};
use crate::vt::IUnknownVT;

/// [`IShellItem`](crate::IShellItem) virtual table.
#[repr(C)]
pub struct IShellItemVT {
	pub IUnknownVT: IUnknownVT,
	pub BindToHandler: fn(ComPtr, PVOID, PCVOID, PCVOID, *mut ComPtr) -> HRES,
	pub GetParent: fn(ComPtr, *mut ComPtr) -> HRES,
	pub GetDisplayName: fn(ComPtr, u32, *mut PSTR) -> HRES,
	pub GetAttributes: fn(ComPtr, u32, *mut u32) -> HRES,
	pub Compare: fn(ComPtr, PVOID, u32, *mut i32) -> HRES,
}

com_interface! { IShellItem: "43826d1e-e718-42ee-bc55-a1e261c37bfe";
	/// [`IShellItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitem)
	/// COM interface over [`IShellItemVT`](crate::vt::IShellItemVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with
	/// [`SHCreateItemFromParsingName`](crate::SHCreateItemFromParsingName)
	/// function.
}

impl shell_IShellItem for IShellItem {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IShellItem`](crate::IShellItem).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IShellItem: ole_IUnknown {
	/// [`IShellItem::BindToHandler`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-bindtohandler)
	/// method.
	///
	/// # Examples
	///
	/// Retrieving the items inside a directory:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, IBindCtx, IEnumShellItems, IShellItem};
	///
	/// let sh_folder: IShellItem; // initialized somewhere
	///
	/// # let sh_folder = IShellItem::from(unsafe { winsafe::ComPtr::null() });
	/// let sh_items = sh_folder.BindToHandler::<IEnumShellItems>(
	///     None::<&IBindCtx>,
	///     &co::BHID::EnumItems,
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	#[must_use]
	fn BindToHandler<T>(&self,
		bind_ctx: Option<&impl ole_IBindCtx>,
		bhid: &co::BHID,
	) -> HrResult<T>
		where T: ole_IUnknown,
	{
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IShellItemVT>();
			ok_to_hrresult(
				(vt.BindToHandler)(
					self.ptr(),
					bind_ctx.map_or(std::ptr::null_mut(), |i| i.ptr().0 as _),
					bhid as *const _ as _,
					&T::IID as *const _ as _,
					&mut ppv_queried,
				),
			).map(|_| T::from(ppv_queried))
		}
	}

	/// [`IShellItem::GetAttributes`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getattributes)
	/// method.
	#[must_use]
	fn GetAttributes(&self, sfgao_mask: co::SFGAO) -> HrResult<co::SFGAO> {
		let mut attrs = u32::default();
		match co::HRESULT(
			unsafe {
				let vt = self.vt_ref::<IShellItemVT>();
				(vt.GetAttributes)(self.ptr(), sfgao_mask.0, &mut attrs)
			},
		) {
			co::HRESULT::S_OK
			| co::HRESULT::S_FALSE => Ok(co::SFGAO(attrs)),
			hr => Err(hr),
		}
	}

	/// [`IShellItem::GetDisplayName`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getdisplayname)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, IBindCtx, IShellItem, SHCreateItemFromParsingName};
	///
	/// let shi = SHCreateItemFromParsingName::<IShellItem>(
	///     "C:\\Temp\\foo.txt",
	///     None::<&IBindCtx>,
	/// )?;
	///
	/// let full_path = shi.GetDisplayName(co::SIGDN::FILESYSPATH)?;
	///
	/// println!("{}", full_path);
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	#[must_use]
	fn GetDisplayName(&self, sigdn_name: co::SIGDN) -> HrResult<String> {
		let mut pstr: *mut u16 = std::ptr::null_mut();
		unsafe {
			let vt = self.vt_ref::<IShellItemVT>();
			ok_to_hrresult(
				(vt.GetDisplayName)(self.ptr(), sigdn_name.0, &mut pstr),
			)
		}.map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr as _);
			name.to_string()
		})
	}

	/// [`IShellItem::GetParent`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getparent)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, IBindCtx, IShellItem, SHCreateItemFromParsingName};
	///
	/// let shi = SHCreateItemFromParsingName::<IShellItem>(
	///     "C:\\Temp\\foo.txt",
	///     None::<&IBindCtx>,
	/// )?;
	///
	/// let parent_shi = shi.GetParent()?;
	/// let full_path = parent_shi.GetDisplayName(co::SIGDN::FILESYSPATH)?;
	///
	/// println!("{}", full_path);
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	#[must_use]
	fn GetParent(&self) -> HrResult<IShellItem> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IShellItemVT>();
			ok_to_hrresult((vt.GetParent)(self.ptr(), &mut ppv_queried))
				.map(|_| IShellItem::from(ppv_queried))
		}
	}
}
