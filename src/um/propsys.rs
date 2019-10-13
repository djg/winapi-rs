// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::DWORD;
use shared::wtypes::{BSTR, PROPERTYKEY};
use um::propidl::{PROPVARIANT, REFPROPVARIANT};
use um::propkeydef::REFPROPERTYKEY;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCWSTR};
pub type IPropertyDescriptionList = IUnknown; // TODO
RIDL!{#[uuid(0x886d8eeb, 0x8cf2, 0x4446, 0x8d, 0x02, 0xcd, 0xba, 0x1d, 0xbd, 0xcf, 0x99)]
interface IPropertyStore(IPropertyStoreVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(
        cProps: *mut DWORD,
    ) -> HRESULT,
    fn GetAt(
        iProp: DWORD,
        pkey: *mut PROPERTYKEY,
    ) -> HRESULT,
    fn GetValue(
        key: REFPROPERTYKEY,
        pv: *mut PROPVARIANT,
    ) -> HRESULT,
    fn SetValue(
        key: REFPROPERTYKEY,
        propvar: REFPROPVARIANT,
    ) -> HRESULT,
    fn Commit() -> HRESULT,
}}
ENUM!{enum GETPROPERTYSTOREFLAGS {
    GPS_DEFAULT = 0,
    GPS_HANDLERPROPERTIESONLY = 0x1,
    GPS_READWRITE = 0x2,
    GPS_TEMPORARY = 0x4,
    GPS_FASTPROPERTIESONLY = 0x8,
    GPS_OPENSLOWITEM = 0x10,
    GPS_DELAYCREATION = 0x20,
    GPS_BESTEFFORT = 0x40,
    GPS_NO_OPLOCK = 0x80,
    GPS_PREFERQUERYPROPERTIES = 0x100,
    GPS_EXTRINSICPROPERTIES = 0x200,
    GPS_EXTRINSICPROPERTIESONLY = 0x400,
    GPS_MASK_VALID = 0x7ff,
}}
RIDL!{#[uuid(0x71604b0f, 0x97b0, 0x4764, 0x85, 0x77, 0x2f, 0x13, 0xe9, 0x8a, 0x14, 0x22)]
interface INamedPropertyStore(INamedPropertyStoreVtbl): IUnknown(IUnknownVtbl) {
    fn GetNamedValue(
        pszName: LPCWSTR,
        ppropvar: *mut PROPVARIANT,
    ) -> HRESULT,
    fn SetNamedValue(
        pszName: LPCWSTR,
        propvar: REFPROPVARIANT,
    ) -> HRESULT,
    fn GetNameCount(
        pdwCount: *mut DWORD,
    ) -> HRESULT,
    fn GetNameAt(
        iProp: DWORD,
        pbstrName: *mut BSTR,
    ) -> HRESULT,
}}
