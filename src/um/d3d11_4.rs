// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::BOOL;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
DEFINE_GUID!{IID_ID3D11Device4,
    0x8992ab71, 0x02e6, 0x4b8d, 0xba, 0x48, 0xb0, 0x56, 0xdc, 0xda, 0x42, 0xc4}
RIDL!{#[uuid(0x9b7e4e00, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0)]
interface ID3D11Multithread(ID3D11MultithreadVtbl): IUnknown(IUnknownVtbl) {
    fn Enter() -> (),
    fn Leave() -> (),
    fn SetMultithreadProtected( 
        bMTProtect: BOOL,
    ) -> BOOL,
    fn GetMultithreadProtected() -> BOOL,
}}
