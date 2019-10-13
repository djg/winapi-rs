// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of mfidl.h
use ctypes::{c_double, c_float, c_int, c_long, c_void};
use shared::basetsd::{DWORD64, UINT32, UINT64};
use shared::guiddef::{CLSID, GUID, REFCLSID, REFGUID, REFIID};
use shared::minwindef::{
    BOOL, BYTE, DWORD, FILETIME, FLOAT, LPVOID, MAX_PATH, UINT, ULONG, WORD 
};
use shared::windef::{HWND, RECT, SIZE};
use shared::wtypes::{PROPERTYKEY, VARIANT_BOOL};
use um::mfobjects::{
    IMFActivate, IMFAsyncCallback, IMFAsyncResult, IMFAttributes,
    IMFAttributesVtbl, IMFByteStream, IMFCollection, IMFDXGIDeviceManager,
    IMFMediaEvent, IMFMediaEventGenerator, IMFMediaEventGeneratorVtbl,
    IMFMediaType, IMFSample, MFARGB, MFT_REGISTER_TYPE_INFO, MF_ATTRIBUTE_TYPE,
    MF_STREAM_STATE, QWORD
};
use um::mftransform::IMFDeviceTransform;
use um::objidlbase::IStream;
use um::propidl::PROPVARIANT;
use um::propsys::{INamedPropertyStore, IPropertyStore};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{
    HRESULT, LONG, LONGLONG, LPCSTR, LPCWSTR, LPWSTR, PVOID, ULONGLONG, WCHAR
};
ENUM!{enum MFSESSION_SETTOPOLOGY_FLAGS {
    MFSESSION_SETTOPOLOGY_IMMEDIATE = 0x1,
    MFSESSION_SETTOPOLOGY_NORESOLUTION = 0x2,
    MFSESSION_SETTOPOLOGY_CLEAR_CURRENT = 0x4,
}}
ENUM!{enum MFSESSION_GETFULLTOPOLOGY_FLAGS {
    MFSESSION_GETFULLTOPOLOGY_CURRENT = 0x1,
}}
ENUM!{enum MFPMPSESSION_CREATION_FLAGS {
    MFPMPSESSION_UNPROTECTED_PROCESS = 0x1,
    MFPMPSESSION_IN_PROCESS = 0x2,
}}
pub type TOPOID = u64;
DEFINE_GUID!{MF_WVC1_PROG_SINGLE_SLICE_CONTENT,
    0x67ec2559, 0x0f2f, 0x4420, 0xa4, 0xdd, 0x2f, 0x8e, 0xe7, 0xa5, 0x73, 0x8b}
DEFINE_GUID!{MF_PROGRESSIVE_CODING_CONTENT,
    0x8f020eea, 0x1508, 0x471f, 0x9d, 0xa6, 0x50, 0x7d, 0x7c, 0xfa, 0x40, 0xdb}
DEFINE_GUID!{MF_NALU_LENGTH_SET,
    0xa7911d53, 0x12a4, 0x4965, 0xae, 0x70, 0x6e, 0xad, 0xd6, 0xff, 0x05, 0x51}
DEFINE_GUID!{MF_NALU_LENGTH_INFORMATION,
    0x19124e7c, 0xad4b, 0x465f, 0xbb, 0x18, 0x20, 0x18, 0x62, 0x87, 0xb6, 0xaf}
DEFINE_GUID!{MF_USER_DATA_PAYLOAD,
    0xd1d4985d, 0xdc92, 0x457a, 0xb3, 0xa0, 0x65, 0x1a, 0x33, 0xa3, 0x10, 0x47}
DEFINE_GUID!{MF_MPEG4SINK_SPSPPS_PASSTHROUGH,
    0x5601a134, 0x2005, 0x4ad2, 0xb3, 0x7d, 0x22, 0xa6, 0xc5, 0x54, 0xde, 0xb2}
DEFINE_GUID!{MF_MPEG4SINK_MOOV_BEFORE_MDAT,
    0xf672e3ac, 0xe1e6, 0x4f10, 0xb5, 0xec, 0x5f, 0x3b, 0x30, 0x82, 0x88, 0x16}
DEFINE_GUID!{MF_MPEG4SINK_MINIMUM_PROPERTIES_SIZE,
    0xdca1ed52, 0x450e, 0x4a22, 0x8c, 0x62, 0x4e, 0xd4, 0x52, 0xf7, 0xa1, 0x87}
DEFINE_GUID!{MF_MPEG4SINK_MIN_FRAGMENT_DURATION,
    0xa30b570c, 0x8efd, 0x45e8, 0x94, 0xfe, 0x27, 0xc8, 0x4b, 0x5b, 0xdf, 0xf6}
DEFINE_GUID!{MF_MPEG4SINK_MAX_CODED_SEQUENCES_PER_FRAGMENT,
    0xfc1b3bd6, 0x692d, 0x4ce5, 0x92, 0x99, 0x73, 0x8a, 0xa5, 0x46, 0x3e, 0x9a}
RIDL!{#[uuid(0x90377834, 0x21D0, 0x4dee, 0x82, 0x14, 0xBA, 0x2E, 0x3E, 0x6C, 0x11, 0x27)]
interface IMFMediaSession(IMFMediaSessionVtbl):
  IMFMediaEventGenerator(IMFMediaEventGeneratorVtbl) {
    fn SetTopology(
        dwSetTopologyFlags: DWORD,
        pTopology: *mut IMFTopology,
    ) -> HRESULT,
    fn ClearTopologies() -> HRESULT,
    fn Start(
        pguidTimeFormat: *const GUID,
        pvarStartPosition: *const PROPVARIANT,
    ) -> HRESULT,
    fn Pause() -> HRESULT,
    fn Stop() -> HRESULT,
    fn Close() -> HRESULT,
    fn Shutdown() -> HRESULT,
    fn GetClock(
        ppClock: *mut *mut IMFClock,
    ) -> HRESULT,
    fn GetSessionCapabilities(
        pdwCaps: *mut DWORD,
    ) -> HRESULT,
    fn GetFullTopology(
        dwGetFullTopologyFlags: DWORD,
        TopoId: TOPOID,
        ppFullTopology: *mut *mut IMFTopology,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_SESSION_TOPOLOADER,
    0x1e83d482, 0x1f1c, 0x4571, 0x84, 0x05, 0x88, 0xf4, 0xb2, 0x18, 0x1f, 0x71}
DEFINE_GUID!{MF_SESSION_GLOBAL_TIME,
    0x1e83d482, 0x1f1c, 0x4571, 0x84, 0x05, 0x88, 0xf4, 0xb2, 0x18, 0x1f, 0x72}
DEFINE_GUID!{MF_SESSION_QUALITY_MANAGER,
    0x1e83d482, 0x1f1c, 0x4571, 0x84, 0x05, 0x88, 0xf4, 0xb2, 0x18, 0x1f, 0x73}
DEFINE_GUID!{MF_SESSION_CONTENT_PROTECTION_MANAGER,
    0x1e83d482, 0x1f1c, 0x4571, 0x84, 0x05, 0x88, 0xf4, 0xb2, 0x18, 0x1f, 0x74}
DEFINE_GUID!{MF_SESSION_SERVER_CONTEXT,
    0xafe5b291, 0x50fa, 0x46e8, 0xb9, 0xbe, 0x0c, 0x0c, 0x3c, 0xe4, 0xb3, 0xa5}
DEFINE_GUID!{MF_SESSION_REMOTE_SOURCE_MODE,
    0xf4033ef4, 0x9bb3, 0x4378, 0x94, 0x1f, 0x85, 0xa0, 0x85, 0x6b, 0xc2, 0x44}
DEFINE_GUID!{MF_SESSION_APPROX_EVENT_OCCURRENCE_TIME,
    0x190e852f, 0x6238, 0x42d1, 0xb5, 0xaf, 0x69, 0xea, 0x33, 0x8e, 0xf8, 0x50}
DEFINE_GUID!{MF_PMP_SERVER_CONTEXT,
    0x2f00c910, 0xd2cf, 0x4278, 0x8b, 0x6a, 0xd0, 0x77, 0xfa, 0xc3, 0xa2, 0x5f}
extern "system" {
    pub fn MFCreateMediaSession(
        pConfiguration: *mut IMFAttributes,
        ppMediaSession: *mut *mut IMFMediaSession,
    ) -> HRESULT;
    pub fn MFCreatePMPMediaSession(
        dwCreationFlags: DWORD,
        pConfiguration: *mut IMFAttributes,
        ppMediaSession: *mut *mut IMFMediaSession,
        ppEnablerActivate: *mut *mut IMFActivate,
    ) -> HRESULT;
}
ENUM!{enum MF_OBJECT_TYPE {
    MF_OBJECT_MEDIASOURCE,
    MF_OBJECT_BYTESTREAM,
    MF_OBJECT_INVALID,
}}
pub const MF_RESOLUTION_MEDIASOURCE: DWORD = 0x1;
pub const MF_RESOLUTION_BYTESTREAM: DWORD = 0x2;
pub const MF_RESOLUTION_CONTENT_DOES_NOT_HAVE_TO_MATCH_EXTENSION_OR_MIME_TYPE: DWORD = 0x10;
pub const MF_RESOLUTION_KEEP_BYTE_STREAM_ALIVE_ON_FAIL: DWORD = 0x20;
pub const MF_RESOLUTION_DISABLE_LOCAL_PLUGINS: DWORD = 0x40;
pub const MF_RESOLUTION_PLUGIN_CONTROL_POLICY_APPROVED_ONLY: DWORD = 0x80;
pub const MF_RESOLUTION_PLUGIN_CONTROL_POLICY_WEB_ONLY: DWORD = 0x100;
pub const MF_RESOLUTION_PLUGIN_CONTROL_POLICY_WEB_ONLY_EDGEMODE: DWORD = 0x200;
pub const MF_RESOLUTION_ENABLE_STORE_PLUGINS: DWORD = 0x400;
pub const MF_RESOLUTION_READ: DWORD = 0x10000;
pub const MF_RESOLUTION_WRITE: DWORD = 0x20000;
ENUM!{enum MF_CONNECT_METHOD {
    MF_CONNECT_DIRECT = 0,
    MF_CONNECT_ALLOW_CONVERTER = 0x1,
    MF_CONNECT_ALLOW_DECODER = 0x3,
    MF_CONNECT_RESOLVE_INDEPENDENT_OUTPUTTYPES = 0x4,
    MF_CONNECT_AS_OPTIONAL = 0x1_0000,
    MF_CONNECT_AS_OPTIONAL_BRANCH = 0x2_0000,
}}
ENUM!{enum MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS {
    MF_TOPOLOGY_RESOLUTION_SUCCEEDED = 0,
    MF_OPTIONAL_NODE_REJECTED_MEDIA_TYPE = 0x1,
    MF_OPTIONAL_NODE_REJECTED_PROTECTED_PROCESS = 0x2,
}}
RIDL!{#[uuid(0xFBE5A32D, 0xA497, 0x4b61, 0xBB, 0x85, 0x97, 0xB1, 0xA8, 0x48, 0xA6, 0xE3)]
interface IMFSourceResolver(IMFSourceResolverVtbl): IUnknown(IUnknownVtbl) {
    fn CreateObjectFromURL(
        pwszURL: LPCWSTR,
        dwFlags: DWORD,
        pProps: *mut IPropertyStore,
        pObjectType: *mut MF_OBJECT_TYPE,
        ppObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn CreateObjectFromByteStream(
        pByteStream: *mut IMFByteStream,
        pwszURL: LPCWSTR,
        dwFlags: DWORD,
        pProps: *mut IPropertyStore,
        pObjectType: *mut MF_OBJECT_TYPE,
        ppObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn BeginCreateObjectFromURL(
        pwszURL: LPCWSTR,
        dwFlags: DWORD,
        pProps: *mut IPropertyStore,
        ppIUnknownCancelCookie: *mut *mut IUnknown,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndCreateObjectFromURL(
        pResult: *mut IMFAsyncResult,
        pObjectType: *mut MF_OBJECT_TYPE,
        ppObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn BeginCreateObjectFromByteStream(
        pByteStream: *mut IMFByteStream,
        pwszURL: LPCWSTR,
        dwFlags: DWORD,
        pProps: *mut IPropertyStore,
        ppIUnknownCancelCookie: *mut *mut IUnknown,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndCreateObjectFromByteStream(
        pResult: *mut IMFAsyncResult,
        pObjectType: *mut MF_OBJECT_TYPE,
        ppObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn CancelObjectCreation(
        pIUnknownCancelCookie: *mut IUnknown,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateSourceResolver(
        ppISourceResolver: *mut *mut IMFSourceResolver,
    ) -> HRESULT;
    pub fn CreatePropertyStore(
        ppStore: *mut *mut IPropertyStore,
    ) -> HRESULT;
    pub fn MFGetSupportedSchemes(
        pPropVarSchemeArray: *mut PROPVARIANT,
    ) -> HRESULT;
    pub fn MFGetSupportedMimeTypes(
        pPropVarMimeTypeArray: *mut PROPVARIANT,
    ) -> HRESULT;
}
DEFINE_PROPERTYKEY!{MFPKEY_SourceOpenMonitor,
    0x074d4637, 0xb5ae, 0x465d, 0xaf, 0x17, 0x1a, 0x53, 0x8d, 0x28, 0x59, 0xdd, 2}
DEFINE_PROPERTYKEY!{MFPKEY_ASFMediaSource_ApproxSeek,
    0xb4cd270f, 0x244d, 0x4969, 0xbb, 0x92, 0x3f, 0x0f, 0xb8, 0x31, 0x6f, 0x10, 1}
DEFINE_PROPERTYKEY!{MFPKEY_ASFMediaSource_IterativeSeekIfNoIndex,
    0x170b65dc, 0x4a4e, 0x407a, 0xac, 0x22, 0x57, 0x7f, 0x50, 0xe4, 0xa3, 0x7c, 1}
DEFINE_PROPERTYKEY!{MFPKEY_ASFMediaSource_IterativeSeek_Max_Count,
    0x170b65dc, 0x4a4e, 0x407a, 0xac, 0x22, 0x57, 0x7f, 0x50, 0xe4, 0xa3, 0x7c, 2}
DEFINE_PROPERTYKEY!{MFPKEY_ASFMediaSource_IterativeSeek_Tolerance_In_MilliSecond,
    0x170b65dc, 0x4a4e, 0x407a, 0xac, 0x22, 0x57, 0x7f, 0x50, 0xe4, 0xa3, 0x7c, 3}
DEFINE_PROPERTYKEY!{MFPKEY_Content_DLNA_Profile_ID,
    0xcfa31b45, 0x525d, 0x4998, 0xbb, 0x44, 0x3f, 0x7d, 0x81, 0x54, 0x2f, 0xa4, 1}
DEFINE_PROPERTYKEY!{MFPKEY_MediaSource_DisableReadAhead,
    0x26366c14, 0xc5bf, 0x4c76, 0x88, 0x7b, 0x9f, 0x17, 0x54, 0xdb, 0x5f, 0x09, 1}
DEFINE_PROPERTYKEY!{MFPKEY_SBESourceMode,
    0x3fae10bb, 0xf859, 0x4192, 0xb5, 0x62, 0x18, 0x68, 0xd3, 0xda, 0x3a, 0x02, 1}
DEFINE_PROPERTYKEY!{MFPKEY_PMP_Creation_Callback,
    0x28bb4de2, 0x26a2, 0x4870, 0xb7, 0x20, 0xd2, 0x6b, 0xbe, 0xb1, 0x49, 0x42, 1}
DEFINE_PROPERTYKEY!{MFPKEY_HTTP_ByteStream_Enable_Urlmon,
    0xeda8afdf, 0xc171, 0x417f, 0x8d, 0x17, 0x2e, 0x09, 0x18, 0x30, 0x32, 0x92, 1}
DEFINE_PROPERTYKEY!{MFPKEY_HTTP_ByteStream_Urlmon_Bind_Flags,
    0xeda8afdf, 0xc171, 0x417f, 0x8d, 0x17, 0x2e, 0x09, 0x18, 0x30, 0x32, 0x92, 2}
DEFINE_PROPERTYKEY!{MFPKEY_HTTP_ByteStream_Urlmon_Security_Id,
    0xeda8afdf, 0xc171, 0x417f, 0x8d, 0x17, 0x2e, 0x09, 0x18, 0x30, 0x32, 0x92, 3}
DEFINE_PROPERTYKEY!{MFPKEY_HTTP_ByteStream_Urlmon_Window,
    0xeda8afdf, 0xc171, 0x417f, 0x8d, 0x17, 0x2e, 0x09, 0x18, 0x30, 0x32, 0x92, 4}
DEFINE_PROPERTYKEY!{MFPKEY_HTTP_ByteStream_Urlmon_Callback_QueryService,
    0xeda8afdf, 0xc171, 0x417f, 0x8d, 0x17, 0x2e, 0x09, 0x18, 0x30, 0x32, 0x92, 5}
DEFINE_PROPERTYKEY!{MFPKEY_MediaProtectionSystemId,
    0x636b271d, 0xddc7, 0x49e9, 0xa6, 0xc6, 0x47, 0x38, 0x59, 0x62, 0xe5, 0xbd, 1}
DEFINE_PROPERTYKEY!{MFPKEY_MediaProtectionSystemContext,
    0x636b271d, 0xddc7, 0x49e9, 0xa6, 0xc6, 0x47, 0x38, 0x59, 0x62, 0xe5, 0xbd, 2}
DEFINE_PROPERTYKEY!{MFPKEY_MediaProtectionSystemIdMapping,
    0x636b271d, 0xddc7, 0x49e9, 0xa6, 0xc6, 0x47, 0x38, 0x59, 0x62, 0xe5, 0xbd, 3}
DEFINE_PROPERTYKEY!{MFPKEY_MediaProtectionContainerGuid,
    0x42af3d7c, 0x00cf, 0x4a0f, 0x81, 0xf0, 0xad, 0xf5, 0x24, 0xa5, 0xa5, 0xb5, 1}
DEFINE_PROPERTYKEY!{MFPKEY_MediaProtectionSystemContextsPerTrack,
    0x4454b092, 0xd3da, 0x49b0, 0x84, 0x52, 0x68, 0x50, 0xc7, 0xdb, 0x76, 0x4d, 3}
DEFINE_PROPERTYKEY!{MFPKEY_HTTP_ByteStream_Download_Mode,
    0x817f11b7, 0xa982, 0x46ec, 0xa4, 0x49, 0xef, 0x58, 0xae, 0xd5, 0x3c, 0xa8, 1}
DEFINE_PROPERTYKEY!{MFPKEY_HTTP_ByteStream_Caching_Mode,
    0x86a2403e, 0xc78b, 0x44d7, 0x8b, 0xc8, 0xff, 0x72, 0x58, 0x11, 0x75, 0x08, 1}
DEFINE_PROPERTYKEY!{MFPKEY_HTTP_ByteStream_Cache_Limit,
    0x86a2403e, 0xc78b, 0x44d7, 0x8b, 0xc8, 0xff, 0x72, 0x58, 0x11, 0x75, 0x08, 2}
ENUM!{enum MFMEDIASOURCE_CHARACTERISTICS {
    MFMEDIASOURCE_IS_LIVE = 0x1,
    MFMEDIASOURCE_CAN_SEEK = 0x2,
    MFMEDIASOURCE_CAN_PAUSE = 0x4,
    MFMEDIASOURCE_HAS_SLOW_SEEK = 0x8,
    MFMEDIASOURCE_HAS_MULTIPLE_PRESENTATIONS = 0x10,
    MFMEDIASOURCE_CAN_SKIPFORWARD = 0x20,
    MFMEDIASOURCE_CAN_SKIPBACKWARD = 0x40,
    MFMEDIASOURCE_DOES_NOT_USE_NETWORK = 0x80,
}}
DEFINE_GUID!{MF_TIME_FORMAT_ENTRY_RELATIVE,
    0x4399f178, 0x46d3, 0x4504, 0xaf, 0xda, 0x20, 0xd3, 0x2e, 0x9b, 0xa3, 0x60 }
RIDL!{#[uuid(0x279a808d, 0xaec7, 0x40c8, 0x9c, 0x6b, 0xa6, 0xb4, 0x92, 0xc7, 0x8a, 0x66)]
interface IMFMediaSource(IMFMediaSourceVtbl): IMFMediaEventGenerator(IMFMediaEventGeneratorVtbl) {
    fn GetCharacteristics(
        pdwCharacteristics: *mut DWORD,
    ) -> HRESULT,
    fn CreatePresentationDescriptor(
        ppPresentationDescriptor: *mut *mut IMFPresentationDescriptor,
    ) -> HRESULT,
    fn Start(
        pPresentationDescriptor: *mut IMFPresentationDescriptor,
        pguidTimeFormat: *const GUID,
        pvarStartPosition: *const PROPVARIANT,
    ) -> HRESULT,
    fn Stop() -> HRESULT,
    fn Pause() -> HRESULT,
    fn Shutdown() -> HRESULT,
}}
RIDL!{#[uuid(0x3C9B2EB9, 0x86D5, 0x4514, 0xA3, 0x94, 0xF5, 0x66, 0x64, 0xF9, 0xF0, 0xD8)]
interface IMFMediaSourceEx(IMFMediaSourceExVtbl): IMFMediaSource(IMFMediaSourceVtbl) {
    fn GetSourceAttributes(
        ppAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT,
    fn GetStreamAttributes(
        dwStreamIdentifier: DWORD,
        ppAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT,
    fn SetD3DManager(
        pManager: *mut IUnknown,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_SOURCE_STREAM_SUPPORTS_HW_CONNECTION,
    0xa38253aa, 0x6314, 0x42fd, 0xa3, 0xce, 0xbb, 0x27, 0xb6, 0x85, 0x99, 0x46}
RIDL!{#[uuid(0x6ef2a662, 0x47c0, 0x4666, 0xb1, 0x3d, 0xcb, 0xb7, 0x17, 0xf2, 0xfa, 0x2c)]
interface IMFClockConsumer(IMFClockConsumerVtbl): IUnknown(IUnknownVtbl) {
    fn SetPresentationClock(
        pPresentationClock: *mut IMFPresentationClock,
    ) -> HRESULT,
    fn GetPresentationClock(
        ppPresentationClock: *mut *mut IMFPresentationClock,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xD182108F, 0x4EC6, 0x443f, 0xAA, 0x42, 0xA7, 0x11, 0x06, 0xEC, 0x82, 0x5F)]
interface IMFMediaStream(IMFMediaStreamVtbl): IMFMediaEventGenerator(IMFMediaEventGeneratorVtbl) {
    fn GetMediaSource(
        ppMediaSource: *mut *mut IMFMediaSource,
    ) -> HRESULT,
    fn GetStreamDescriptor(
        ppStreamDescriptor: *mut *mut IMFStreamDescriptor,
    ) -> HRESULT,
    fn RequestSample(
        pToken: *mut IUnknown,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_STREAM_SINK_SUPPORTS_HW_CONNECTION,
    0x9b465cbf, 0x0597, 0x4f9e, 0x9f, 0x3c, 0xb9, 0x7e, 0xee, 0xf9, 0x03, 0x59}
DEFINE_GUID!{MF_STREAM_SINK_SUPPORTS_ROTATION,
    0xb3e96280, 0xbd05, 0x41a5, 0x97, 0xad, 0x8a, 0x7f, 0xee, 0x24, 0xb9, 0x12}
pub const MEDIASINK_FIXED_STREAMS: DWORD = 0x00000001;
pub const MEDIASINK_CANNOT_MATCH_CLOCK: DWORD = 0x00000002;
pub const MEDIASINK_RATELESS: DWORD = 0x00000004;
pub const MEDIASINK_CLOCK_REQUIRED: DWORD = 0x00000008;
pub const MEDIASINK_CAN_PREROLL: DWORD = 0x00000010;
pub const MEDIASINK_REQUIRE_REFERENCE_MEDIATYPE: DWORD = 0x00000020;
ENUM!{enum MF_TRANSFER_VIDEO_FRAME_FLAGS {
    MF_TRANSFER_VIDEO_FRAME_DEFAULT,
    MF_TRANSFER_VIDEO_FRAME_STRETCH,
    MF_TRANSFER_VIDEO_FRAME_IGNORE_PAR,
}}
DEFINE_GUID!{MF_SINK_VIDEO_PTS,
     0x2162bde7, 0x421e, 0x4b90, 0x9b, 0x33, 0xe5, 0x8f, 0xbf, 0x1d, 0x58, 0xb6}
DEFINE_GUID!{MF_SINK_VIDEO_NATIVE_WIDTH,
     0xe6d6a707, 0x1505, 0x4747, 0x9b, 0x10, 0x72, 0xd2, 0xd1, 0x58, 0xcb, 0x3a}
DEFINE_GUID!{MF_SINK_VIDEO_NATIVE_HEIGHT,
     0xf0ca6705, 0x490c, 0x43e8, 0x94, 0x1c, 0xc0, 0xb3, 0x20, 0x6b, 0x9a, 0x65}
DEFINE_GUID!{MF_SINK_VIDEO_DISPLAY_ASPECT_RATIO_NUMERATOR,
     0xd0f33b22, 0xb78a, 0x4879, 0xb4, 0x55, 0xf0, 0x3e, 0xf3, 0xfa, 0x82, 0xcd}
DEFINE_GUID!{MF_SINK_VIDEO_DISPLAY_ASPECT_RATIO_DENOMINATOR,
     0x6ea1eb97, 0x1fe0, 0x4f10, 0xa6, 0xe4, 0x1f, 0x4f, 0x66, 0x15, 0x64, 0xe0}
DEFINE_GUID!{MF_BD_MVC_PLANE_OFFSET_METADATA,
     0x62a654e4, 0xb76c, 0x4901, 0x98, 0x23, 0x2c, 0xb6, 0x15, 0xd4, 0x73, 0x18}
DEFINE_GUID!{MF_LUMA_KEY_ENABLE,
    0x7369820f, 0x76de, 0x43ca, 0x92, 0x84, 0x47, 0xb8, 0xf3, 0x7e, 0x06, 0x49}
DEFINE_GUID!{MF_LUMA_KEY_LOWER,
     0x93d7b8d5, 0x0b81, 0x4715, 0xae, 0xa0, 0x87, 0x25, 0x87, 0x16, 0x21, 0xe9}
DEFINE_GUID!{MF_LUMA_KEY_UPPER,
     0xd09f39bb, 0x4602, 0x4c31, 0xa7, 0x06, 0xa1, 0x21, 0x71, 0xa5, 0x11, 0x0a}
DEFINE_GUID!{MF_USER_EXTENDED_ATTRIBUTES,
     0xc02abac6, 0xfeb2, 0x4541, 0x92, 0x2f, 0x92, 0x0b, 0x43, 0x70, 0x27, 0x22}
DEFINE_GUID!{MF_INDEPENDENT_STILL_IMAGE,
    0xea12af41, 0x0710, 0x42c9, 0xa1, 0x27, 0xda, 0xa3, 0xe7, 0x84, 0x83, 0xa5}
RIDL!{#[uuid(0x6ef2a660, 0x47c0, 0x4666, 0xb1, 0x3d, 0xcb, 0xb7, 0x17, 0xf2, 0xfa, 0x2c)]
interface IMFMediaSink(IMFMediaSinkVtbl): IUnknown(IUnknownVtbl) {
    fn GetCharacteristics(
        pdwCharacteristics: *mut DWORD,
    ) -> HRESULT,
    fn AddStreamSink(
        dwStreamSinkIdentifier: DWORD,
        pMediaType: *mut IMFMediaType,
        ppStreamSink: *mut *mut IMFStreamSink,
    ) -> HRESULT,
    fn RemoveStreamSink(
        dwStreamSinkIdentifier: DWORD,
    ) -> HRESULT,
    fn GetStreamSinkCount(
        pcStreamSinkCount: *mut DWORD,
    ) -> HRESULT,
    fn GetStreamSinkByIndex(
        dwIndex: DWORD,
        ppStreamSink: *mut *mut IMFStreamSink,
    ) -> HRESULT,
    fn GetStreamSinkById(
        dwStreamSinkIdentifier: DWORD,
        ppStreamSink: *mut *mut IMFStreamSink,
    ) -> HRESULT,
    fn SetPresentationClock(
        pPresentationClock: *mut IMFPresentationClock,
    ) -> HRESULT,
    fn GetPresentationClock(
        ppPresentationClock: *mut *mut IMFPresentationClock,
    ) -> HRESULT,
    fn Shutdown() -> HRESULT,
}}
ENUM!{enum MFSTREAMSINK_MARKER_TYPE {
    MFSTREAMSINK_MARKER_DEFAULT,
    MFSTREAMSINK_MARKER_ENDOFSEGMENT,
    MFSTREAMSINK_MARKER_TICK,
    MFSTREAMSINK_MARKER_EVENT,
}}
RIDL!{#[uuid(0x0A97B3CF, 0x8E7C, 0x4a3d, 0x8F, 0x8C, 0x0C, 0x84, 0x3D, 0xC2, 0x47, 0xFB)]
interface IMFStreamSink(IMFStreamSinkVtbl): IMFMediaEventGenerator(IMFMediaEventGeneratorVtbl) {
    fn GetMediaSink(
        ppMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT,
    fn GetIdentifier(
        pdwIdentifier: *mut DWORD,
    ) -> HRESULT,
    fn GetMediaTypeHandler(
        ppHandler: *mut *mut IMFMediaTypeHandler,
    ) -> HRESULT,
    fn ProcessSample(
        pSample: *mut IMFSample,
    ) -> HRESULT,
    fn PlaceMarker(
        eMarkerType: MFSTREAMSINK_MARKER_TYPE,
        pvarMarkerValue: *const PROPVARIANT,
        pvarContextValue: *const PROPVARIANT,
    ) -> HRESULT,
    fn Flush() -> HRESULT,
}}
RIDL!{#[uuid(0x86cbc910, 0xe533, 0x4751, 0x8e, 0x3b, 0xf1, 0x9b, 0x5b, 0x80, 0x6a, 0x03)]
interface IMFVideoSampleAllocator(IMFVideoSampleAllocatorVtbl):
  IUnknown(IUnknownVtbl) {
    fn SetDirectXManager(
        pManager: *mut IUnknown,
    ) -> HRESULT,
    fn UninitializeSampleAllocator() -> HRESULT,
    fn InitializeSampleAllocator(
        cRequestedFrames: DWORD,
        pMediaType: *mut IMFMediaType,
    ) -> HRESULT,
    fn AllocateSample(
        ppSample: *mut *mut IMFSample,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xA792CDBE, 0xC374, 0x4e89, 0x83, 0x35, 0x27, 0x8E, 0x7B, 0x99, 0x56, 0xA4)]
interface IMFVideoSampleAllocatorNotify(IMFVideoSampleAllocatorNotifyVtbl):
  IUnknown(IUnknownVtbl) {
    fn NotifyRelease() -> HRESULT,
}}
RIDL!{#[uuid(0x3978AA1A, 0x6D5B, 0x4B7F, 0xA3, 0x40, 0x90, 0x89, 0x91, 0x89, 0xAE, 0x34)]
interface IMFVideoSampleAllocatorNotifyEx(IMFVideoSampleAllocatorNotifyExVtbl):
  IMFVideoSampleAllocatorNotify(IMFVideoSampleAllocatorNotifyVtbl) {
    fn NotifyPrune(
        pSample: *mut IMFSample,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x992388B4, 0x3372, 0x4f67, 0x8B, 0x6F, 0xC8, 0x4C, 0x07, 0x1F, 0x47, 0x51)]
interface IMFVideoSampleAllocatorCallback(IMFVideoSampleAllocatorCallbackVtbl):
  IUnknown(IUnknownVtbl) {
    fn SetCallback(
        pNotify: *mut IMFVideoSampleAllocatorNotify,
    ) -> HRESULT,
    fn GetFreeSampleCount(
        plSamples: *mut LONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x545b3a48, 0x3283, 0x4f62, 0x86, 0x6f, 0xa6, 0x2d, 0x8f, 0x59, 0x8f, 0x9f)]
interface IMFVideoSampleAllocatorEx(IMFVideoSampleAllocatorExVtbl):
  IMFVideoSampleAllocator(IMFVideoSampleAllocatorVtbl) {
    fn InitializeSampleAllocatorEx(
        cInitialSamples: DWORD,
        cMaximumSamples: DWORD,
        pAttributes: *mut IMFAttributes,
        pMediaType: *mut IMFMediaType,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x20bc074b, 0x7a8d, 0x4609, 0x8c, 0x3b, 0x64, 0xa0, 0xa3, 0xb5, 0xd7, 0xce)]
interface IMFDXGIDeviceManagerSource(IMFDXGIDeviceManagerSourceVtbl):
  IUnknown(IUnknownVtbl) {
    fn GetManager(
        ppManager: *mut *mut IMFDXGIDeviceManager,
    ) -> HRESULT,
}}
ENUM!{enum MF_VIDEO_PROCESSOR_ROTATION {
        ROTATION_NONE,
        ROTATION_NORMAL,
}}
ENUM!{enum MF_VIDEO_PROCESSOR_MIRROR {
    MIRROR_NONE,
    MIRROR_HORIZONTAL,
    MIRROR_VERTICAL,
}}
RIDL!{#[uuid(0xA3F675D5, 0x6119, 0x4f7f, 0xA1, 0x00, 0x1D, 0x8B, 0x28, 0x0F, 0x0E, 0xFB)]
interface IMFVideoProcessorControl(IMFVideoProcessorControlVtbl):
  IUnknown(IUnknownVtbl) {
    fn SetBorderColor(
        pBorderColor: *mut MFARGB,
    ) -> HRESULT,
    fn SetSourceRectangle(
        pSrcRect: *mut RECT,
    ) -> HRESULT,
    fn SetDestinationRectangle(
        pDstRect: *mut RECT,
    ) -> HRESULT,
    fn SetMirror(
        eMirror: MF_VIDEO_PROCESSOR_MIRROR,
    ) -> HRESULT,
    fn SetRotation(
        eRotation: MF_VIDEO_PROCESSOR_ROTATION,
    ) -> HRESULT,
    fn SetConstrictionSize(
        pConstrictionSize: *mut SIZE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xBDE633D3, 0xE1DC, 0x4a7f, 0xA6, 0x93, 0xBB, 0xAE, 0x39, 0x9C, 0x4A, 0x20)]
interface IMFVideoProcessorControl2(IMFVideoProcessorControl2Vtbl):
  IMFVideoProcessorControl(IMFVideoProcessorControlVtbl) {
    fn SetRotationOverride(
        uiRotation: UINT,
    ) -> HRESULT,
    fn EnableHardwareEffects(
        fEnabled: BOOL,
    ) -> HRESULT,
    fn GetSupportedHardwareEffects(
        puiSupport: *mut UINT,
    ) -> HRESULT,
}}
ENUM!{enum MFVideoSphericalFormat {
    MFVideoSphericalFormat_Unsupported,
    MFVideoSphericalFormat_Equirectangular,
    MFVideoSphericalFormat_CubeMap,
    MFVideoSphericalFormat_3DMesh,
}}
DEFINE_GUID!{MF_XVP_SAMPLE_LOCK_TIMEOUT,
    0xaa4ddb29, 0x5134, 0x4363, 0xac, 0x72, 0x83, 0xec, 0x4b, 0xc1, 0x04, 0x26}
ENUM!{enum MFVideoSphericalProjectionMode {
    MFVideoSphericalProjectionMode_Spherical,
    MFVideoSphericalProjectionMode_Flat,
}}
RIDL!{#[uuid(0x2424B3F2, 0xEB23, 0x40f1, 0x91, 0xAA, 0x74, 0xBD, 0xDE, 0xEA, 0x08, 0x83)]
interface IMFVideoProcessorControl3(IMFVideoProcessorControl3Vtbl):
  IMFVideoProcessorControl2(IMFVideoProcessorControl2Vtbl) {
    fn GetNaturalOutputType(
        ppType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn EnableSphericalVideoProcessing(
        fEnable: BOOL,
        eFormat: MFVideoSphericalFormat,
        eProjectionMode: MFVideoSphericalProjectionMode,
    ) -> HRESULT,
    fn SetSphericalVideoProperties(
        X: c_float,
        Y: c_float,
        Z: c_float,
        W: c_float,
        fieldOfView: c_float,
    ) -> HRESULT,
    fn SetOutputDevice(
        pOutputDevice: *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x83CF873A, 0xF6DA, 0x4bc8, 0x82, 0x3F, 0xBA, 0xCF, 0xD5, 0x5D, 0xC4, 0x33)]
interface IMFTopology(IMFTopologyVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetTopologyID(
        pID: *mut TOPOID,
    ) -> HRESULT,
    fn AddNode(
        pNode: *mut IMFTopologyNode,
    ) -> HRESULT,
    fn RemoveNode(
        pNode: *mut IMFTopologyNode,
    ) -> HRESULT,
    fn GetNodeCount(
        pwNodes: *mut WORD,
    ) -> HRESULT,
    fn GetNode(
        wIndex: WORD,
        ppNode: *mut *mut IMFTopologyNode,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
    fn CloneFrom(
        pTopology: *mut IMFTopology,
    ) -> HRESULT,
    fn GetNodeByID(
        qwTopoNodeID: TOPOID,
        ppNode: *mut *mut IMFTopologyNode,
    ) -> HRESULT,
    fn GetSourceNodeCollection(
        ppCollection: *mut *mut IMFCollection,
    ) -> HRESULT,
    fn GetOutputNodeCollection(
        ppCollection: *mut *mut IMFCollection,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_TOPOLOGY_PROJECTSTART,
    0x7ed3f802, 0x86bb, 0x4b3f, 0xb7, 0xe4, 0x7c, 0xb4, 0x3a, 0xfd, 0x4b, 0x80}
DEFINE_GUID!{MF_TOPOLOGY_PROJECTSTOP,
    0x7ed3f803, 0x86bb, 0x4b3f, 0xb7, 0xe4, 0x7c, 0xb4, 0x3a, 0xfd, 0x4b, 0x80}
DEFINE_GUID!{MF_TOPOLOGY_NO_MARKIN_MARKOUT,
    0x7ed3f804, 0x86bb, 0x4b3f, 0xb7, 0xe4, 0x7c, 0xb4, 0x3a, 0xfd, 0x4b, 0x80}
ENUM!{enum MFTOPOLOGY_DXVA_MODE {
    MFTOPOLOGY_DXVA_DEFAULT,
    MFTOPOLOGY_DXVA_NONE,
    MFTOPOLOGY_DXVA_FULL,
}}
DEFINE_GUID!{MF_TOPOLOGY_DXVA_MODE,
    0x1e8d34f6, 0xf5ab, 0x4e23, 0xbb, 0x88, 0x87, 0x4a, 0xa3, 0xa1, 0xa7, 0x4d}
DEFINE_GUID!{MF_TOPOLOGY_ENABLE_XVP_FOR_PLAYBACK,
    0x1967731f, 0xcd78, 0x42fc, 0xb0, 0x26, 0x09, 0x92, 0xa5, 0x6e, 0x56, 0x93}
DEFINE_GUID!{MF_TOPOLOGY_STATIC_PLAYBACK_OPTIMIZATIONS,
    0xb86cac42, 0x41a6, 0x4b79, 0x89, 0x7a, 0x1a, 0xb0, 0xe5, 0x2b, 0x4a, 0x1b}
DEFINE_GUID!{MF_TOPOLOGY_PLAYBACK_MAX_DIMS,
    0x5715cf19, 0x5768, 0x44aa, 0xad, 0x6e, 0x87, 0x21, 0xf1, 0xb0, 0xf9, 0xbb}
ENUM!{enum MFTOPOLOGY_HARDWARE_MODE {
    MFTOPOLOGY_HWMODE_SOFTWARE_ONLY,
    MFTOPOLOGY_HWMODE_USE_HARDWARE,
    MFTOPOLOGY_HWMODE_USE_ONLY_HARDWARE,
}}
DEFINE_GUID!{MF_TOPOLOGY_HARDWARE_MODE,
    0xd2d362fd, 0x4e4f, 0x4191, 0xa5, 0x79, 0xc6, 0x18, 0xb6, 0x67, 0x06, 0xaf}
DEFINE_GUID!{MF_TOPOLOGY_PLAYBACK_FRAMERATE,
    0xc164737a, 0xc2b1, 0x4553, 0x83, 0xbb, 0x5a, 0x52, 0x60, 0x72, 0x44, 0x8f}
DEFINE_GUID!{MF_TOPOLOGY_DYNAMIC_CHANGE_NOT_ALLOWED,
    0xd529950b, 0xd484, 0x4527, 0xa9, 0xcd, 0xb1, 0x90, 0x95, 0x32, 0xb5, 0xb0}
DEFINE_GUID!{MF_TOPOLOGY_ENUMERATE_SOURCE_TYPES,
    0x6248c36d, 0x5d0b, 0x4f40, 0xa0, 0xbb, 0xb0, 0xb3, 0x05, 0xf7, 0x76, 0x98}
DEFINE_GUID!{MF_TOPOLOGY_START_TIME_ON_PRESENTATION_SWITCH,
    0xc8cc113f, 0x7951, 0x4548, 0xaa, 0xd6, 0x9e, 0xd6, 0x20, 0x2e, 0x62, 0xb3}
DEFINE_GUID!{MF_DISABLE_LOCALLY_REGISTERED_PLUGINS,
    0x66b16da9, 0xadd4, 0x47e0, 0xa1, 0x6b, 0x5a, 0xf1, 0xfb, 0x48, 0x36, 0x34}
DEFINE_GUID!{MF_LOCAL_PLUGIN_CONTROL_POLICY,
    0xd91b0085, 0xc86d, 0x4f81, 0x88, 0x22, 0x8c, 0x68, 0xe1, 0xd7, 0xfa, 0x04}
extern "system" {
    pub fn MFCreateTopology(
        ppTopo: *mut *mut IMFTopology,
    ) -> HRESULT;
}
ENUM!{enum MF_TOPOLOGY_TYPE {
    MF_TOPOLOGY_OUTPUT_NODE,
    MF_TOPOLOGY_SOURCESTREAM_NODE,
    MF_TOPOLOGY_TRANSFORM_NODE,
    MF_TOPOLOGY_TEE_NODE,
    MF_TOPOLOGY_MAX = -1i32 as u32,
}}
RIDL!{#[uuid(0x83CF873A, 0xF6DA, 0x4bc8, 0x82, 0x3F, 0xBA, 0xCF, 0xD5, 0x5D, 0xC4, 0x30)]
interface IMFTopologyNode(IMFTopologyNodeVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn SetObject(
        pObject: *mut IUnknown,
    ) -> HRESULT,
    fn GetObject(
        ppObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn GetNodeType(
        pType: *mut MF_TOPOLOGY_TYPE,
    ) -> HRESULT,
    fn GetTopoNodeID(
        pID: *mut TOPOID,
    ) -> HRESULT,
    fn SetTopoNodeID(
        ullTopoID: TOPOID,
    ) -> HRESULT,
    fn GetInputCount(
        pcInputs: *mut DWORD,
    ) -> HRESULT,
    fn GetOutputCount(
        pcOutputs: *mut DWORD,
    ) -> HRESULT,
    fn ConnectOutput(
        dwOutputIndex: DWORD,
        pDownstreamNode: *mut IMFTopologyNode,
        dwInputIndexOnDownstreamNode: DWORD,
    ) -> HRESULT,
    fn DisconnectOutput(
        dwOutputIndex: DWORD,
    ) -> HRESULT,
    fn GetInput(
        dwInputIndex: DWORD,
        ppUpstreamNode: *mut *mut IMFTopologyNode,
        pdwOutputIndexOnUpstreamNode: *mut DWORD,
    ) -> HRESULT,
    fn GetOutput(
        dwOutputIndex: DWORD,
        ppDownstreamNode: *mut *mut IMFTopologyNode,
        pdwInputIndexOnDownstreamNode: *mut DWORD,
    ) -> HRESULT,
    fn SetOutputPrefType(
        dwOutputIndex: DWORD,
        pType: *mut IMFMediaType,
    ) -> HRESULT,
    fn GetOutputPrefType(
        dwOutputIndex: DWORD,
        ppType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn SetInputPrefType(
        dwInputIndex: DWORD,
        pType: *mut IMFMediaType,
    ) -> HRESULT,
    fn GetInputPrefType(
        dwInputIndex: DWORD,
        ppType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn CloneFrom(
        pNode: *mut IMFTopologyNode,
    ) -> HRESULT,
}}
ENUM!{enum MF_TOPONODE_FLUSH_MODE {
    MF_TOPONODE_FLUSH_ALWAYS,
    MF_TOPONODE_FLUSH_SEEK,
    MF_TOPONODE_FLUSH_NEVER,
}}
DEFINE_GUID!{MF_TOPONODE_FLUSH,
    0x494bbce8, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
ENUM!{enum MF_TOPONODE_DRAIN_MODE {
    MF_TOPONODE_DRAIN_DEFAULT,
    MF_TOPONODE_DRAIN_ALWAYS,
    MF_TOPONODE_DRAIN_NEVER,
}}
DEFINE_GUID!{MF_TOPONODE_DRAIN,
    0x494bbce9, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPONODE_D3DAWARE,
    0x494bbced, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPOLOGY_RESOLUTION_STATUS,
    0x494bbcde, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPONODE_ERRORCODE,
    0x494bbcee, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPONODE_CONNECT_METHOD,
    0x494bbcf1, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPONODE_LOCKED,
    0x494bbcf7, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPONODE_WORKQUEUE_ID,
    0x494bbcf8, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPONODE_WORKQUEUE_MMCSS_CLASS,
    0x494bbcf9, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPONODE_DECRYPTOR,
    0x494bbcfa, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPONODE_DISCARDABLE,
    0x494bbcfb, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPONODE_ERROR_MAJORTYPE,
    0x494bbcfd, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPONODE_ERROR_SUBTYPE,
    0x494bbcfe, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPONODE_WORKQUEUE_MMCSS_TASKID,
    0x494bbcff, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPONODE_WORKQUEUE_MMCSS_PRIORITY,
    0x5001f840, 0x2816, 0x48f4, 0x93, 0x64, 0xad, 0x1e, 0xf6, 0x61, 0xa1, 0x23}
DEFINE_GUID!{MF_TOPONODE_WORKQUEUE_ITEM_PRIORITY,
    0xa1ff99be, 0x5e97, 0x4a53, 0xb4, 0x94, 0x56, 0x8c, 0x64, 0x2c, 0x0f, 0xf3}
DEFINE_GUID!{MF_TOPONODE_MARKIN_HERE,
    0x494bbd00, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPONODE_MARKOUT_HERE,
    0x494bbd01, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPONODE_DECODER,
    0x494bbd02, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID!{MF_TOPONODE_MEDIASTART,
    0x835c58ea, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6}
DEFINE_GUID!{MF_TOPONODE_MEDIASTOP,
    0x835c58eb, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6}
DEFINE_GUID!{MF_TOPONODE_SOURCE,
    0x835c58ec, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6}
DEFINE_GUID!{MF_TOPONODE_PRESENTATION_DESCRIPTOR,
    0x835c58ed, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6}
DEFINE_GUID!{MF_TOPONODE_STREAM_DESCRIPTOR,
    0x835c58ee, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6}
DEFINE_GUID!{MF_TOPONODE_SEQUENCE_ELEMENTID,
    0x835c58ef, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6}
DEFINE_GUID!{MF_TOPONODE_TRANSFORM_OBJECTID,
    0x88dcc0c9, 0x293e, 0x4e8b, 0x9a, 0xeb, 0x0a, 0xd6, 0x4c, 0xc0, 0x16, 0xb0}
DEFINE_GUID!{MF_TOPONODE_STREAMID,
    0x14932f9b, 0x9087, 0x4bb4, 0x84, 0x12, 0x51, 0x67, 0x14, 0x5c, 0xbe, 0x04}
DEFINE_GUID!{MF_TOPONODE_NOSHUTDOWN_ON_REMOVE,
    0x14932f9c, 0x9087, 0x4bb4, 0x84, 0x12, 0x51, 0x67, 0x14, 0x5c, 0xbe, 0x04}
DEFINE_GUID!{MF_TOPONODE_RATELESS,
    0x14932f9d, 0x9087, 0x4bb4, 0x84, 0x12, 0x51, 0x67, 0x14, 0x5c, 0xbe, 0x04}
DEFINE_GUID!{MF_TOPONODE_DISABLE_PREROLL,
    0x14932f9e, 0x9087, 0x4bb4, 0x84, 0x12, 0x51, 0x67, 0x14, 0x5c, 0xbe, 0x04}
DEFINE_GUID!{MF_TOPONODE_PRIMARYOUTPUT,
    0x6304ef99, 0x16b2, 0x4ebe, 0x9d, 0x67, 0xe4, 0xc5, 0x39, 0xb3, 0xa2, 0x59}
extern "system" {
    pub fn MFCreateTopologyNode(
        NodeType: MF_TOPOLOGY_TYPE,
        ppNode: *mut *mut IMFTopologyNode,
    ) -> HRESULT;
    pub fn MFGetTopoNodeCurrentType(
        pNode: *mut IMFTopologyNode,
        dwStreamIndex: DWORD,
        fOutput: BOOL,
        ppType: *mut *mut IMFMediaType,
    ) -> HRESULT;
}
RIDL!{#[uuid(0xfa993888, 0x4383, 0x415a, 0xa9, 0x30, 0xdd, 0x47, 0x2a, 0x8c, 0xf6, 0xf7)]
interface IMFGetService(IMFGetServiceVtbl): IUnknown(IUnknownVtbl) {
    fn GetService(
        guidService: REFGUID,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFGetService(
        punkObject: *mut IUnknown,
        guidService: REFGUID,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT;
}
pub type MFTIME = LONGLONG;
ENUM!{enum MFCLOCK_CHARACTERISTICS_FLAGS {
    MFCLOCK_CHARACTERISTICS_FLAG_FREQUENCY_10MHZ = 0x2,
    MFCLOCK_CHARACTERISTICS_FLAG_ALWAYS_RUNNING = 0x4,
    MFCLOCK_CHARACTERISTICS_FLAG_IS_SYSTEM_CLOCK = 0x8,
}}
ENUM!{enum MFCLOCK_STATE {
    MFCLOCK_STATE_INVALID,
    MFCLOCK_STATE_RUNNING,
    MFCLOCK_STATE_STOPPED,
    MFCLOCK_STATE_PAUSED,
}}
ENUM!{enum MFCLOCK_RELATIONAL_FLAGS {
    MFCLOCK_RELATIONAL_FLAG_JITTER_NEVER_AHEAD = 0x1,
}}
STRUCT!{struct MFCLOCK_PROPERTIES {
    qwCorrelationRate: u64,
    guidClockId: GUID,
    dwClockFlags: DWORD,
    qwClockFrequency: u64,
    dwClockTolerance: DWORD,
    dwClockJitter: DWORD,
}}
pub const MFCLOCK_FREQUENCY_HNS: DWORD = 10000000;
pub const MFCLOCK_TOLERANCE_UNKNOWN: DWORD = 50000;
pub const MFCLOCK_JITTER_ISR: DWORD = 1000;
pub const MFCLOCK_JITTER_DPC: DWORD = 4000;
pub const MFCLOCK_JITTER_PASSIVE: DWORD = 10000;
RIDL!{#[uuid(0x2eb1e945, 0x18b8, 0x4139, 0x9b, 0x1a, 0xd5, 0xd5, 0x84, 0x81, 0x85, 0x30)]
interface IMFClock(IMFClockVtbl): IUnknown(IUnknownVtbl) {
    fn GetClockCharacteristics(
        pdwCharacteristics: *mut DWORD,
    ) -> HRESULT,
    fn GetCorrelatedTime(
        dwReserved: DWORD,
        pllClockTime: *mut LONGLONG,
        phnsSystemTime: *mut MFTIME,
    ) -> HRESULT,
    fn GetContinuityKey(
        pdwContinuityKey: *mut DWORD,
    ) -> HRESULT,
    fn GetState(
        dwReserved: DWORD,
        peClockState: *mut MFCLOCK_STATE,
    ) -> HRESULT,
    fn GetProperties(
        pClockProperties: *mut MFCLOCK_PROPERTIES,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFGetSystemTime() -> MFTIME;
}
pub const PRESENTATION_CURRENT_POSITION: DWORD64 = 0x7fff_ffff_ffff_ffff;
RIDL!{#[uuid(0x868CE85C, 0x8EA9, 0x4f55, 0xAB, 0x82, 0xB0, 0x09, 0xA9, 0x10, 0xA8, 0x05)]
interface IMFPresentationClock(IMFPresentationClockVtbl): IMFClock(IMFClockVtbl) {
    fn SetTimeSource(
        pTimeSource: *mut IMFPresentationTimeSource,
    ) -> HRESULT,
    fn GetTimeSource(
        ppTimeSource: *mut *mut IMFPresentationTimeSource,
    ) -> HRESULT,
    fn GetTime(
        phnsClockTime: *mut MFTIME,
    ) -> HRESULT,
    fn AddClockStateSink(
        pStateSink: *mut IMFClockStateSink,
    ) -> HRESULT,
    fn RemoveClockStateSink(
        pStateSink: *mut IMFClockStateSink,
    ) -> HRESULT,
    fn Start(
        llClockStartOffset: LONGLONG,
    ) -> HRESULT,
    fn Stop() -> HRESULT,
    fn Pause() -> HRESULT,
}}
extern "system" {
    pub fn MFCreatePresentationClock(
        ppPresentationClock: *mut *mut IMFPresentationClock,
    ) -> HRESULT;
}
RIDL!{#[uuid(0x7FF12CCE, 0xF76F, 0x41c2, 0x86, 0x3B, 0x16, 0x66, 0xC8, 0xE5, 0xE1, 0x39)]
interface IMFPresentationTimeSource(IMFPresentationTimeSourceVtbl): IMFClock(IMFClockVtbl) {
    fn GetUnderlyingClock(
        ppClock: *mut *mut IMFClock,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateSystemTimeSource(
        ppSystemTimeSource: *mut *mut IMFPresentationTimeSource,
    ) -> HRESULT;
}
RIDL!{#[uuid(0xF6696E82, 0x74F7, 0x4f3d, 0xA1, 0x78, 0x8A, 0x5E, 0x09, 0xC3, 0x65, 0x9F)]
interface IMFClockStateSink(IMFClockStateSinkVtbl): IUnknown(IUnknownVtbl) {
    fn OnClockStart(
        hnsSystemTime: MFTIME,
        llClockStartOffset: LONGLONG,
    ) -> HRESULT,
    fn OnClockStop(
        hnsSystemTime: MFTIME,
    ) -> HRESULT,
    fn OnClockPause(
        hnsSystemTime: MFTIME,
    ) -> HRESULT,
    fn OnClockRestart(
        hnsSystemTime: MFTIME,
    ) -> HRESULT,
    fn OnClockSetRate(
        hnsSystemTime: MFTIME,
        flRate: c_float,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_PD_PMPHOST_CONTEXT,
    0x6c990d31, 0xbb8e, 0x477a, 0x85, 0x98, 0x0d, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID!{MF_PD_APP_CONTEXT,
    0x6c990d32, 0xbb8e, 0x477a, 0x85, 0x98, 0x0d, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID!{MF_PD_DURATION,
    0x6c990d33, 0xbb8e, 0x477a, 0x85, 0x98, 0x0d, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID!{MF_PD_TOTAL_FILE_SIZE,
    0x6c990d34, 0xbb8e, 0x477a, 0x85, 0x98, 0x0d, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID!{MF_PD_AUDIO_ENCODING_BITRATE,
    0x6c990d35, 0xbb8e, 0x477a, 0x85, 0x98, 0x0d, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID!{MF_PD_VIDEO_ENCODING_BITRATE,
    0x6c990d36, 0xbb8e, 0x477a, 0x85, 0x98, 0x0d, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID!{MF_PD_MIME_TYPE,
    0x6c990d37, 0xbb8e, 0x477a, 0x85, 0x98, 0x0d, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID!{MF_PD_LAST_MODIFIED_TIME,
    0x6c990d38, 0xbb8e, 0x477a, 0x85, 0x98, 0x0d, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID!{MF_PD_PLAYBACK_ELEMENT_ID,
    0x6c990d39, 0xbb8e, 0x477a, 0x85, 0x98, 0x0d, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID!{MF_PD_PREFERRED_LANGUAGE,
    0x6c990d3A, 0xbb8e, 0x477a, 0x85, 0x98, 0x0d, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID!{MF_PD_PLAYBACK_BOUNDARY_TIME,
    0x6c990d3b, 0xbb8e, 0x477a, 0x85, 0x98, 0x0d, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID!{MF_PD_AUDIO_ISVARIABLEBITRATE,
    0x33026ee0, 0xe387, 0x4582, 0xae, 0x0a, 0x34, 0xa2, 0xad, 0x3b, 0xaa, 0x18}
DEFINE_GUID!{MF_PD_ADAPTIVE_STREAMING,
    0xea0d5d97, 0x29f9, 0x488b, 0xae, 0x6b, 0x7d, 0x6b, 0x41, 0x36, 0x11, 0x2b}
RIDL!{#[uuid(0x03cb2711, 0x24d7, 0x4db6, 0xa1, 0x7f, 0xf3, 0xa7, 0xa4, 0x79, 0xa5, 0x36)]
interface IMFPresentationDescriptor(IMFPresentationDescriptorVtbl):
  IMFAttributes(IMFAttributesVtbl) {
    fn GetStreamDescriptorCount(
        pdwDescriptorCount: *mut DWORD,
    ) -> HRESULT,
    fn GetStreamDescriptorByIndex(
        dwIndex: DWORD,
        pfSelected: *mut BOOL,
        ppDescriptor: *mut *mut IMFStreamDescriptor,
    ) -> HRESULT,
    fn SelectStream(
        dwDescriptorIndex: DWORD,
    ) -> HRESULT,
    fn DeselectStream(
        dwDescriptorIndex: DWORD,
    ) -> HRESULT,
    fn Clone(
        ppPresentationDescriptor: *mut *mut IMFPresentationDescriptor,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreatePresentationDescriptor(
        cStreamDescriptors: DWORD,
        apStreamDescriptors: *mut *mut IMFStreamDescriptor,
        ppPresentationDescriptor: *mut *mut IMFPresentationDescriptor,
    ) -> HRESULT;
    pub fn MFRequireProtectedEnvironment(
        pPresentationDescriptor: *mut IMFPresentationDescriptor,
    ) -> HRESULT;
    pub fn MFSerializePresentationDescriptor(
        pPD: *mut IMFPresentationDescriptor,
        pcbData: *mut DWORD,
        ppbData: *mut *mut BYTE,
    ) -> HRESULT;
    pub fn MFDeserializePresentationDescriptor(
        cbData: DWORD,
        pbData: *mut BYTE,
        ppPD: *mut *mut IMFPresentationDescriptor,
    ) -> HRESULT;
}
DEFINE_GUID!{MF_SD_LANGUAGE,
    0x00af2180, 0xbdc2, 0x423c, 0xab, 0xca, 0xf5, 0x03, 0x59, 0x3b, 0xc1, 0x21}
DEFINE_GUID!{MF_SD_PROTECTED,
    0x00af2181, 0xbdc2, 0x423c, 0xab, 0xca, 0xf5, 0x03, 0x59, 0x3b, 0xc1, 0x21}
DEFINE_GUID!{MF_SD_STREAM_NAME,
    0x4f1b099d, 0xd314, 0x41e5, 0xa7, 0x81, 0x7f, 0xef, 0xaa, 0x4c, 0x50, 0x1f}
DEFINE_GUID!{MF_SD_MUTUALLY_EXCLUSIVE,
    0x023ef79c, 0x388d, 0x487f, 0xac, 0x17, 0x69, 0x6c, 0xd6, 0xe3, 0xc6, 0xf5}
RIDL!{#[uuid(0x56c03d9c, 0x9dbb, 0x45f5, 0xab, 0x4b, 0xd8, 0x0f, 0x47, 0xc0, 0x59, 0x38)]
interface IMFStreamDescriptor(IMFStreamDescriptorVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetStreamIdentifier(
        pdwStreamIdentifier: *mut DWORD,
    ) -> HRESULT,
    fn GetMediaTypeHandler(
        ppMediaTypeHandler: *mut *mut IMFMediaTypeHandler,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateStreamDescriptor(
        dwStreamIdentifier: DWORD,
        cMediaTypes: DWORD,
        apMediaTypes: *mut *mut IMFMediaType,
        ppDescriptor: *mut *mut IMFStreamDescriptor,
    ) -> HRESULT;
}
RIDL!{#[uuid(0xe93dcf6c, 0x4b07, 0x4e1e, 0x81, 0x23, 0xaa, 0x16, 0xed, 0x6e, 0xad, 0xf5)]
interface IMFMediaTypeHandler(IMFMediaTypeHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn IsMediaTypeSupported(
        pMediaType: *mut IMFMediaType,
        ppMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetMediaTypeCount(
        pdwTypeCount: *mut DWORD,
    ) -> HRESULT,
    fn GetMediaTypeByIndex(
        dwIndex: DWORD,
        ppType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn SetCurrentMediaType(
        pMediaType: *mut IMFMediaType,
    ) -> HRESULT,
    fn GetCurrentMediaType(
        ppMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetMajorType(
        pguidMajorType: *mut GUID,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateSimpleTypeHandler(
        ppHandler: *mut *mut IMFMediaTypeHandler,
    ) -> HRESULT;
}
ENUM!{enum MFTIMER_FLAGS {
    MFTIMER_RELATIVE = 0x1,
}}
RIDL!{#[uuid(0xe56e4cbd, 0x8f70, 0x49d8, 0xa0, 0xf8, 0xed, 0xb3, 0xd6, 0xab, 0x9b, 0xf2)]
interface IMFTimer(IMFTimerVtbl): IUnknown(IUnknownVtbl) {
    fn SetTimer(
        dwFlags: DWORD,
        llClockTime: LONGLONG,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
        ppunkKey: *mut *mut IUnknown,
    ) -> HRESULT,
    fn CancelTimer(
        punkKey: *mut IUnknown,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_ACTIVATE_CUSTOM_VIDEO_MIXER_CLSID,
    0xba491360, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8}
DEFINE_GUID!{MF_ACTIVATE_CUSTOM_VIDEO_MIXER_ACTIVATE,
    0xba491361, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8}
DEFINE_GUID!{MF_ACTIVATE_CUSTOM_VIDEO_MIXER_FLAGS,
    0xba491362, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8}
DEFINE_GUID!{MF_ACTIVATE_CUSTOM_VIDEO_PRESENTER_CLSID,
    0xba491364, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8}
DEFINE_GUID!{MF_ACTIVATE_CUSTOM_VIDEO_PRESENTER_ACTIVATE,
    0xba491365, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8}
DEFINE_GUID!{MF_ACTIVATE_CUSTOM_VIDEO_PRESENTER_FLAGS,
    0xba491366, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8}
pub const MF_ACTIVATE_CUSTOM_MIXER_ALLOWFAIL: DWORD = 0x1;
pub const MF_ACTIVATE_CUSTOM_PRESENTER_ALLOWFAIL: DWORD = 0x1;
DEFINE_GUID!{MF_ACTIVATE_MFT_LOCKED,
    0xc1f6093c, 0x7f65, 0x4fbd, 0x9e, 0x39, 0x5f, 0xae, 0xc3, 0xc4, 0xfb, 0xd7}
DEFINE_GUID!{MF_ACTIVATE_VIDEO_WINDOW,
    0x9a2dbbdd, 0xf57e, 0x4162, 0x82, 0xb9, 0x68, 0x31, 0x37, 0x76, 0x82, 0xd3}
ENUM!{enum MFSHUTDOWN_STATUS {
    MFSHUTDOWN_INITIATED,
    MFSHUTDOWN_COMPLETED,
}}
RIDL!{#[uuid(0x97ec2ea4, 0x0e42, 0x4937, 0x97, 0xac, 0x9d, 0x6d, 0x32, 0x88, 0x24, 0xe1)]
interface IMFShutdown(IMFShutdownVtbl): IUnknown(IUnknownVtbl) {
    fn Shutdown() -> HRESULT,
    fn GetShutdownStatus(
        pStatus: *mut MFSHUTDOWN_STATUS,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFShutdownObject(
        pUnk: *mut IUnknown,
    ) -> HRESULT;
    pub fn MFCreateAudioRenderer(
        pAudioAttributes: *mut IMFAttributes,
        ppSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreateAudioRendererActivate(
        ppActivate: *mut *mut IMFActivate,
    ) -> HRESULT;
}
DEFINE_GUID!{MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS,
    0xede4b5e0, 0xf805, 0x4d6c, 0x99, 0xb3, 0xdb, 0x01, 0xbf, 0x95, 0xdf, 0xab}
pub const MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS_CROSSPROCESS: DWORD = 0x1;
pub const MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS_NOPERSIST: DWORD = 0x2;
pub const MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS_DONT_ALLOW_FORMAT_CHANGES: DWORD = 0x4;
DEFINE_GUID!{MF_AUDIO_RENDERER_ATTRIBUTE_SESSION_ID,
    0xede4b5e3, 0xf805, 0x4d6c, 0x99, 0xb3, 0xdb, 0x01, 0xbf, 0x95, 0xdf, 0xab}
DEFINE_GUID!{MF_AUDIO_RENDERER_ATTRIBUTE_ENDPOINT_ID,
    0xb10aaec3, 0xef71, 0x4cc3, 0xb8, 0x73, 0x05, 0xa9, 0xa0, 0x8b, 0x9f, 0x8e}
DEFINE_GUID!{MF_AUDIO_RENDERER_ATTRIBUTE_ENDPOINT_ROLE,
    0x6ba644ff, 0x27c5, 0x4d02, 0x98, 0x87, 0xc2, 0x86, 0x19, 0xfd, 0xb9, 0x1b}
DEFINE_GUID!{MF_AUDIO_RENDERER_ATTRIBUTE_STREAM_CATEGORY,
    0xa9770471, 0x92ec, 0x4df4, 0x94, 0xfe, 0x81, 0xc3, 0x6f, 0x0c, 0x3a, 0x7a}
extern "system" {
    pub fn MFCreateVideoRendererActivate(
        hwndVideo: HWND,
        ppActivate: *mut *mut IMFActivate,
    ) -> HRESULT;
    pub fn MFCreateMPEG4MediaSink(
        pIByteStream: *mut IMFByteStream,
        pVideoMediaType: *mut IMFMediaType,
        pAudioMediaType: *mut IMFMediaType,
        ppIMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreate3GPMediaSink(
        pIByteStream: *mut IMFByteStream,
        pVideoMediaType: *mut IMFMediaType,
        pAudioMediaType: *mut IMFMediaType,
        ppIMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreateMP3MediaSink(
        pTargetByteStream: *mut IMFByteStream,
        ppMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreateAC3MediaSink(
        pTargetByteStream: *mut IMFByteStream,
        pAudioMediaType: *mut IMFMediaType,
        ppMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreateADTSMediaSink(
        pTargetByteStream: *mut IMFByteStream,
        pAudioMediaType: *mut IMFMediaType,
        ppMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreateMuxSink(
        guidOutputSubType: GUID,
        pOutputAttributes: *mut IMFAttributes,
        pOutputByteStream: *mut IMFByteStream,
        ppMuxSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreateFMPEG4MediaSink(
        pIByteStream: *mut IMFByteStream,
        pVideoMediaType: *mut IMFMediaType,
        pAudioMediaType: *mut IMFMediaType,
        ppIMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreateAVIMediaSink(
        pIByteStream: *mut IMFByteStream,
        pVideoMediaType: *mut IMFMediaType,
        pAudioMediaType: *mut IMFMediaType,
        ppIMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreateWAVEMediaSink(
        pTargetByteStream: *mut IMFByteStream,
        pAudioMediaType: *mut IMFMediaType,
        ppMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
}
RIDL!{#[uuid(0xDE9A6157, 0xF660, 0x4643, 0xB5, 0x6A, 0xDF, 0x9F, 0x79, 0x98, 0xC7, 0xCD)]
interface IMFTopoLoader(IMFTopoLoaderVtbl): IUnknown(IUnknownVtbl) {
    fn Load(
        pInputTopo: *mut IMFTopology,
        ppOutputTopo: *mut *mut IMFTopology,
        pCurrentTopo: *mut IMFTopology,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateTopoLoader(
        ppObj: *mut *mut IMFTopoLoader,
    ) -> HRESULT;
}
RIDL!{#[uuid(0xACF92459, 0x6A61, 0x42bd, 0xB5, 0x7C, 0xB4, 0x3E, 0x51, 0x20, 0x3C, 0xB0)]
interface IMFContentProtectionManager(IMFContentProtectionManagerVtbl): IUnknown(IUnknownVtbl) {
    fn BeginEnableContent(
        pEnablerActivate: *mut IMFActivate,
        pTopo: *mut IMFTopology,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndEnableContent(
        pResult: *mut IMFAsyncResult,
    ) -> HRESULT,
}}
ENUM!{enum MF_URL_TRUST_STATUS {
    MF_LICENSE_URL_UNTRUSTED,
    MF_LICENSE_URL_TRUSTED,
    MF_LICENSE_URL_TAMPERED,
}}
RIDL!{#[uuid(0xD3C4EF59, 0x49CE, 0x4381, 0x90, 0x71, 0xD5, 0xBC, 0xD0, 0x44, 0xC7, 0x70)]
interface IMFContentEnabler(IMFContentEnablerVtbl): IUnknown(IUnknownVtbl) {
    fn GetEnableType(
        pType: *mut GUID,
    ) -> HRESULT,
    fn GetEnableURL(
        ppwszURL: *mut LPWSTR,
        pcchURL: *mut DWORD,
        pTrustStatus: *mut MF_URL_TRUST_STATUS,
    ) -> HRESULT,
    fn GetEnableData(
        ppbData: *mut *mut BYTE,
        pcbData: *mut DWORD,
    ) -> HRESULT,
    fn IsAutomaticSupported(
        pfAutomatic: *mut BOOL,
    ) -> HRESULT,
    fn AutomaticEnable() -> HRESULT,
    fn MonitorEnable() -> HRESULT,
    fn Cancel() -> HRESULT,
}}
DEFINE_GUID!{MFENABLETYPE_WMDRMV1_LicenseAcquisition,
    0x4ff6eeaf, 0x0b43, 0x4797, 0x9b, 0x85, 0xab, 0xf3, 0x18, 0x15, 0xe7, 0xb0}
DEFINE_GUID!{MFENABLETYPE_WMDRMV7_LicenseAcquisition,
    0x003306df, 0x4a06, 0x4884, 0xa0, 0x97, 0xef, 0x6d, 0x22, 0xec, 0x84, 0xa3}
DEFINE_GUID!{MFENABLETYPE_WMDRMV7_Individualization,
    0xacd2c84a, 0xb303, 0x4f65, 0xbc, 0x2c, 0x2c, 0x84, 0x8d, 0x01, 0xa9, 0x89}
DEFINE_GUID!{MFENABLETYPE_MF_UpdateRevocationInformation,
    0xe558b0b5, 0xb3c4, 0x44a0, 0x92, 0x4c, 0x50, 0xd1, 0x78, 0x93, 0x23, 0x85}
DEFINE_GUID!{MFENABLETYPE_MF_UpdateUntrustedComponent,
    0x9879f3d6, 0xcee2, 0x48e6, 0xb5, 0x73, 0x97, 0x67, 0xab, 0x17, 0x2f, 0x16}
DEFINE_GUID!{MFENABLETYPE_MF_RebootRequired,
    0x6d4d3d4b, 0x0ece, 0x4652, 0x8b, 0x3a, 0xf2, 0xd2, 0x42, 0x60, 0xd8, 0x87}
pub const MFRR_INFO_VERSION: DWORD = 0;
pub const MF_USER_MODE_COMPONENT_LOAD: DWORD = 0x0000_0001;
pub const MF_KERNEL_MODE_COMPONENT_LOAD: DWORD = 0x0000_0002;
pub const MF_GRL_LOAD_FAILED: DWORD = 0x0000_0010;
pub const MF_INVALID_GRL_SIGNATURE: DWORD = 0x0000_0020;
pub const MF_GRL_ABSENT: DWORD = 0x0000_1000;
pub const MF_COMPONENT_REVOKED: DWORD = 0x0000_2000;
pub const MF_COMPONENT_INVALID_EKU: DWORD = 0x0000_4000;
pub const MF_COMPONENT_CERT_REVOKED: DWORD = 0x0000_8000;
pub const MF_COMPONENT_INVALID_ROOT: DWORD = 0x0001_0000;
pub const MF_COMPONENT_HS_CERT_REVOKED: DWORD = 0x0002_0000;
pub const MF_COMPONENT_LS_CERT_REVOKED: DWORD = 0x0004_0000;
pub const MF_BOOT_DRIVER_VERIFICATION_FAILED: DWORD = 0x0010_0000;
pub const MF_TEST_SIGNED_COMPONENT_LOADING: DWORD = 0x0100_0000;
pub const MF_MINCRYPT_FAILURE: DWORD = 0x1000_0000;
const SHA_HASH_LEN: usize = 20;
const STR_HASH_LEN: usize = 2*SHA_HASH_LEN + 3;
STRUCT!{struct MFRR_COMPONENT_HASH_INFO {
    ulReason: DWORD,
    rgHeaderHash: [WCHAR; STR_HASH_LEN],
    rgPublicKeyHash: [WCHAR; STR_HASH_LEN],
    wszName: [WCHAR; MAX_PATH],
}}
pub type PMFRR_COMPONENT_HASH_INFO = *mut MFRR_COMPONENT_HASH_INFO;
STRUCT!{struct MFRR_COMPONENTS {
    dwRRInfoVersion: DWORD,
    dwRRComponents: DWORD,
    pRRComponents: PMFRR_COMPONENT_HASH_INFO,
}}
pub type PMFRR_COMPONENTS = *mut MFRR_COMPONENTS;
STRUCT!{#[repr(packed)] struct ASF_FLAT_PICTURE {
    bPictureType: BYTE,
    dwDataLen: DWORD,
}}
STRUCT!{#[repr(packed)] struct ASF_FLAT_SYNCHRONISED_LYRICS {
    bTimeStampFormat: BYTE,
    bContentType: BYTE,
    dwLyricsLen: DWORD,
}}
RIDL!{#[uuid(0xF88CFB8C, 0xEF16, 0x4991, 0xB4, 0x50, 0xCB, 0x8C, 0x69, 0xE5, 0x17, 0x04)]
interface IMFMetadata(IMFMetadataVtbl): IUnknown(IUnknownVtbl) {
    fn SetLanguage(
        pwszRFC1766: LPCWSTR,
    ) -> HRESULT,
    fn GetLanguage(
        ppwszRFC1766: *mut LPWSTR,
    ) -> HRESULT,
    fn GetAllLanguages(
        ppvLanguages: *mut PROPVARIANT,
    ) -> HRESULT,
    fn SetProperty(
        pwszName: LPCWSTR,
        ppvValue: *const PROPVARIANT,
    ) -> HRESULT,
    fn GetProperty(
        pwszName: LPCWSTR,
        ppvValue: *mut PROPVARIANT,
    ) -> HRESULT,
    fn DeleteProperty(
        pwszName: LPCWSTR,
    ) -> HRESULT,
    fn GetAllPropertyNames(
        ppvNames: *mut PROPVARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56181D2D, 0xE221, 0x4adb, 0xB1, 0xC8, 0x3C, 0xEE, 0x6A, 0x53, 0xF7, 0x6F)]
interface IMFMetadataProvider(IMFMetadataProviderVtbl): IUnknown(IUnknownVtbl) {
    fn GetMFMetadata(
        pPresentationDescriptor: *mut IMFPresentationDescriptor,
        dwStreamIdentifier: DWORD,
        dwFlags: DWORD,
        ppMFMetadata: *mut *mut IMFMetadata,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_METADATA_PROVIDER_SERVICE,
    0xdb214084, 0x58a4, 0x4d2e, 0xb8, 0x4f, 0x6f, 0x75, 0x5b, 0x2f, 0x7a, 0x0d}
DEFINE_GUID!{MF_PROPERTY_HANDLER_SERVICE,
    0xa3face02, 0x32b8, 0x41dd, 0x90, 0xe7, 0x5f, 0xef, 0x7c, 0x89, 0x91, 0xb5}
ENUM!{enum MFRATE_DIRECTION {
    MFRATE_FORWARD,
    MFRATE_REVERSE,
}}
RIDL!{#[uuid(0x0a9ccdbc, 0xd797, 0x4563, 0x96, 0x67, 0x94, 0xec, 0x5d, 0x79, 0x29, 0x2d)]
interface IMFRateSupport(IMFRateSupportVtbl): IUnknown(IUnknownVtbl) {
    fn GetSlowestRate(
        eDirection: MFRATE_DIRECTION,
        fThin: BOOL,
        pflRate: *mut c_float,
    ) -> HRESULT,
    fn GetFastestRate(
        eDirection: MFRATE_DIRECTION,
        fThin: BOOL,
        pflRate: *mut c_float,
    ) -> HRESULT,
    fn IsRateSupported(
        fThin: BOOL,
        flRate: c_float,
        pflNearestSupportedRate: *mut c_float,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_RATE_CONTROL_SERVICE,
    0x866fa297, 0xb802, 0x4bf8, 0x9d, 0xc9, 0x5e, 0x3b, 0x6a, 0x9f, 0x53, 0xc9}
RIDL!{#[uuid(0x88ddcd21, 0x03c3, 0x4275, 0x91, 0xed, 0x55, 0xee, 0x39, 0x29, 0x32, 0x8f)]
interface IMFRateControl(IMFRateControlVtbl): IUnknown(IUnknownVtbl) {
    fn SetRate(
        fThin: BOOL,
        flRate: c_float,
    ) -> HRESULT,
    fn GetRate(
        pfThin: *mut BOOL,
        pflRate: *mut c_float,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xab9d8661, 0xf7e8, 0x4ef4, 0x98, 0x61, 0x89, 0xf3, 0x34, 0xf9, 0x4e, 0x74)]
interface IMFTimecodeTranslate(IMFTimecodeTranslateVtbl): IUnknown(IUnknownVtbl) {
    fn BeginConvertTimecodeToHNS(
        pPropVarTimecode: *const PROPVARIANT,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndConvertTimecodeToHNS(
        pResult: *mut IMFAsyncResult,
        phnsTime: *mut MFTIME,
    ) -> HRESULT,
    fn BeginConvertHNSToTimecode(
        hnsTime: MFTIME,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndConvertHNSToTimecode(
        pResult: *mut IMFAsyncResult,
        pPropVarTimecode: *mut PROPVARIANT,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_TIMECODE_SERVICE,
    0xa0d502a7, 0x0eb3, 0x4885, 0xb1, 0xb9, 0x9f, 0xeb, 0x0d, 0x08, 0x34, 0x54 }
RIDL!{#[uuid(0x26AFEA53, 0xD9ED, 0x42B5, 0xAB, 0x80, 0xE6, 0x4F, 0x9E, 0xE3, 0x47, 0x79)]
interface IMFSeekInfo(IMFSeekInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetNearestKeyFrames(
        pguidTimeFormat: *const GUID,
        pvarStartPosition: *const PROPVARIANT,
        pvarPreviousKeyFrame: *mut PROPVARIANT,
        pvarNextKeyFrame: *mut PROPVARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x089EDF13, 0xCF71, 0x4338, 0x8D, 0x13, 0x9E, 0x56, 0x9D, 0xBD, 0xC3, 0x19)]
interface IMFSimpleAudioVolume(IMFSimpleAudioVolumeVtbl): IUnknown(IUnknownVtbl) {
    fn SetMasterVolume(
        fLevel: c_float,
    ) -> HRESULT,
    fn GetMasterVolume(
        pfLevel: *mut c_float,
    ) -> HRESULT,
    fn SetMute(
        bMute: BOOL,
    ) -> HRESULT,
    fn GetMute(
        pbMute: *mut BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{MR_POLICY_VOLUME_SERVICE,
    0x1abaa2ac, 0x9d3b, 0x47c6, 0xab, 0x48, 0xc5, 0x95, 0x06, 0xde, 0x78, 0x4d}
DEFINE_GUID!{MR_CAPTURE_POLICY_VOLUME_SERVICE,
    0x24030acd, 0x107a, 0x4265, 0x97, 0x5c, 0x41, 0x4e, 0x33, 0xe6, 0x5f, 0x2a}
RIDL!{#[uuid(0x76B1BBDB, 0x4EC8, 0x4f36, 0xB1, 0x06, 0x70, 0xA9, 0x31, 0x6D, 0xF5, 0x93)]
interface IMFAudioStreamVolume(IMFAudioStreamVolumeVtbl): IUnknown(IUnknownVtbl) {
    fn GetChannelCount(
        pdwCount: *mut UINT32,
    ) -> HRESULT,
    fn SetChannelVolume(
        dwIndex: UINT32,
        fLevel: c_float,
    ) -> HRESULT,
    fn GetChannelVolume(
        dwIndex: UINT32,
        pfLevel: *mut c_float,
    ) -> HRESULT,
    fn SetAllVolumes(
        dwCount: UINT32,
        pfVolumes: *const c_float,
    ) -> HRESULT,
    fn GetAllVolumes(
        dwCount: UINT32,
        pfVolumes: *mut c_float,
    ) -> HRESULT,
}}
DEFINE_GUID!{MR_STREAM_VOLUME_SERVICE,
    0xf8b5fa2f, 0x32ef, 0x46f5, 0xb1, 0x72, 0x13, 0x21, 0x21, 0x2f, 0xb2, 0xc4}
RIDL!{#[uuid(0xa0638c2b, 0x6465, 0x4395, 0x9a, 0xe7, 0xa3, 0x21, 0xa9, 0xfd, 0x28, 0x56)]
interface IMFAudioPolicy(IMFAudioPolicyVtbl): IUnknown(IUnknownVtbl) {
    fn SetGroupingParam(
        rguidClass: REFGUID,
    ) -> HRESULT,
    fn GetGroupingParam(
        pguidClass: *mut GUID,
    ) -> HRESULT,
    fn SetDisplayName(
        pszName: LPCWSTR,
    ) -> HRESULT,
    fn GetDisplayName(
        pszName: *mut LPWSTR,
    ) -> HRESULT,
    fn SetIconPath(
        pszPath: LPCWSTR,
    ) -> HRESULT,
    fn GetIconPath(
        pszPath: *mut LPWSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{MR_AUDIO_POLICY_SERVICE,
    0x911fd737, 0x6775, 0x4ab0, 0xa6, 0x14, 0x29, 0x78, 0x62, 0xfd, 0xac, 0x88}
RIDL!{#[uuid(0x8C7B80BF, 0xEE42, 0x4b59, 0xB1, 0xDF, 0x55, 0x66, 0x8E, 0x1B, 0xDC, 0xA8)]
interface IMFSampleGrabberSinkCallback(IMFSampleGrabberSinkCallbackVtbl):
  IMFClockStateSink(IMFClockStateSinkVtbl) {
    fn OnSetPresentationClock(
        pPresentationClock: *mut IMFPresentationClock,
    ) -> HRESULT,
    fn OnProcessSample(
        guidMajorMediaType: REFGUID,
        dwSampleFlags: DWORD,
        llSampleTime: LONGLONG,
        llSampleDuration: LONGLONG,
        pSampleBuffer: *const BYTE,
        dwSampleSize: DWORD,
    ) -> HRESULT,
    fn OnShutdown() -> HRESULT,
}}
extern "system" {
    pub fn MFCreateSampleGrabberSinkActivate(
        pIMFMediaType: *mut IMFMediaType,
        pIMFSampleGrabberSinkCallback: *mut IMFSampleGrabberSinkCallback,
        ppIActivate: *mut *mut IMFActivate,
    ) -> HRESULT;
}
DEFINE_GUID!{MF_SAMPLEGRABBERSINK_SAMPLE_TIME_OFFSET,
    0x62e3d776, 0x8100, 0x4e03, 0xa6, 0xe8, 0xbd, 0x38, 0x57, 0xac, 0x9c, 0x47}
DEFINE_GUID!{MF_SAMPLEGRABBERSINK_IGNORE_CLOCK,
    0x0efda2c0, 0x2b69, 0x4e2e, 0xab, 0x8d, 0x46, 0xdc, 0xbf, 0xf7, 0xd2, 0x5d}
RIDL!{#[uuid(0xca86aa50, 0xc46e, 0x429e, 0xab, 0x27, 0x16, 0xd6, 0xac, 0x68, 0x44, 0xcb)]
interface IMFSampleGrabberSinkCallback2(IMFSampleGrabberSinkCallback2Vtbl):
  IMFSampleGrabberSinkCallback(IMFSampleGrabberSinkCallbackVtbl) {
    fn OnProcessSampleEx(
        guidMajorMediaType: REFGUID,
        dwSampleFlags: DWORD,
        llSampleTime: LONGLONG,
        llSampleDuration: LONGLONG,
        pSampleBuffer: *const BYTE,
        dwSampleSize: DWORD,
        pAttributes: *mut IMFAttributes,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_QUALITY_SERVICES,
    0xb7e2be11, 0x2f96, 0x4640, 0xb5, 0x2c, 0x28, 0x23, 0x65, 0xbd, 0xf1, 0x6c}
RIDL!{#[uuid(0x35FE1BB8, 0xA3A9, 0x40fe, 0xBB, 0xEC, 0xEB, 0x56, 0x9C, 0x9C, 0xCC, 0xA3)]
interface IMFWorkQueueServices(IMFWorkQueueServicesVtbl): IUnknown(IUnknownVtbl) {
    fn BeginRegisterTopologyWorkQueuesWithMMCSS(
        pCallback: *mut IMFAsyncCallback,
        pState: *mut IUnknown,
    ) -> HRESULT,
    fn EndRegisterTopologyWorkQueuesWithMMCSS(
        pResult: *mut IMFAsyncResult,
    ) -> HRESULT,
    fn BeginUnregisterTopologyWorkQueuesWithMMCSS(
        pCallback: *mut IMFAsyncCallback,
        pState: *mut IUnknown,
    ) -> HRESULT,
    fn EndUnregisterTopologyWorkQueuesWithMMCSS(
        pResult: *mut IMFAsyncResult,
    ) -> HRESULT,
    fn GetTopologyWorkQueueMMCSSClass(
        dwTopologyWorkQueueId: DWORD,
        pwszClass: LPWSTR,
        pcchClass: *mut DWORD,
    ) -> HRESULT,
    fn GetTopologyWorkQueueMMCSSTaskId(
        dwTopologyWorkQueueId: DWORD,
        pdwTaskId: *mut DWORD,
    ) -> HRESULT,
    fn BeginRegisterPlatformWorkQueueWithMMCSS(
        dwPlatformWorkQueue: DWORD,
        wszClass: LPCWSTR,
        dwTaskId: DWORD,
        pCallback: *mut IMFAsyncCallback,
        pState: *mut IUnknown,
    ) -> HRESULT,
    fn EndRegisterPlatformWorkQueueWithMMCSS(
        pResult: *mut IMFAsyncResult,
        pdwTaskId: *mut DWORD,
    ) -> HRESULT,
    fn BeginUnregisterPlatformWorkQueueWithMMCSS(
        dwPlatformWorkQueue: DWORD,
        pCallback: *mut IMFAsyncCallback,
        pState: *mut IUnknown,
    ) -> HRESULT,
    fn EndUnregisterPlatformWorkQueueWithMMCSS(
        pResult: *mut IMFAsyncResult,
    ) -> HRESULT,
    fn GetPlaftormWorkQueueMMCSSClass(
        dwPlatformWorkQueueId: DWORD,
        pwszClass: LPWSTR,
        pcchClass: *mut DWORD,
    ) -> HRESULT,
    fn GetPlatformWorkQueueMMCSSTaskId(
        dwPlatformWorkQueueId: DWORD,
        pdwTaskId: *mut DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_WORKQUEUE_SERVICES,
    0x8e37d489, 0x41e0, 0x413a, 0x90, 0x68, 0x28, 0x7c, 0x88, 0x6d, 0x8d, 0xda}
RIDL!{#[uuid(0x96bf961b, 0x40fe, 0x42f1, 0xba, 0x9d, 0x32, 0x02, 0x38, 0xb4, 0x97, 0x00)]
interface IMFWorkQueueServicesEx(IMFWorkQueueServicesExVtbl):
  IMFWorkQueueServices(IMFWorkQueueServicesVtbl) {
    fn GetTopologyWorkQueueMMCSSPriority(
        dwTopologyWorkQueueId: DWORD,
        plPriority: *mut LONG,
    ) -> HRESULT,
    fn BeginRegisterPlatformWorkQueueWithMMCSSEx(
        dwPlatformWorkQueue: DWORD,
        wszClass: LPCWSTR,
        dwTaskId: DWORD,
        lPriority: LONG,
        pCallback: *mut IMFAsyncCallback,
        pState: *mut IUnknown,
    ) -> HRESULT,
    fn GetPlatformWorkQueueMMCSSPriority(
        dwPlatformWorkQueueId: DWORD,
        plPriority: *mut LONG,
    ) -> HRESULT,
}}
ENUM!{enum MF_QUALITY_DROP_MODE {
    MF_DROP_MODE_NONE,
    MF_DROP_MODE_1,
    MF_DROP_MODE_2,
    MF_DROP_MODE_3,
    MF_DROP_MODE_4,
    MF_DROP_MODE_5,
    MF_NUM_DROP_MODES,
}}
ENUM!{enum MF_QUALITY_LEVEL {
    MF_QUALITY_NORMAL,
    MF_QUALITY_NORMAL_MINUS_1,
    MF_QUALITY_NORMAL_MINUS_2,
    MF_QUALITY_NORMAL_MINUS_3,
    MF_QUALITY_NORMAL_MINUS_4,
    MF_QUALITY_NORMAL_MINUS_5,
    MF_NUM_QUALITY_LEVELS,
}}
ENUM!{enum MF_QUALITY_ADVISE_FLAGS {
    MF_QUALITY_CANNOT_KEEP_UP = 0x1,
}}
RIDL!{#[uuid(0x8D009D86, 0x5B9F, 0x4115, 0xB1, 0xFC, 0x9F, 0x80, 0xD5, 0x2A, 0xB8, 0xAB)]
interface IMFQualityManager(IMFQualityManagerVtbl): IUnknown(IUnknownVtbl) {
    fn NotifyTopology(
        pTopology: *mut IMFTopology,
    ) -> HRESULT,
    fn NotifyPresentationClock(
        pClock: *mut IMFPresentationClock,
    ) -> HRESULT,
    fn NotifyProcessInput(
        pNode: *mut IMFTopologyNode,
        lInputIndex: c_long,
        pSample: *mut IMFSample,
    ) -> HRESULT,
    fn NotifyProcessOutput(
        pNode: *mut IMFTopologyNode,
        lOutputIndex: c_long,
        pSample: *mut IMFSample,
    ) -> HRESULT,
    fn NotifyQualityEvent(
        pObject: *mut IUnknown,
        pEvent: *mut IMFMediaEvent,
    ) -> HRESULT,
    fn Shutdown() -> HRESULT,
}}
extern "system" {
    pub fn MFCreateStandardQualityManager(
        ppQualityManager: *mut *mut IMFQualityManager,
    ) -> HRESULT;
}
DEFINE_GUID!{MF_QUALITY_NOTIFY_PROCESSING_LATENCY,
    0xf6b44af8, 0x604d, 0x46fe, 0xa9, 0x5d, 0x45, 0x47, 0x9b, 0x10, 0xc9, 0xbc }
DEFINE_GUID!{MF_QUALITY_NOTIFY_SAMPLE_LAG,
    0x30d15206, 0xed2a, 0x4760, 0xbe, 0x17, 0xeb, 0x4a, 0x9f, 0x12, 0x29, 0x5c }
RIDL!{#[uuid(0xEC15E2E9, 0xE36B, 0x4f7c, 0x87, 0x58, 0x77, 0xD4, 0x52, 0xEF, 0x4C, 0xE7)]
interface IMFQualityAdvise(IMFQualityAdviseVtbl): IUnknown(IUnknownVtbl) {
    fn SetDropMode(
        eDropMode: MF_QUALITY_DROP_MODE,
    ) -> HRESULT,
    fn SetQualityLevel(
        eQualityLevel: MF_QUALITY_LEVEL,
    ) -> HRESULT,
    fn GetDropMode(
        peDropMode: *mut MF_QUALITY_DROP_MODE,
    ) -> HRESULT,
    fn GetQualityLevel(
        peQualityLevel: *mut MF_QUALITY_LEVEL,
    ) -> HRESULT,
    fn DropTime(
        hnsAmountToDrop: LONGLONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xF3706F0D, 0x8EA2, 0x4886, 0x80, 0x00, 0x71, 0x55, 0xE9, 0xEC, 0x2E, 0xAE)]
interface IMFQualityAdvise2(IMFQualityAdvise2Vtbl):
  IMFQualityAdvise(IMFQualityAdviseVtbl) {
    fn NotifyQualityEvent(
        pEvent: *mut IMFMediaEvent,
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdfcd8e4d, 0x30b5, 0x4567, 0xac, 0xaa, 0x8e, 0xb5, 0xb7, 0x85, 0x3d, 0xc9)]
interface IMFQualityAdviseLimits(IMFQualityAdviseLimitsVtbl): IUnknown(IUnknownVtbl) {
    fn GetMaximumDropMode(
        peDropMode: *mut MF_QUALITY_DROP_MODE,
    ) -> HRESULT,
    fn GetMinimumQualityLevel(
        peQualityLevel: *mut MF_QUALITY_LEVEL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2347D60B, 0x3FB5, 0x480c, 0x88, 0x03, 0x8D, 0xF3, 0xAD, 0xCD, 0x3E, 0xF0)]
interface IMFRealTimeClient(IMFRealTimeClientVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterThreads(
        dwTaskIndex: DWORD,
        wszClass: LPCWSTR,
    ) -> HRESULT,
    fn UnregisterThreads() -> HRESULT,
    fn SetWorkQueue(
        dwWorkQueueId: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x03910848, 0xAB16, 0x4611, 0xB1, 0x00, 0x17, 0xB8, 0x8A, 0xE2, 0xF2, 0x48)]
interface IMFRealTimeClientEx(IMFRealTimeClientExVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterThreadsEx(
        pdwTaskIndex: *mut DWORD,
        wszClassName: LPCWSTR,
        lBasePriority: LONG,
    ) -> HRESULT,
    fn UnregisterThreads() -> HRESULT,
    fn SetWorkQueueEx(
        dwMultithreadedWorkQueueId: DWORD,
        lWorkItemBasePriority: LONG,
    ) -> HRESULT,
}}
pub type MFSequencerElementId = DWORD;
pub const MFSEQUENCER_INVALID_ELEMENT_ID: DWORD = -1i32 as DWORD;
ENUM!{enum MFSequencerTopologyFlags {
    SequencerTopologyFlags_Last = 0x1,
}}
RIDL!{#[uuid(0x197CD219, 0x19CB, 0x4de1, 0xA6, 0x4C, 0xAC, 0xF2, 0xED, 0xCB, 0xE5, 0x9E)]
interface IMFSequencerSource(IMFSequencerSourceVtbl): IUnknown(IUnknownVtbl) {
    fn AppendTopology(
        pTopology: *mut IMFTopology,
        dwFlags: DWORD,
        pdwId: *mut MFSequencerElementId,
    ) -> HRESULT,
    fn DeleteTopology(
        dwId: MFSequencerElementId,
    ) -> HRESULT,
    fn GetPresentationContext(
        pPD: *mut IMFPresentationDescriptor,
        pId: *mut MFSequencerElementId,
        ppTopology: *mut *mut IMFTopology,
    ) -> HRESULT,
    fn UpdateTopology(
        dwId: MFSequencerElementId,
        pTopology: *mut IMFTopology,
    ) -> HRESULT,
    fn UpdateTopologyFlags(
        dwId: MFSequencerElementId,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_TIME_FORMAT_SEGMENT_OFFSET,
    0xc8b8be77, 0x869c, 0x431d, 0x81, 0x2e, 0x16, 0x96, 0x93, 0xf6, 0x5a, 0x39 }
extern "system" {
    pub fn MFCreateSequencerSource(
        pReserved: *mut IUnknown,
        ppSequencerSource: *mut *mut IMFSequencerSource,
    ) -> HRESULT;
    pub fn MFCreateSequencerSegmentOffset(
        dwId: MFSequencerElementId,
        hnsOffset: MFTIME,
        pvarSegmentOffset: *mut PROPVARIANT,
    ) -> HRESULT;
    pub fn MFCreateAggregateSource(
        pSourceCollection: *mut IMFCollection,
        ppAggSource: *mut *mut IMFMediaSource,
    ) -> HRESULT;
}
RIDL!{#[uuid(0x0E1D6009, 0xC9F3, 0x442d, 0x8C, 0x51, 0xA4, 0x2D, 0x2D, 0x49, 0x45, 0x2F)]
interface IMFMediaSourceTopologyProvider(IMFMediaSourceTopologyProviderVtbl):
  IUnknown(IUnknownVtbl) {
    fn GetMediaSourceTopology(
        pPresentationDescriptor: *mut IMFPresentationDescriptor,
        ppTopology: *mut *mut IMFTopology,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0E1D600a, 0xC9F3, 0x442d, 0x8C, 0x51, 0xA4, 0x2D, 0x2D, 0x49, 0x45, 0x2F)]
interface IMFMediaSourcePresentationProvider(IMFMediaSourcePresentationProviderVtbl):
  IUnknown(IUnknownVtbl) {
    fn ForceEndOfPresentation(
        pPresentationDescriptor: *mut IMFPresentationDescriptor,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_SOURCE_PRESENTATION_PROVIDER_SERVICE,
    0xe002aadc, 0xf4af, 0x4ee5, 0x98, 0x47, 0x05, 0x3e, 0xdf, 0x84, 0x04, 0x26 }
UNION!{union MFTOPONODE_ATTRIBUTE_UPDATE_u {
    [u64; 1],
    u32 u32_mut: UINT32,
    u64 u64_mut: UINT64,
    d d_mut: c_double,
}}
STRUCT!{struct MFTOPONODE_ATTRIBUTE_UPDATE {
    NodeId: TOPOID,
    guidAttributeKey: GUID,
    attrType: MF_ATTRIBUTE_TYPE,
    u: MFTOPONODE_ATTRIBUTE_UPDATE_u,
}}
RIDL!{#[uuid(0x676aa6dd, 0x238a, 0x410d, 0xbb, 0x99, 0x65, 0x66, 0x8d, 0x01, 0x60, 0x5a)]
interface IMFTopologyNodeAttributeEditor(IMFTopologyNodeAttributeEditorVtbl):
  IUnknown(IUnknownVtbl) {
    fn UpdateNodeAttributes(
        TopoId: TOPOID,
        cUpdates: DWORD,
        pUpdates: *mut MFTOPONODE_ATTRIBUTE_UPDATE,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_TOPONODE_ATTRIBUTE_EDITOR_SERVICE,
    0x65656e1a, 0x077f, 0x4472, 0x83, 0xef, 0x31, 0x6f, 0x11, 0xd5, 0x08, 0x7a }
STRUCT!{struct MF_LEAKY_BUCKET_PAIR {
    dwBitrate: DWORD,
    msBufferWindow: DWORD,
}}
STRUCT!{struct MFBYTESTREAM_BUFFERING_PARAMS {
    cbTotalFileSize: QWORD,
    cbPlayableDataSize: QWORD,
    prgBuckets: *mut MF_LEAKY_BUCKET_PAIR,
    cBuckets: DWORD,
    qwNetBufferingTime: QWORD,
    qwExtraBufferingTimeDuringSeek: QWORD,
    qwPlayDuration: QWORD,
    dRate: c_float,
}}
RIDL!{#[uuid(0x6d66d782, 0x1d4f, 0x4db7, 0x8c, 0x63, 0xcb, 0x8c, 0x77, 0xf1, 0xef, 0x5e)]
interface IMFByteStreamBuffering(IMFByteStreamBufferingVtbl): IUnknown(IUnknownVtbl) {
    fn SetBufferingParams(
        pParams: *mut MFBYTESTREAM_BUFFERING_PARAMS,
    ) -> HRESULT,
    fn EnableBuffering(
        fEnable: BOOL,
    ) -> HRESULT,
    fn StopBuffering() -> HRESULT,
}}
RIDL!{#[uuid(0xF5042EA4, 0x7A96, 0x4a75, 0xAA, 0x7B, 0x2B, 0xE1, 0xEF, 0x7F, 0x88, 0xD5)]
interface IMFByteStreamCacheControl(IMFByteStreamCacheControlVtbl):
  IUnknown(IUnknownVtbl) {
    fn StopBackgroundTransfer() -> HRESULT,
}}
RIDL!{#[uuid(0x64976BFA, 0xFB61, 0x4041, 0x90, 0x69, 0x8C, 0x9A, 0x5F, 0x65, 0x9B, 0xEB)]
interface IMFByteStreamTimeSeek(IMFByteStreamTimeSeekVtbl): IUnknown(IUnknownVtbl) {
    fn IsTimeSeekSupported(
        pfTimeSeekIsSupported: *mut BOOL,
    ) -> HRESULT,
    fn TimeSeek(
        qwTimePosition: QWORD,
    ) -> HRESULT,
    fn GetTimeSeekResult(
        pqwStartTime: *mut QWORD,
        pqwStopTime: *mut QWORD,
        pqwDuration: *mut QWORD,
    ) -> HRESULT,
}}
STRUCT!{struct MF_BYTE_STREAM_CACHE_RANGE {
    qwStartOffset: QWORD,
    qwEndOffset: QWORD,
}}
RIDL!{#[uuid(0x71CE469C, 0xF34B, 0x49EA, 0xA5, 0x6B, 0x2D, 0x2A, 0x10, 0xE5, 0x11, 0x49)]
interface IMFByteStreamCacheControl2(IMFByteStreamCacheControl2Vtbl):
  IMFByteStreamCacheControl(IMFByteStreamCacheControlVtbl) {
    fn GetByteRanges(
        pcRanges: *mut DWORD,
        ppRanges: *mut *mut MF_BYTE_STREAM_CACHE_RANGE,
    ) -> HRESULT,
    fn SetCacheLimit(
        qwBytes: QWORD,
    ) -> HRESULT,
    fn IsBackgroundTransferActive(
        pfActive: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x5b87ef6a, 0x7ed8, 0x434f, 0xba, 0x0e, 0x18, 0x4f, 0xac, 0x16, 0x28, 0xd1)]
interface IMFNetCredential(IMFNetCredentialVtbl): IUnknown(IUnknownVtbl) {
    fn SetUser(
        pbData: *mut BYTE,
        cbData: DWORD,
        fDataIsEncrypted: BOOL,
    ) -> HRESULT,
    fn SetPassword(
        pbData: *mut BYTE,
        cbData: DWORD,
        fDataIsEncrypted: BOOL,
    ) -> HRESULT,
    fn GetUser(
        pbData: *mut BYTE,
        pcbData: *mut DWORD,
        fEncryptData: BOOL,
    ) -> HRESULT,
    fn GetPassword(
        pbData: *mut BYTE,
        pcbData: *mut DWORD,
        fEncryptData: BOOL,
    ) -> HRESULT,
    fn LoggedOnUser(
        pfLoggedOnUser: *mut BOOL,
    ) -> HRESULT,
}}
STRUCT!{struct MFNetCredentialManagerGetParam {
    hrOp: HRESULT,
    fAllowLoggedOnUser: BOOL,
    fClearTextPackage: BOOL,
    pszUrl: LPCWSTR,
    pszSite: LPCWSTR,
    pszRealm: LPCWSTR,
    pszPackage: LPCWSTR,
    nRetries: LONG,
}}
RIDL!{#[uuid(0x5b87ef6b, 0x7ed8, 0x434f, 0xba, 0x0e, 0x18, 0x4f, 0xac, 0x16, 0x28, 0xd1)]
interface IMFNetCredentialManager(IMFNetCredentialManagerVtbl): IUnknown(IUnknownVtbl) {
    fn BeginGetCredentials(
        pParam: *mut MFNetCredentialManagerGetParam,
        pCallback: *mut IMFAsyncCallback,
        pState: *mut IUnknown,
    ) -> HRESULT,
    fn EndGetCredentials(
        pResult: *mut IMFAsyncResult,
        ppCred: *mut *mut IMFNetCredential,
    ) -> HRESULT,
    fn SetGood(
        pCred: *mut IMFNetCredential,
        fGood: BOOL,
    ) -> HRESULT,
}}
ENUM!{enum MFNetCredentialRequirements {
    REQUIRE_PROMPT = 0x1,
    REQUIRE_SAVE_SELECTED = 0x2,
}}
ENUM!{enum _MFNetCredentialOptions {
    MFNET_CREDENTIAL_SAVE = 0x1,
    MFNET_CREDENTIAL_DONT_CACHE = 0x2,
    MFNET_CREDENTIAL_ALLOW_CLEAR_TEXT = 0x4,
}}
ENUM!{enum _MFNetAuthenticationFlags {
    MFNET_AUTHENTICATION_PROXY = 0x1,
    MFNET_AUTHENTICATION_CLEAR_TEXT = 0x2,
    MFNET_AUTHENTICATION_LOGGED_ON_USER = 0x4,
}}
RIDL!{#[uuid(0x5b87ef6c, 0x7ed8, 0x434f, 0xba, 0x0e, 0x18, 0x4f, 0xac, 0x16, 0x28, 0xd1)]
interface IMFNetCredentialCache(IMFNetCredentialCacheVtbl): IUnknown(IUnknownVtbl) {
    fn GetCredential(
        pszUrl: LPCWSTR,
        pszRealm: LPCWSTR,
        dwAuthenticationFlags: DWORD,
        ppCred: *mut *mut IMFNetCredential,
        pdwRequirementsFlags: *mut DWORD,
    ) -> HRESULT,
    fn SetGood(
        pCred: *mut IMFNetCredential,
        fGood: BOOL,
    ) -> HRESULT,
    fn SetUserOptions(
        pCred: *mut IMFNetCredential,
        dwOptionsFlags: DWORD,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateCredentialCache(
        ppCache: *mut *mut IMFNetCredentialCache,
    ) -> HRESULT;
}
RIDL!{#[uuid(0x61f7d887, 0x1230, 0x4a8b, 0xae, 0xba, 0x8a, 0xd4, 0x34, 0xd1, 0xa6, 0x4d)]
interface IMFSSLCertificateManager(IMFSSLCertificateManagerVtbl): IUnknown(IUnknownVtbl) {
    fn GetClientCertificate(
        pszURL: LPCWSTR,
        ppbData: *mut *mut BYTE,
        pcbData: *mut DWORD,
    ) -> HRESULT,
    fn BeginGetClientCertificate(
        pszURL: LPCWSTR,
        pCallback: *mut IMFAsyncCallback,
        pState: *mut IUnknown,
    ) -> HRESULT,
    fn EndGetClientCertificate(
        pResult: *mut IMFAsyncResult,
        ppbData: *mut *mut BYTE,
        pcbData: *mut DWORD,
    ) -> HRESULT,
    fn GetCertificatePolicy(
        pszURL: LPCWSTR,
        pfOverrideAutomaticCheck: *mut BOOL,
        pfClientCertificateAvailable: *mut BOOL,
    ) -> HRESULT,
    fn OnServerCertificate(
        pszURL: LPCWSTR,
        pbData: *mut BYTE,
        cbData: DWORD,
        pfIsGood: *mut BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{MFNETSOURCE_SSLCERTIFICATE_MANAGER,
    0x55e6cb27, 0xe69b, 0x4267, 0x94, 0x0c, 0x2d, 0x7e, 0xc5, 0xbb, 0x8a, 0x0f }
RIDL!{#[uuid(0x091878a3, 0xbf11, 0x4a5c, 0xbc, 0x9f, 0x33, 0x99, 0x5b, 0x06, 0xef, 0x2d)]
interface IMFNetResourceFilter(IMFNetResourceFilterVtbl): IUnknown(IUnknownVtbl) {
    fn OnRedirect(
        pszUrl: LPCWSTR,
        pvbCancel: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn OnSendingRequest(
        pszUrl: LPCWSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{MFNETSOURCE_RESOURCE_FILTER,
    0x815d0ff6, 0x265a, 0x4477, 0x9e, 0x46, 0x7b, 0x80, 0xad, 0x80, 0xb5, 0xfb}
RIDL!{#[uuid(0x059054B3, 0x027C, 0x494C, 0xA2, 0x7D, 0x91, 0x13, 0x29, 0x1C, 0xF8, 0x7F)]
interface IMFSourceOpenMonitor(IMFSourceOpenMonitorVtbl): IUnknown(IUnknownVtbl) {
    fn OnSourceEvent(
        pEvent: *mut IMFMediaEvent,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe9cd0383, 0xa268, 0x4bb4, 0x82, 0xde, 0x65, 0x8d, 0x53, 0x57, 0x4d, 0x41)]
interface IMFNetProxyLocator(IMFNetProxyLocatorVtbl): IUnknown(IUnknownVtbl) {
    fn FindFirstProxy(
        pszHost: LPCWSTR,
        pszUrl: LPCWSTR,
        fReserved: BOOL,
    ) -> HRESULT,
    fn FindNextProxy() -> HRESULT,
    fn RegisterProxyResult(
        hrOp: HRESULT,
    ) -> HRESULT,
    fn GetCurrentProxy(
        pszStr: LPWSTR,
        pcchStr: *mut DWORD,
    ) -> HRESULT,
    fn Clone(
        ppProxyLocator: *mut *mut IMFNetProxyLocator,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateProxyLocator(
        pszProtocol: LPCWSTR,
        pProxyConfig: *mut IPropertyStore,
        ppProxyLocator: *mut *mut IMFNetProxyLocator,
    ) -> HRESULT;
}
RIDL!{#[uuid(0xe9cd0384, 0xa268, 0x4bb4, 0x82, 0xde, 0x65, 0x8d, 0x53, 0x57, 0x4d, 0x41)]
interface IMFNetProxyLocatorFactory(IMFNetProxyLocatorFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn CreateProxyLocator(
        pszProtocol: LPCWSTR,
        ppProxyLocator: *mut *mut IMFNetProxyLocator,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe9931663, 0x80bf, 0x4c6e, 0x98, 0xaf, 0x5d, 0xcf, 0x58, 0x74, 0x7d, 0x1f)]
interface IMFSaveJob(IMFSaveJobVtbl): IUnknown(IUnknownVtbl) {
    fn BeginSave(
        pStream: *mut IMFByteStream,
        pCallback: *mut IMFAsyncCallback,
        pState: *mut IUnknown,
    ) -> HRESULT,
    fn EndSave(
        pResult: *mut IMFAsyncResult,
    ) -> HRESULT,
    fn CancelSave() -> HRESULT,
    fn GetProgress(
        pdwPercentComplete: *mut DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{MFNET_SAVEJOB_SERVICE,
    0xb85a587f, 0x3d02, 0x4e52, 0x95, 0x65, 0x55, 0xd3, 0xec, 0x1e, 0x7f, 0xf7 }
ENUM!{enum MFNETSOURCE_PROTOCOL_TYPE {
    MFNETSOURCE_UNDEFINED,
    MFNETSOURCE_HTTP,
    MFNETSOURCE_RTSP,
    MFNETSOURCE_FILE,
    MFNETSOURCE_MULTICAST,
}}
RIDL!{#[uuid(0x7BE19E73, 0xC9BF, 0x468a, 0xAC, 0x5A, 0xA5, 0xE8, 0x65, 0x3B, 0xEC, 0x87)]
interface IMFNetSchemeHandlerConfig(IMFNetSchemeHandlerConfigVtbl): IUnknown(IUnknownVtbl) {
    fn GetNumberOfSupportedProtocols(
        pcProtocols: *mut ULONG,
    ) -> HRESULT,
    fn GetSupportedProtocolType(
        nProtocolIndex: ULONG,
        pnProtocolType: *mut MFNETSOURCE_PROTOCOL_TYPE,
    ) -> HRESULT,
    fn ResetProtocolRolloverSettings() -> HRESULT,
}}
extern "system" {
    pub fn MFCreateNetSchemePlugin(
        riid: REFIID,
        ppvHandler: *mut LPVOID,
    ) -> HRESULT;
}
ENUM!{enum MFNETSOURCE_TRANSPORT_TYPE {
    MFNETSOURCE_UDP,
    MFNETSOURCE_TCP,
}}
ENUM!{enum MFNETSOURCE_CACHE_STATE {
    MFNETSOURCE_CACHE_UNAVAILABLE,
    MFNETSOURCE_CACHE_ACTIVE_WRITING,
    MFNETSOURCE_CACHE_ACTIVE_COMPLETE,
}}
ENUM!{enum MFNETSOURCE_STATISTICS_IDS {
    MFNETSOURCE_RECVPACKETS_ID,
    MFNETSOURCE_LOSTPACKETS_ID,
    MFNETSOURCE_RESENDSREQUESTED_ID,
    MFNETSOURCE_RESENDSRECEIVED_ID,
    MFNETSOURCE_RECOVEREDBYECCPACKETS_ID,
    MFNETSOURCE_RECOVEREDBYRTXPACKETS_ID,
    MFNETSOURCE_OUTPACKETS_ID,
    MFNETSOURCE_RECVRATE_ID,
    MFNETSOURCE_AVGBANDWIDTHBPS_ID,
    MFNETSOURCE_BYTESRECEIVED_ID,
    MFNETSOURCE_PROTOCOL_ID,
    MFNETSOURCE_TRANSPORT_ID,
    MFNETSOURCE_CACHE_STATE_ID,
    MFNETSOURCE_LINKBANDWIDTH_ID,
    MFNETSOURCE_CONTENTBITRATE_ID,
    MFNETSOURCE_SPEEDFACTOR_ID,
    MFNETSOURCE_BUFFERSIZE_ID,
    MFNETSOURCE_BUFFERPROGRESS_ID,
    MFNETSOURCE_LASTBWSWITCHTS_ID,
    MFNETSOURCE_SEEKRANGESTART_ID,
    MFNETSOURCE_SEEKRANGEEND_ID,
    MFNETSOURCE_BUFFERINGCOUNT_ID,
    MFNETSOURCE_INCORRECTLYSIGNEDPACKETS_ID,
    MFNETSOURCE_SIGNEDSESSION_ID,
    MFNETSOURCE_MAXBITRATE_ID,
    MFNETSOURCE_RECEPTION_QUALITY_ID,
    MFNETSOURCE_RECOVEREDPACKETS_ID,
    MFNETSOURCE_VBR_ID,
    MFNETSOURCE_DOWNLOADPROGRESS_ID,
    MFNETSOURCE_UNPREDEFINEDPROTOCOLNAME_ID,
}}
DEFINE_GUID!{MFNETSOURCE_STATISTICS_SERVICE,
    0x3cb1f275, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_STATISTICS,
    0x3cb1f274, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_BUFFERINGTIME,
    0x3cb1f276, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_ACCELERATEDSTREAMINGDURATION,
    0x3cb1f277, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_MAXUDPACCELERATEDSTREAMINGDURATION,
    0x4aab2879, 0xbbe1, 0x4994, 0x9f, 0xf0, 0x54, 0x95, 0xbd, 0x25, 0x01, 0x29}
DEFINE_GUID!{MFNETSOURCE_MAXBUFFERTIMEMS,
    0x408b24e6, 0x4038, 0x4401, 0xb5, 0xb2, 0xfe, 0x70, 0x1a, 0x9e, 0xbf, 0x10}
DEFINE_GUID!{MFNETSOURCE_CONNECTIONBANDWIDTH,
    0x3cb1f278, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_CACHEENABLED,
    0x3cb1f279, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_AUTORECONNECTLIMIT,
    0x3cb1f27a, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_RESENDSENABLED,
    0x3cb1f27b, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_THINNINGENABLED,
    0x3cb1f27c, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_PROTOCOL,
    0x3cb1f27d, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_TRANSPORT,
    0x3cb1f27e, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_PREVIEWMODEENABLED,
    0x3cb1f27f, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_CREDENTIAL_MANAGER,
    0x3cb1f280, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_PPBANDWIDTH,
    0x3cb1f281, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_AUTORECONNECTPROGRESS,
    0x3cb1f282, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_PROXYLOCATORFACTORY,
    0x3cb1f283, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_BROWSERUSERAGENT,
    0x3cb1f28b, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_BROWSERWEBPAGE,
    0x3cb1f28c, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_PLAYERVERSION,
    0x3cb1f28d, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_PLAYERID,
    0x3cb1f28e, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_HOSTEXE,
    0x3cb1f28f, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_HOSTVERSION,
    0x3cb1f291, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_PLAYERUSERAGENT,
    0x3cb1f292, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_CLIENTGUID,
    0x60a2c4a6, 0xf197, 0x4c14, 0xa5, 0xbf, 0x88, 0x83, 0x0d, 0x24, 0x58, 0xaf}
DEFINE_GUID!{MFNETSOURCE_LOGURL,
    0x3cb1f293, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_ENABLE_UDP,
    0x3cb1f294, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_ENABLE_TCP,
    0x3cb1f295, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_ENABLE_MSB,
    0x3cb1f296, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_ENABLE_RTSP,
    0x3cb1f298, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_ENABLE_HTTP,
    0x3cb1f299, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_ENABLE_STREAMING,
    0x3cb1f29c, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_ENABLE_DOWNLOAD,
    0x3cb1f29d, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_ENABLE_PRIVATEMODE,
    0x824779d8, 0xf18b, 0x4405, 0x8c, 0xf1, 0x46, 0x4f, 0xb5, 0xaa, 0x8f, 0x71}
DEFINE_GUID!{MFNETSOURCE_UDP_PORT_RANGE,
    0x3cb1f29a, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_PROXYINFO,
    0x3cb1f29b, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_DRMNET_LICENSE_REPRESENTATION,
    0x47eae1bd, 0xbdfe, 0x42e2, 0x82, 0xf3, 0x54, 0xa4, 0x8c, 0x17, 0x96, 0x2d}
DEFINE_GUID!{MFNETSOURCE_PROXYSETTINGS,
    0x3cb1f287, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_PROXYHOSTNAME,
    0x3cb1f284, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_PROXYPORT,
    0x3cb1f288, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_PROXYEXCEPTIONLIST,
    0x3cb1f285, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_PROXYBYPASSFORLOCAL,
    0x3cb1f286, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_PROXYRERUNAUTODETECTION,
    0x3cb1f289, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1}
DEFINE_GUID!{MFNETSOURCE_STREAM_LANGUAGE,
    0x9ab44318, 0xf7cd, 0x4f2d, 0x8d, 0x6d, 0xfa, 0x35, 0xb4, 0x92, 0xce, 0xcb}
DEFINE_GUID!{MFNETSOURCE_LOGPARAMS,
    0x64936ae8, 0x9418, 0x453a, 0x8c, 0xda, 0x3e, 0x0a, 0x66, 0x8b, 0x35, 0x3b}
DEFINE_GUID!{MFNETSOURCE_PEERMANAGER,
    0x48b29adb, 0xfebf, 0x45ee, 0xa9, 0xbf, 0xef, 0xb8, 0x1c, 0x49, 0x2e, 0xfc}
DEFINE_GUID!{MFNETSOURCE_FRIENDLYNAME,
    0x5b2a7757, 0xbc6b, 0x447e, 0xaa, 0x06, 0x0d, 0xda, 0x1c, 0x64, 0x6e, 0x2f}
ENUM!{enum MFNET_PROXYSETTINGS {
    MFNET_PROXYSETTING_NONE,
    MFNET_PROXYSETTING_MANUAL,
    MFNET_PROXYSETTING_AUTO,
    MFNET_PROXYSETTING_BROWSER,
}}
RIDL!{#[uuid(0x6D4C7B74, 0x52A0, 0x4bb7, 0xB0, 0xDB, 0x55, 0xF2, 0x9F, 0x47, 0xA6, 0x68)]
interface IMFSchemeHandler(IMFSchemeHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn BeginCreateObject(
        pwszURL: LPCWSTR,
        dwFlags: DWORD,
        pProps: *mut IPropertyStore,
        ppIUnknownCancelCookie: *mut *mut IUnknown,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndCreateObject(
        pResult: *mut IMFAsyncResult,
        pObjectType: *mut MF_OBJECT_TYPE,
        ppObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn CancelObjectCreation(
        pIUnknownCancelCookie: *mut IUnknown,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_BYTESTREAMHANDLER_ACCEPTS_SHARE_WRITE,
    0xa6e1f733, 0x3001, 0x4915, 0x81, 0x50, 0x15, 0x58, 0xa2, 0x18, 0x0e, 0xc8}
RIDL!{#[uuid(0xBB420AA4, 0x765B, 0x4a1f, 0x91, 0xFE, 0xD6, 0xA8, 0xA1, 0x43, 0x92, 0x4C)]
interface IMFByteStreamHandler(IMFByteStreamHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn BeginCreateObject(
        pByteStream: *mut IMFByteStream,
        pwszURL: LPCWSTR,
        dwFlags: DWORD,
        pProps: *mut IPropertyStore,
        ppIUnknownCancelCookie: *mut *mut IUnknown,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndCreateObject(
        pResult: *mut IMFAsyncResult,
        pObjectType: *mut MF_OBJECT_TYPE,
        ppObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn CancelObjectCreation(
        pIUnknownCancelCookie: *mut IUnknown,
    ) -> HRESULT,
    fn GetMaxNumberOfBytesRequiredForResolution(
        pqwBytes: *mut QWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_BYTESTREAM_SERVICE,
    0xab025e2b, 0x16d9, 0x4180, 0xa1, 0x27, 0xba, 0x6c, 0x70, 0x15, 0x61, 0x61 }
RIDL!{#[uuid(0x542612C4, 0xA1B8, 0x4632, 0xB5, 0x21, 0xDE, 0x11, 0xEA, 0x64, 0xA0, 0xB0)]
interface IMFTrustedInput(IMFTrustedInputVtbl): IUnknown(IUnknownVtbl) {
    fn GetInputTrustAuthority(
        dwStreamID: DWORD,
        riid: REFIID,
        ppunkObject: *mut *mut IUnknown,
    ) -> HRESULT,
}}
ENUM!{enum MFPOLICYMANAGER_ACTION {
    PEACTION_NO = 0,
    PEACTION_PLAY = 1,
    PEACTION_COPY = 2,
    PEACTION_EXPORT = 3,
    PEACTION_EXTRACT = 4,
    PEACTION_RESERVED1 = 5,
    PEACTION_RESERVED2 = 6,
    PEACTION_RESERVED3 = 7,
    PEACTION_LAST = 7,
}}
STRUCT!{struct MFINPUTTRUSTAUTHORITY_ACCESS_ACTION {
    Action: MFPOLICYMANAGER_ACTION,
    pbTicket: *mut BYTE,
    cbTicket: DWORD,
}}
STRUCT!{struct MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS {
    dwSize: DWORD,
    dwVer: DWORD,
    cbSignatureOffset: DWORD,
    cbSignatureSize: DWORD,
    cbExtensionOffset: DWORD,
    cbExtensionSize: DWORD,
    cActions: DWORD,
    rgOutputActions: [MFINPUTTRUSTAUTHORITY_ACCESS_ACTION; 1],
}}
DEFINE_GUID!{MF_MEDIA_PROTECTION_MANAGER_PROPERTIES,
    0x38bd81a9, 0xacea, 0x4c73, 0x89, 0xb2, 0x55, 0x32, 0xc0, 0xae, 0xca, 0x79}
RIDL!{#[uuid(0xd19f8e98, 0xb126, 0x4446, 0x89, 0x0c, 0x5d, 0xcb, 0x7a, 0xd7, 0x14, 0x53)]
interface IMFInputTrustAuthority(IMFInputTrustAuthorityVtbl): IUnknown(IUnknownVtbl) {
    fn GetDecrypter(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn RequestAccess(
        Action: MFPOLICYMANAGER_ACTION,
        ppContentEnablerActivate: *mut *mut IMFActivate,
    ) -> HRESULT,
    fn GetPolicy(
        Action: MFPOLICYMANAGER_ACTION,
        ppPolicy: *mut *mut IMFOutputPolicy,
    ) -> HRESULT,
    fn BindAccess(
        pParam: *mut MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS,
    ) -> HRESULT,
    fn UpdateAccess(
        pParam: *mut MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
}}
RIDL!{#[uuid(0xD19F8E95, 0xB126, 0x4446, 0x89, 0x0C, 0x5D, 0xCB, 0x7A, 0xD7, 0x14, 0x53)]
interface IMFTrustedOutput(IMFTrustedOutputVtbl): IUnknown(IUnknownVtbl) {
    fn GetOutputTrustAuthorityCount(
        pcOutputTrustAuthorities: *mut DWORD,
    ) -> HRESULT,
    fn GetOutputTrustAuthorityByIndex(
        dwIndex: DWORD,
        ppauthority: *mut *mut IMFOutputTrustAuthority,
    ) -> HRESULT,
    fn IsFinal(
        pfIsFinal: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xD19F8E94, 0xB126, 0x4446, 0x89, 0x0C, 0x5D, 0xCB, 0x7A, 0xD7, 0x14, 0x53)]
interface IMFOutputTrustAuthority(IMFOutputTrustAuthorityVtbl): IUnknown(IUnknownVtbl) {
    fn GetAction(
        pAction: *mut MFPOLICYMANAGER_ACTION,
    ) -> HRESULT,
    fn SetPolicy(
        ppPolicy: *mut *mut IMFOutputPolicy,
        nPolicy: DWORD,
        ppbTicket: *mut *mut BYTE,
        pcbTicket: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7F00F10A, 0xDAED, 0x41AF, 0xAB, 0x26, 0x5F, 0xDF, 0xA4, 0xDF, 0xBA, 0x3C)]
interface IMFOutputPolicy(IMFOutputPolicyVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GenerateRequiredSchemas(
        dwAttributes: DWORD,
        guidOutputSubType: GUID,
        rgGuidProtectionSchemasSupported: *mut GUID,
        cProtectionSchemasSupported: DWORD,
        ppRequiredProtectionSchemas: *mut *mut IMFCollection,
    ) -> HRESULT,
    fn GetOriginatorID(
        pguidOriginatorID: *mut GUID,
    ) -> HRESULT,
    fn GetMinimumGRLVersion(
        pdwMinimumGRLVersion: *mut DWORD,
    ) -> HRESULT,
}}
pub const MFOUTPUTATTRIBUTE_DIGITAL: DWORD = 0x0000_0001;
pub const MFOUTPUTATTRIBUTE_NONSTANDARDIMPLEMENTATION: DWORD = 0x0000_0002;
pub const MFOUTPUTATTRIBUTE_VIDEO: DWORD = 0x0000_0004;
pub const MFOUTPUTATTRIBUTE_COMPRESSED: DWORD = 0x0000_0008;
pub const MFOUTPUTATTRIBUTE_SOFTWARE: DWORD = 0x0000_0010;
pub const MFOUTPUTATTRIBUTE_BUS: DWORD = 0x0000_0020;
pub const MFOUTPUTATTRIBUTE_BUSIMPLEMENTATION: DWORD = 0x0000_FF00;
DEFINE_GUID!{MFCONNECTOR_SPDIF,
    0x0b94a712, 0xad3e, 0x4cee, 0x83, 0xce, 0xce, 0x32, 0xe3, 0xdb, 0x65, 0x22}
DEFINE_GUID!{MFCONNECTOR_UNKNOWN,
    0xac3aef5c, 0xce43, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_PCI,
     0xac3aef5d, 0xce43, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_PCIX,
    0xac3aef5e, 0xce43, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_PCI_Express,
    0xac3aef5f, 0xce43, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_AGP,
    0xac3aef60, 0xce43, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_VGA,
    0x57cd5968, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_SVIDEO,
    0x57cd5969, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_COMPOSITE,
    0x57cd596a, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_COMPONENT,
    0x57cd596b, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_DVI,
    0x57cd596c, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_HDMI,
    0x57cd596d, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_LVDS,
    0x57cd596e, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_D_JPN,
    0x57cd5970, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_SDI,
    0x57cd5971, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_DISPLAYPORT_EXTERNAL,
    0x57cd5972, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_DISPLAYPORT_EMBEDDED,
    0x57cd5973, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_UDI_EXTERNAL,
    0x57cd5974, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_UDI_EMBEDDED,
    0x57cd5975, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_MIRACAST,
    0x57cd5977, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_TRANSPORT_AGNOSTIC_DIGITAL_MODE_A,
    0x57cd5978, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
DEFINE_GUID!{MFCONNECTOR_TRANSPORT_AGNOSTIC_DIGITAL_MODE_B,
    0x57cd5979, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98}
RIDL!{#[uuid(0x7BE0FC5B, 0xABD9, 0x44FB, 0xA5, 0xC8, 0xF5, 0x01, 0x36, 0xE7, 0x15, 0x99)]
interface IMFOutputSchema(IMFOutputSchemaVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetSchemaType(
        pguidSchemaType: *mut GUID,
    ) -> HRESULT,
    fn GetConfigurationData(
        pdwVal: *mut DWORD,
    ) -> HRESULT,
    fn GetOriginatorID(
        pguidOriginatorID: *mut GUID,
    ) -> HRESULT,
}}
DEFINE_GUID!{MFPROTECTION_DISABLE,
    0x8cc6d81b, 0xfec6, 0x4d8f, 0x96, 0x4b, 0xcf, 0xba, 0x0b, 0x0d, 0xad, 0x0d}
DEFINE_GUID!{MFPROTECTION_CONSTRICTVIDEO,
    0x193370ce, 0xc5e4, 0x4c3a, 0x8a, 0x66, 0x69, 0x59, 0xb4, 0xda, 0x44, 0x42}
DEFINE_GUID!{MFPROTECTION_CONSTRICTVIDEO_NOOPM,
    0xa580e8cd, 0xc247, 0x4957, 0xb9, 0x83, 0x3c, 0x2e, 0xeb, 0xd1, 0xff, 0x59}
DEFINE_GUID!{MFPROTECTION_CONSTRICTAUDIO,
    0xffc99b44, 0xdf48, 0x4e16, 0x8e, 0x66, 0x09, 0x68, 0x92, 0xc1, 0x57, 0x8a}
DEFINE_GUID!{MFPROTECTION_TRUSTEDAUDIODRIVERS,
    0x65bdf3d2, 0x0168, 0x4816, 0xa5, 0x33, 0x55, 0xd4, 0x7b, 0x02, 0x71, 0x01}
DEFINE_GUID!{MFPROTECTION_HDCP,
    0xAE7CC03D, 0xC828, 0x4021, 0xac, 0xb7, 0xd5, 0x78, 0xd2, 0x7a, 0xaf, 0x13}
DEFINE_GUID!{MFPROTECTION_CGMSA,
    0xE57E69E9, 0x226B, 0x4d31, 0xB4, 0xE3, 0xD3, 0xDB, 0x00, 0x87, 0x36, 0xDD}
DEFINE_GUID!{MFPROTECTION_ACP,
    0xc3fd11c6, 0xf8b7, 0x4d20, 0xb0, 0x08, 0x1d, 0xb1, 0x7d, 0x61, 0xf2, 0xda}
DEFINE_GUID!{MFPROTECTION_WMDRMOTA,
    0xa267a6a1, 0x362e, 0x47d0, 0x88, 0x05, 0x46, 0x28, 0x59, 0x8a, 0x23, 0xe4}
DEFINE_GUID!{MFPROTECTION_FFT,
    0x462a56b2, 0x2866, 0x4bb6, 0x98, 0x0d, 0x6d, 0x8d, 0x9e, 0xdb, 0x1a, 0x8c}
DEFINE_GUID!{MFPROTECTION_PROTECTED_SURFACE,
    0x4f5d9566, 0xe742, 0x4a25, 0x8d, 0x1f, 0xd2, 0x87, 0xb5, 0xfa, 0x0a, 0xde}
DEFINE_GUID!{MFPROTECTION_DISABLE_SCREEN_SCRAPE,
    0xa21179a4, 0xb7cd, 0x40d8, 0x96, 0x14, 0x8e, 0xf2, 0x37, 0x1b, 0xa7, 0x8d}
DEFINE_GUID!{MFPROTECTION_VIDEO_FRAMES,
    0x36a59cbc, 0x7401, 0x4a8c, 0xbc, 0x20, 0x46, 0xa7, 0xc9, 0xe5, 0x97, 0xf0}
DEFINE_GUID!{MFPROTECTION_HARDWARE,
    0x4ee7f0c1, 0x9ed7, 0x424f, 0xb6, 0xbe, 0x99, 0x6b, 0x33, 0x52, 0x88, 0x56}
DEFINE_GUID!{MFPROTECTION_HDCP_WITH_TYPE_ENFORCEMENT,
    0xa4a585e8, 0xed60, 0x442d, 0x81, 0x4d, 0xdb, 0x4d, 0x42, 0x20, 0xa0, 0x6d}
ENUM!{enum MF_OPM_CGMSA_PROTECTION_LEVEL {
    MF_OPM_CGMSA_OFF = 0,
    MF_OPM_CGMSA_COPY_FREELY = 0x1,
    MF_OPM_CGMSA_COPY_NO_MORE = 0x2,
    MF_OPM_CGMSA_COPY_ONE_GENERATION = 0x3,
    MF_OPM_CGMSA_COPY_NEVER = 0x4,
    MF_OPM_CGMSA_REDISTRIBUTION_CONTROL_REQUIRED = 0x8,
}}
ENUM!{enum MF_OPM_ACP_PROTECTION_LEVEL {
    MF_OPM_ACP_OFF,
    MF_OPM_ACP_LEVEL_ONE,
    MF_OPM_ACP_LEVEL_TWO,
    MF_OPM_ACP_LEVEL_THREE,
}}
DEFINE_GUID!{MFPROTECTIONATTRIBUTE_BEST_EFFORT,
    0xc8e06331, 0x75f0, 0x4ec1, 0x8e, 0x77, 0x17, 0x57, 0x8f, 0x77, 0x3b, 0x46}
DEFINE_GUID!{MFPROTECTIONATTRIBUTE_FAIL_OVER,
    0x8536abc5, 0x38f1, 0x4151, 0x9c, 0xce, 0xf5, 0x5d, 0x94, 0x12, 0x29, 0xac}
DEFINE_GUID!{MFPROTECTION_GRAPHICS_TRANSFER_AES_ENCRYPTION,
    0xc873de64, 0xd8a5, 0x49e6, 0x88, 0xbb, 0xfb, 0x96, 0x3f, 0xd3, 0xd4, 0xce}
DEFINE_GUID!{MFPROTECTIONATTRIBUTE_CONSTRICTVIDEO_IMAGESIZE,
    0x008476fc, 0x4b58, 0x4d80, 0xa7, 0x90, 0xe7, 0x29, 0x76, 0x73, 0x16, 0x1d}
DEFINE_GUID!{MFPROTECTIONATTRIBUTE_HDCP_SRM,
    0x6f302107, 0x3477, 0x4468, 0x8a, 0x08, 0xee, 0xf9, 0xdb, 0x10, 0xe2, 0x0f}
#[inline]
pub fn MAKE_MFPROTECTIONDATA_DISABLE(Disable: bool) -> DWORD {
    if Disable { 0x1 } else { 0 }
}
#[inline]
pub fn EXTRACT_MFPROTECTIONDATA_DISABLE_ON(Data: DWORD) -> bool {
    (Data & 0x1) != 0
}
#[inline]
pub fn EXTRACT_MFPROTECTIONDATA_DISABLE_RESERVED(Data: DWORD) -> DWORD {
    (Data & 0xFFFFFFFE) >> 1
}
#[inline]
pub fn MAKE_MFPROTECTIONDATA_CONSTRICTAUDIO(Level: DWORD) -> DWORD {
    Level
}
#[inline]
pub fn EXTRACT_MFPROTECTIONDATA_CONSTRICTAUDIO_LEVEL(Data: DWORD) -> DWORD {
    Data & 0xFF
}
#[inline]
pub fn EXTRACT_MFPROTECTIONDATA_CONSTRICTAUDIO_RESERVED(Data: DWORD) -> DWORD {
    (Data & 0xFFFF_FF00) >> 8
}
ENUM!{enum MFAudioConstriction {
    MFaudioConstrictionOff,
    MFaudioConstriction48_16,
    MFaudioConstriction44_16,
    MFaudioConstriction14_14,
    MFaudioConstrictionMute,
}}
#[inline]
pub fn MAKE_MFPROTECTIONDATA_TRUSTEDAUDIODRIVERS(
    TestCertificateEnable: bool,
    DigitalOutputDisable: bool,
    DrmLevel: DWORD,
) -> DWORD {
    let mut result = DrmLevel;
    if TestCertificateEnable { result |= 0x2_0000; }
    if DigitalOutputDisable { result |= 0x1_0000; }
    result
}
#[inline]
pub fn MAKE_MFPROTECTIONDATA_TRUSTEDAUDIODRIVERS2(
    TestCertificateEnable: bool,
    DigitalOutputDisable: bool,
    CopyOK: bool,
    DrmLevel: DWORD,
) -> DWORD {
    let mut result = DrmLevel;
    if TestCertificateEnable { result |= 0x2_0000; }
    if DigitalOutputDisable { result |= 0x1_0000; }
    if CopyOK { result |= 0x4_0000; }
    result
}
#[inline]
pub fn EXTRACT_MFPROTECTIONDATA_TRUSTEDAUDIODRIVERS_DRMLEVEL(Data: DWORD) -> DWORD {
    Data & 0x0000_FFFF
}
#[inline]
pub fn EXTRACT_MFPROTECTIONDATA_TRUSTEDAUDIODRIVERS_DIGITALOUTPUTDISABLE(Data: DWORD) -> bool {
    (Data & 0x0001_0000) != 0
}
#[inline]
pub fn EXTRACT_MFPROTECTIONDATA_TRUSTEDAUDIODRIVERS_TESTCERTIFICATEENABLE(Data: DWORD) -> bool {
    (Data & 0x2_0000) != 0
}
#[inline]
pub fn EXTRACT_MFPROTECTIONDATA_TRUSTEDAUDIODRIVERS_COPYOK(Data: DWORD) -> bool {
    (Data & 0x4_0000) != 0
}
#[inline]
pub fn EXTRACT_MFPROTECTIONDATA_TRUSTEDAUDIODRIVERS_RESERVED(Data: DWORD) -> DWORD {
    (Data & 0xFFF8_0000) >> 19
}
RIDL!{#[uuid(0xd0ae555d, 0x3b12, 0x4d97, 0xb0, 0x60, 0x09, 0x90, 0xbc, 0x5a, 0xeb, 0x67)]
interface IMFSecureChannel(IMFSecureChannelVtbl): IUnknown(IUnknownVtbl) {
    fn GetCertificate(
        ppCert: *mut *mut BYTE,
        pcbCert: *mut DWORD,
    ) -> HRESULT,
    fn SetupSession(
        pbEncryptedSessionKey: *mut BYTE,
        cbSessionKey: DWORD,
    ) -> HRESULT,
}}
ENUM!{enum SAMPLE_PROTECTION_VERSION {
    SAMPLE_PROTECTION_VERSION_NO,
    SAMPLE_PROTECTION_VERSION_BASIC_LOKI,
    SAMPLE_PROTECTION_VERSION_SCATTER,
    SAMPLE_PROTECTION_VERSION_RC4,
    SAMPLE_PROTECTION_VERSION_AES128CTR,
}}
DEFINE_GUID!{MF_SampleProtectionSalt,
    0x5403deee, 0xb9ee, 0x438f, 0xaa, 0x83, 0x38, 0x04, 0x99, 0x7e, 0x56, 0x9d}
RIDL!{#[uuid(0x8e36395f, 0xc7b9, 0x43c4, 0xa5, 0x4d, 0x51, 0x2b, 0x4a, 0xf6, 0x3c, 0x95)]
interface IMFSampleProtection(IMFSampleProtectionVtbl): IUnknown(IUnknownVtbl) {
    fn GetInputProtectionVersion(
        pdwVersion: *mut DWORD,
    ) -> HRESULT,
    fn GetOutputProtectionVersion(
        pdwVersion: *mut DWORD,
    ) -> HRESULT,
    fn GetProtectionCertificate(
        dwVersion: DWORD,
        ppCert: *mut *mut BYTE,
        pcbCert: *mut DWORD,
    ) -> HRESULT,
    fn InitOutputProtection(
        dwVersion: DWORD,
        dwOutputId: DWORD,
        pbCert: *mut BYTE,
        cbCert: DWORD,
        ppbSeed: *mut *mut BYTE,
        pcbSeed: *mut DWORD,
    ) -> HRESULT,
    fn InitInputProtection(
        dwVersion: DWORD,
        dwInputId: DWORD,
        pbSeed: *mut BYTE,
        cbSeed: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x5dfd4b2a, 0x7674, 0x4110, 0xa4, 0xe6, 0x8a, 0x68, 0xfd, 0x5f, 0x36, 0x88)]
interface IMFMediaSinkPreroll(IMFMediaSinkPrerollVtbl): IUnknown(IUnknownVtbl) {
    fn NotifyPreroll(
        hnsUpcomingStartTime: MFTIME,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xEAECB74A, 0x9A50, 0x42ce, 0x95, 0x41, 0x6A, 0x7F, 0x57, 0xAA, 0x4A, 0xD7)]
interface IMFFinalizableMediaSink(IMFFinalizableMediaSinkVtbl): IMFMediaSink(IMFMediaSinkVtbl) {
    fn BeginFinalize(
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndFinalize(
        pResult: *mut IMFAsyncResult,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9db7aa41, 0x3cc5, 0x40d4, 0x85, 0x09, 0x55, 0x58, 0x04, 0xad, 0x34, 0xcc)]
interface IMFStreamingSinkConfig(IMFStreamingSinkConfigVtbl): IUnknown(IUnknownVtbl) {
    fn StartStreaming(
        fSeekOffsetIsByteOffset: BOOL,
        qwSeekOffset: QWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x994e23ad, 0x1cc2, 0x493c, 0xb9, 0xfa, 0x46, 0xf1, 0xcb, 0x04, 0x0f, 0xa4)]
interface IMFRemoteProxy(IMFRemoteProxyVtbl): IUnknown(IUnknownVtbl) {
    fn GetRemoteObject(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetRemoteHost(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_REMOTE_PROXY,
    0x2f00c90e, 0xd2cf, 0x4278, 0x8b, 0x6a, 0xd0, 0x77, 0xfa, 0xc3, 0xa2, 0x5f}
RIDL!{#[uuid(0x09EF5BE3, 0xC8A7, 0x469e, 0x8B, 0x70, 0x73, 0xBF, 0x25, 0xBB, 0x19, 0x3F)]
interface IMFObjectReferenceStream(IMFObjectReferenceStreamVtbl): IUnknown(IUnknownVtbl) {
    fn SaveReference(
        riid: REFIID,
        pUnk: *mut IUnknown,
    ) -> HRESULT,
    fn LoadReference(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_CreateMediaExtensionObject,
    0xef65a54d, 0x0788, 0x45b8, 0x8b, 0x14, 0xbc, 0x0f, 0x6a, 0x6b, 0x51, 0x37}
RIDL!{#[uuid(0xF70CA1A9, 0xFDC7, 0x4782, 0xB9, 0x94, 0xAD, 0xFF, 0xB1, 0xC9, 0x86, 0x06)]
interface IMFPMPHost(IMFPMPHostVtbl): IUnknown(IUnknownVtbl) {
    fn LockProcess() -> HRESULT,
    fn UnlockProcess() -> HRESULT,
    fn CreateObjectByCLSID(
        clsid: REFCLSID,
        pStream: *mut IStream,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6C4E655D, 0xEAD8, 0x4421, 0xB6, 0xB9, 0x54, 0xDC, 0xDB, 0xBD, 0xF8, 0x20)]
interface IMFPMPClient(IMFPMPClientVtbl): IUnknown(IUnknownVtbl) {
    fn SetPMPHost(
        pPMPHost: *mut IMFPMPHost,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x994e23af, 0x1cc2, 0x493c, 0xb9, 0xfa, 0x46, 0xf1, 0xcb, 0x04, 0x0f, 0xa4)]
interface IMFPMPServer(IMFPMPServerVtbl): IUnknown(IUnknownVtbl) {
    fn LockProcess() -> HRESULT,
    fn UnlockProcess() -> HRESULT,
    fn CreateObjectByCLSID(
        clsid: REFCLSID,
        riid: REFIID,
        ppObject: *mut *mut c_void,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreatePMPServer(
        dwCreationFlags: DWORD,
        ppPMPServer: *mut *mut IMFPMPServer,
    ) -> HRESULT;
}
RIDL!{#[uuid(0x1cde6309, 0xcae0, 0x4940, 0x90, 0x7e, 0xc1, 0xec, 0x9c, 0x3d, 0x1d, 0x4a)]
interface IMFRemoteDesktopPlugin(IMFRemoteDesktopPluginVtbl): IUnknown(IUnknownVtbl) {
    fn UpdateTopology(
        pTopology: *mut IMFTopology,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateRemoteDesktopPlugin(
        ppPlugin: *mut *mut IMFRemoteDesktopPlugin,
    ) -> HRESULT;
    pub fn CreateNamedPropertyStore(
        ppStore: *mut *mut INamedPropertyStore,
    ) -> HRESULT;
}
RIDL!{#[uuid(0xA7E025DD, 0x5303, 0x4a62, 0x89, 0xD6, 0xE7, 0x47, 0xE1, 0xEF, 0xAC, 0x73)]
interface IMFSAMIStyle(IMFSAMIStyleVtbl): IUnknown(IUnknownVtbl) {
    fn GetStyleCount(
        pdwCount: *mut DWORD,
    ) -> HRESULT,
    fn GetStyles(
        pPropVarStyleArray: *mut PROPVARIANT,
    ) -> HRESULT,
    fn SetSelectedStyle(
        pwszStyle: LPCWSTR,
    ) -> HRESULT,
    fn GetSelectedStyle(
        ppwszStyle: *mut LPWSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_SAMI_SERVICE,
    0x49a89ae7, 0xb4d9, 0x4ef2, 0xaa, 0x5c, 0xf6, 0x5a, 0x3e, 0x05, 0xae, 0x4e}
DEFINE_GUID!{MF_PD_SAMI_STYLELIST,
    0xe0b73c7f, 0x486d, 0x484e, 0x98, 0x72, 0x4d, 0xe5, 0x19, 0x2a, 0x7b, 0xf8}
DEFINE_GUID!{MF_SD_SAMI_LANGUAGE,
    0x36fcb98a, 0x6cd0, 0x44cb, 0xac, 0xb9, 0xa8, 0xf5, 0x60, 0x0d, 0xd0, 0xbb}
RIDL!{#[uuid(0x4ADFDBA3, 0x7AB0, 0x4953, 0xA6, 0x2B, 0x46, 0x1E, 0x7F, 0xF3, 0xDA, 0x1E)]
interface IMFTranscodeProfile(IMFTranscodeProfileVtbl): IUnknown(IUnknownVtbl) {
    fn SetAudioAttributes(
        pAttrs: *mut IMFAttributes,
    ) -> HRESULT,
    fn GetAudioAttributes(
        ppAttrs: *mut *mut IMFAttributes,
    ) -> HRESULT,
    fn SetVideoAttributes(
        pAttrs: *mut IMFAttributes,
    ) -> HRESULT,
    fn GetVideoAttributes(
        ppAttrs: *mut *mut IMFAttributes,
    ) -> HRESULT,
    fn SetContainerAttributes(
        pAttrs: *mut IMFAttributes,
    ) -> HRESULT,
    fn GetContainerAttributes(
        ppAttrs: *mut *mut IMFAttributes,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_TRANSCODE_CONTAINERTYPE,
    0x150ff23f, 0x4abc, 0x478b, 0xac, 0x4f, 0xe1, 0x91, 0x6f, 0xba, 0x1c, 0xca}
DEFINE_GUID!{MFTranscodeContainerType_ASF,
    0x430f6f6e, 0xb6bf, 0x4fc1, 0xa0, 0xbd, 0x9e, 0xe4, 0x6e, 0xee, 0x2a, 0xfb}
DEFINE_GUID!{MFTranscodeContainerType_MPEG4,
    0xdc6cd05d, 0xb9d0, 0x40ef, 0xbd, 0x35, 0xfa, 0x62, 0x2c, 0x1a, 0xb2, 0x8a}
DEFINE_GUID!{MFTranscodeContainerType_MP3,
    0xe438b912, 0x83f1, 0x4de6, 0x9e, 0x3a, 0x9f, 0xfb, 0xc6, 0xdd, 0x24, 0xd1}
DEFINE_GUID!{MFTranscodeContainerType_FLAC,
    0x31344aa3, 0x05a9, 0x42b5, 0x90, 0x1b, 0x8e, 0x9d, 0x42, 0x57, 0xf7, 0x5e}
DEFINE_GUID!{MFTranscodeContainerType_3GP,
    0x34c50167, 0x4472, 0x4f34, 0x9e, 0xa0, 0xc4, 0x9f, 0xba, 0xcf, 0x03, 0x7d}
DEFINE_GUID!{MFTranscodeContainerType_AC3,
    0x6d8d91c3, 0x8c91, 0x4ed1, 0x87, 0x42, 0x8c, 0x34, 0x7d, 0x5b, 0x44, 0xd0}
DEFINE_GUID!{MFTranscodeContainerType_ADTS,
    0x132fd27d, 0x0f02, 0x43de, 0xa3, 0x01, 0x38, 0xfb, 0xbb, 0xb3, 0x83, 0x4e}
DEFINE_GUID!{MFTranscodeContainerType_MPEG2,
    0xbfc2dbf9, 0x7bb4, 0x4f8f, 0xaf, 0xde, 0xe1, 0x12, 0xc4, 0x4b, 0xa8, 0x82}
DEFINE_GUID!{MFTranscodeContainerType_WAVE,
    0x64c3453c, 0x0f26, 0x4741, 0xbe, 0x63, 0x87, 0xbd, 0xf8, 0xbb, 0x93, 0x5b}
DEFINE_GUID!{MFTranscodeContainerType_AVI,
    0x7edfe8af, 0x402f, 0x4d76, 0xa3, 0x3c, 0x61, 0x9f, 0xd1, 0x57, 0xd0, 0xf1}
DEFINE_GUID!{MFTranscodeContainerType_FMPEG4,
    0x9ba876f1, 0x419f, 0x4b77, 0xa1, 0xe0, 0x35, 0x95, 0x9d, 0x9d, 0x40, 0x04}
DEFINE_GUID!{MFTranscodeContainerType_AMR,
    0x025d5ad3, 0x621a, 0x475b, 0x96, 0x4d, 0x66, 0xb1, 0xc8, 0x24, 0xf0, 0x79}
DEFINE_GUID!{MF_TRANSCODE_SKIP_METADATA_TRANSFER,
    0x4e4469ef, 0xb571, 0x4959, 0x8f, 0x83, 0x3d, 0xcf, 0xba, 0x33, 0xa3, 0x93}
DEFINE_GUID!{MF_TRANSCODE_TOPOLOGYMODE,
    0x3e3df610, 0x394a, 0x40b2, 0x9d, 0xea, 0x3b, 0xab, 0x65, 0x0b, 0xeb, 0xf2}
ENUM!{enum MF_TRANSCODE_TOPOLOGYMODE_FLAGS {
    MF_TRANSCODE_TOPOLOGYMODE_SOFTWARE_ONLY,
    MF_TRANSCODE_TOPOLOGYMODE_HARDWARE_ALLOWED,
}}
DEFINE_GUID!{MF_TRANSCODE_ADJUST_PROFILE,
    0x9c37c21b, 0x060f, 0x487c, 0xa6, 0x90, 0x80, 0xd7, 0xf5, 0x0d, 0x1c, 0x72}
ENUM!{enum MF_TRANSCODE_ADJUST_PROFILE_FLAGS {
    MF_TRANSCODE_ADJUST_PROFILE_DEFAULT,
    MF_TRANSCODE_ADJUST_PROFILE_USE_SOURCE_ATTRIBUTES,
}}
DEFINE_GUID!{MF_TRANSCODE_ENCODINGPROFILE,
    0x6947787c, 0xf508, 0x4ea9, 0xb1, 0xe9, 0xa1, 0xfe, 0x3a, 0x49, 0xfb, 0xc9}
DEFINE_GUID!{MF_TRANSCODE_QUALITYVSSPEED,
    0x98332df8, 0x03cd, 0x476b, 0x89, 0xfa, 0x3f, 0x9e, 0x44, 0x2d, 0xec, 0x9f}
DEFINE_GUID!{MF_TRANSCODE_DONOT_INSERT_ENCODER,
    0xf45aa7ce, 0xab24, 0x4012, 0xa1, 0x1b, 0xdc, 0x82, 0x20, 0x20, 0x14, 0x10}
ENUM!{enum MF_VIDEO_PROCESSOR_ALGORITHM_TYPE {
    MF_VIDEO_PROCESSOR_ALGORITHM_DEFAULT,
    MF_VIDEO_PROCESSOR_ALGORITHM_MRF_CRF_444,
}}
DEFINE_GUID!{MF_VIDEO_PROCESSOR_ALGORITHM,
    0x4a0a1e1f, 0x272c, 0x4fb6, 0x9e, 0xb1, 0xdb, 0x33, 0x0c, 0xbc, 0x97, 0xca}
DEFINE_GUID!{MF_XVP_DISABLE_FRC,
    0x2c0afa19, 0x7a97, 0x4d5a, 0x9e, 0xe8, 0x16, 0xd4, 0xfc, 0x51, 0x8d, 0x8c}
DEFINE_GUID!{MF_XVP_CALLER_ALLOCATES_OUTPUT,
    0x04a2cabc, 0x0cab, 0x40b1, 0xa1, 0xb9, 0x75, 0xbc, 0x36, 0x58, 0xf0, 0x00}
extern "system" {
    pub fn MFCreateTranscodeProfile(
        ppTranscodeProfile: *mut *mut IMFTranscodeProfile,
    ) -> HRESULT;
    pub fn MFCreateTranscodeTopology(
        pSrc: *mut IMFMediaSource,
        pwszOutputFilePath: LPCWSTR,
        pProfile: *mut IMFTranscodeProfile,
        ppTranscodeTopo: *mut *mut IMFTopology,
    ) -> HRESULT;
    pub fn MFCreateTranscodeTopologyFromByteStream(
        pSrc: *mut IMFMediaSource,
        pOutputStream: *mut IMFByteStream,
        pProfile: *mut IMFTranscodeProfile,
        ppTranscodeTopo: *mut *mut IMFTopology,
    ) -> HRESULT;
    pub fn MFTranscodeGetAudioOutputAvailableTypes(
        guidSubType: REFGUID,
        dwMFTFlags: DWORD,
        pCodecConfig: *mut IMFAttributes,
        ppAvailableTypes: *mut *mut IMFCollection,
    ) -> HRESULT;
}
STRUCT!{struct MF_TRANSCODE_SINK_INFO {
    dwVideoStreamID: DWORD,
    pVideoMediaType: *mut IMFMediaType,
    dwAudioStreamID: DWORD,
    pAudioMediaType: *mut IMFMediaType,
}}
RIDL!{#[uuid(0x8CFFCD2E, 0x5A03, 0x4a3a, 0xAF, 0xF7, 0xED, 0xCD, 0x10, 0x7C, 0x62, 0x0E)]
interface IMFTranscodeSinkInfoProvider(IMFTranscodeSinkInfoProviderVtbl): IUnknown(IUnknownVtbl) {
    fn SetOutputFile(
        pwszFileName: LPCWSTR,
    ) -> HRESULT,
    fn SetOutputByteStream(
        pByteStreamActivate: *mut IMFActivate,
    ) -> HRESULT,
    fn SetProfile(
        pProfile: *mut IMFTranscodeProfile,
    ) -> HRESULT,
    fn GetSinkInfo(
        pSinkInfo: *mut MF_TRANSCODE_SINK_INFO,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateTranscodeSinkActivate(
        ppActivate: *mut *mut IMFActivate,
    ) -> HRESULT;
}
RIDL!{#[uuid(0x508E71D3, 0xEC66, 0x4fc3, 0x87, 0x75, 0xB4, 0xB9, 0xED, 0x6B, 0xA8, 0x47)]
interface IMFFieldOfUseMFTUnlock(IMFFieldOfUseMFTUnlockVtbl): IUnknown(IUnknownVtbl) {
    fn Unlock(
        pUnkMFT: *mut IUnknown,
    ) -> HRESULT,
}}
STRUCT!{struct MFT_REGISTRATION_INFO {
    clsid: CLSID,
    guidCategory: GUID,
    uiFlags: UINT32,
    pszName: LPCWSTR,
    cInTypes: DWORD,
    pInTypes: *mut MFT_REGISTER_TYPE_INFO,
    cOutTypes: DWORD,
    pOutTypes: *mut MFT_REGISTER_TYPE_INFO,
}}
DEFINE_GUID!{MF_LOCAL_MFT_REGISTRATION_SERVICE,
    0xddf5cf9c, 0x4506, 0x45aa, 0xab, 0xf0, 0x6d, 0x5d, 0x94, 0xdd, 0x1b, 0x4a}
RIDL!{#[uuid(0x149c4d73, 0xb4be, 0x4f8d, 0x8b, 0x87, 0x07, 0x9e, 0x92, 0x6b, 0x6a, 0xdd)]
interface IMFLocalMFTRegistration(IMFLocalMFTRegistrationVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterMFTs(
        pMFTs: *mut MFT_REGISTRATION_INFO,
        cMFTs: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x19f68549, 0xca8a, 0x4706, 0xa4, 0xef, 0x48, 0x1d, 0xbc, 0x95, 0xe1, 0x2c)]
interface IMFCapturePhotoConfirmation(IMFCapturePhotoConfirmationVtbl): IUnknown(IUnknownVtbl) {
    fn SetPhotoConfirmationCallback(
        pNotificationCallback: *mut IMFAsyncCallback,
    ) -> HRESULT,
    fn SetPixelFormat(
        subtype: GUID,
    ) -> HRESULT,
    fn GetPixelFormat(
        subtype: *mut GUID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x84d2054a, 0x3aa1, 0x4728, 0xa3, 0xb0, 0x44, 0x0a, 0x41, 0x8c, 0xf4, 0x9c)]
interface IMFPMPHostApp(IMFPMPHostAppVtbl): IUnknown(IUnknownVtbl) {
    fn LockProcess() -> HRESULT,
    fn UnlockProcess() -> HRESULT,
    fn ActivateClassById(
        id: LPCWSTR,
        pStream: *mut IStream,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc004f646, 0xbe2c, 0x48f3, 0x93, 0xa2, 0xa0, 0x98, 0x3e, 0xba, 0x11, 0x08)]
interface IMFPMPClientApp(IMFPMPClientAppVtbl): IUnknown(IUnknownVtbl) {
    fn SetPMPHost(
        pPMPHost: *mut IMFPMPHostApp,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x380b9af9, 0xa85b, 0x4e78, 0xa2, 0xaf, 0xea, 0x5c, 0xe6, 0x45, 0xc6, 0xb4)]
interface IMFMediaStreamSourceSampleRequest(IMFMediaStreamSourceSampleRequestVtbl):
  IUnknown(IUnknownVtbl) {
    fn SetSample(
        value: *mut IMFSample,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x245BF8E9, 0x0755, 0x40f7, 0x88, 0xA5, 0xAE, 0x0F, 0x18, 0xD5, 0x5E, 0x17)]
interface IMFTrackedSample(IMFTrackedSampleVtbl): IUnknown(IUnknownVtbl) {
    fn SetAllocator(
        pSampleAllocator: *mut IMFAsyncCallback,
        pUnkState: *mut IUnknown,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateTrackedSample(
        ppMFSample: *mut *mut IMFTrackedSample,
    ) -> HRESULT;
    pub fn MFCreateMFByteStreamOnStream(
        pStream: *mut IStream,
        ppByteStream: *mut *mut IMFByteStream,
    ) -> HRESULT;
    pub fn MFCreateStreamOnMFByteStream(
        pByteStream: *mut IMFByteStream,
        ppStream: *mut *mut IStream,
    ) -> HRESULT;
    pub fn MFCreateMFByteStreamOnStreamEx(
        punkStream: *mut IUnknown,
        ppByteStream: *mut *mut IMFByteStream,
    ) -> HRESULT;
    pub fn MFCreateStreamOnMFByteStreamEx(
        pByteStream: *mut IMFByteStream,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn MFCreateMediaTypeFromProperties(
        punkStream: *mut IUnknown,
        ppMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT;
    pub fn MFCreatePropertiesFromMediaType(
        pMediaType: *mut IMFMediaType,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
}
DEFINE_GUID!{MF_WRAPPED_BUFFER_SERVICE,
    0xab544072, 0xc269, 0x4ebc, 0xa5, 0x52, 0x1c, 0x3b, 0x32, 0xbe, 0xd5, 0xca}
DEFINE_GUID!{MF_WRAPPED_SAMPLE_SERVICE,
    0x31f52bf2, 0xd03e, 0x4048, 0x80, 0xd0, 0x9c, 0x10, 0x46, 0xd8, 0x7c, 0x61}
DEFINE_GUID!{MF_WRAPPED_OBJECT,
    0x2b182c4c, 0xd6ac, 0x49f4, 0x89, 0x15, 0xf7, 0x18, 0x87, 0xdb, 0x70, 0xcd}
DEFINE_GUID!{CLSID_HttpSchemePlugin,
    0x44cb442b, 0x9da9, 0x49df, 0xb3, 0xfd, 0x02, 0x37, 0x77, 0xb1, 0x6e, 0x50}
DEFINE_GUID!{CLSID_UrlmonSchemePlugin,
    0x9ec4b4f9, 0x3029, 0x45ad, 0x94, 0x7b, 0x34, 0x4d, 0xe2, 0xa2, 0x49, 0xe2}
DEFINE_GUID!{CLSID_NetSchemePlugin,
    0xe9f4ebab, 0xd97b, 0x463e, 0xa2, 0xb1, 0xc5, 0x4e, 0xe3, 0xf9, 0x41, 0x4d}
extern "system" {
    pub fn MFEnumDeviceSources(
        pAttributes: *mut IMFAttributes,
        pppSourceActivate: *mut *mut *mut IMFActivate,
        pcSourceActivate: *mut UINT32,
    ) -> HRESULT;
    pub fn MFCreateDeviceSource(
        pAttributes: *mut IMFAttributes,
        ppSource: *mut *mut IMFMediaSource,
    ) -> HRESULT;
    pub fn MFCreateDeviceSourceActivate(
        pAttributes: *mut IMFAttributes,
        ppActivate: *mut *mut IMFActivate,
    ) -> HRESULT;
}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE,
    0xc60ac5fe, 0x252a, 0x478f, 0xa0, 0xef, 0xbc, 0x8f, 0xa5, 0xf7, 0xca, 0xd3}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_HW_SOURCE,
    0xde7046ba, 0x54d6, 0x4487, 0xa2, 0xa4, 0xec, 0x7c, 0x0d, 0x1b, 0xd1, 0x63}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_FRIENDLY_NAME,
    0x60d0e559, 0x52f8, 0x4fa2, 0xbb, 0xce, 0xac, 0xdb, 0x34, 0xa8, 0xec, 0x01}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_MEDIA_TYPE,
    0x56a819ca, 0x0c78, 0x4de4, 0xa0, 0xa7, 0x3d, 0xda, 0xba, 0x0f, 0x24, 0xd4}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_CATEGORY,
    0x77f0ae69, 0xc3bd, 0x4509, 0x94, 0x1d, 0x46, 0x7e, 0x4d, 0x24, 0x89, 0x9e}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_SYMBOLIC_LINK,
    0x58f0aad8, 0x22bf, 0x4f8a, 0xbb, 0x3d, 0xd2, 0xc4, 0x97, 0x8c, 0x6e, 0x2f}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_SYMBOLIC_LINK,
    0x98d24b5e, 0x5930, 0x4614, 0xb5, 0xa1, 0xf6, 0x00, 0xf9, 0x35, 0x5a, 0x78}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_MAX_BUFFERS,
    0x7dd9b730, 0x4f2d, 0x41d5, 0x8f, 0x95, 0x0c, 0xc9, 0xa9, 0x12, 0xba, 0x26}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_ENDPOINT_ID,
    0x30da9258, 0xfeb9, 0x47a7, 0xa4, 0x53, 0x76, 0x3a, 0x7a, 0x8e, 0x1c, 0x5f}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_ROLE,
    0xbc9d118e, 0x8c67, 0x4a18, 0x85, 0xd4, 0x12, 0xd3, 0x00, 0x40, 0x05, 0x52}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_PROVIDER_DEVICE_ID,
    0x36689d42, 0xa06c, 0x40ae, 0x84, 0xcf, 0xf5, 0xa0, 0x34, 0x06, 0x7c, 0xc4}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_GUID,
    0x14dd9a1c, 0x7cff, 0x41be, 0xb1, 0xb9, 0xba, 0x1a, 0xc6, 0xec, 0xb5, 0x71}
DEFINE_GUID!{MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID,
    0x8ac3587a, 0x4ae7, 0x42d8, 0x99, 0xe0, 0x0a, 0x60, 0x13, 0xee, 0xf9, 0x0f}
DEFINE_GUID!{MF_DEVICESTREAM_IMAGE_STREAM,
    0xa7ffb865, 0xe7b2, 0x43b0, 0x9f, 0x6f, 0x9a, 0xf2, 0xa0, 0xe5, 0x0f, 0xc0}
DEFINE_GUID!{MF_DEVICESTREAM_INDEPENDENT_IMAGE_STREAM,
    0x03eeec7e, 0xd605, 0x4576, 0x8b, 0x29, 0x65, 0x80, 0xb4, 0x90, 0xd7, 0xd3}
DEFINE_GUID!{MF_DEVICESTREAM_STREAM_ID,
    0x11bd5120, 0xd124, 0x446b, 0x88, 0xe6, 0x17, 0x06, 0x02, 0x57, 0xff, 0xf9}
DEFINE_GUID!{MF_DEVICESTREAM_STREAM_CATEGORY,
    0x2939e7b8, 0xa62e, 0x4579, 0xb6, 0x74, 0xd4, 0x07, 0x3d, 0xfa, 0xbb, 0xba}
DEFINE_GUID!{MF_DEVICESTREAM_FRAMESERVER_SHARED,
    0x1CB378E9, 0xB279, 0x41D4, 0xAF, 0x97, 0x34, 0xA2, 0x43, 0xE6, 0x83, 0x20}
DEFINE_GUID!{MF_DEVICESTREAM_TRANSFORM_STREAM_ID,
    0xe63937b7, 0xdaaf, 0x4d49, 0x81, 0x5f, 0xd8, 0x26, 0xf8, 0xad, 0x31, 0xe7}
DEFINE_GUID!{MF_DEVICESTREAM_EXTENSION_PLUGIN_CLSID,
    0x048e6558, 0x60c4, 0x4173, 0xbd, 0x5b, 0x6a, 0x3c, 0xa2, 0x89, 0x6a, 0xee}
DEFINE_GUID!{MF_DEVICEMFT_EXTENSION_PLUGIN_CLSID,
    0x0844dbae, 0x34fa, 0x48a0, 0xa7, 0x83, 0x8e, 0x69, 0x6f, 0xb1, 0xc9, 0xa8}
DEFINE_GUID!{MF_DEVICESTREAM_EXTENSION_PLUGIN_CONNECTION_POINT,
    0x37f9375c, 0xe664, 0x4ea4, 0xaa, 0xe4, 0xcb, 0x6d, 0x1d, 0xac, 0xa1, 0xf4}
DEFINE_GUID!{MF_DEVICESTREAM_TAKEPHOTO_TRIGGER,
    0x1d180e34, 0x538c, 0x4fbb, 0xa7, 0x5a, 0x85, 0x9a, 0xf7, 0xd2, 0x61, 0xa6}
DEFINE_GUID!{MF_DEVICESTREAM_MAX_FRAME_BUFFERS,
    0x1684cebe, 0x3175, 0x4985, 0x88, 0x2c, 0x0e, 0xfd, 0x3e, 0x8a, 0xc1, 0x1e}
DEFINE_GUID!{MF_DEVICEMFT_CONNECTED_FILTER_KSCONTROL,
    0x6a2c4fa6, 0xd179, 0x41cd, 0x95, 0x23, 0x82, 0x23, 0x71, 0xea, 0x40, 0xe5}
DEFINE_GUID!{MF_DEVICEMFT_CONNECTED_PIN_KSCONTROL,
    0xe63310f7, 0xb244, 0x4ef8, 0x9a, 0x7d, 0x24, 0xc7, 0x4e, 0x32, 0xeb, 0xd0}
DEFINE_GUID!{MF_DEVICE_THERMAL_STATE_CHANGED,
    0x70ccd0af, 0xfc9f, 0x4deb, 0xa8, 0x75, 0x9f, 0xec, 0xd1, 0x6c, 0x5b, 0xd4}
DEFINE_GUID!{MFSampleExtension_DeviceTimestamp,
    0x8f3e35e7, 0x2dcd, 0x4887, 0x86, 0x22, 0x2a, 0x58, 0xba, 0xa6, 0x52, 0xb0}
DEFINE_GUID!{MFSampleExtension_Spatial_CameraViewTransform,
    0x4e251fa4, 0x830f, 0x4770, 0x85, 0x9a, 0x4b, 0x8d, 0x99, 0xaa, 0x80, 0x9b}
DEFINE_GUID!{MFSampleExtension_Spatial_CameraCoordinateSystem,
    0x9d13c82f, 0x2199, 0x4e67, 0x91, 0xcd, 0xd1, 0xa4, 0x18, 0x1f, 0x25, 0x34}
DEFINE_GUID!{MFSampleExtension_Spatial_CameraProjectionTransform,
    0x47f9fcb5, 0x2a02, 0x4f26, 0xa4, 0x77, 0x79, 0x2f, 0xdf, 0x95, 0x88, 0x6a}
RIDL!{#[uuid(0xef5dc845, 0xf0d9, 0x4ec9, 0xb0, 0x0c, 0xcb, 0x51, 0x83, 0xd3, 0x84, 0x34)]
interface IMFProtectedEnvironmentAccess(IMFProtectedEnvironmentAccessVtbl):
  IUnknown(IUnknownVtbl) {
    fn Call(
        inputLength: UINT32,
        input: *const BYTE,
        outputLength: UINT32,
        output: *mut BYTE,
    ) -> HRESULT,
    fn ReadGRL(
        outputLength: *mut UINT32,
        output: *mut *mut BYTE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4a724bca, 0xff6a, 0x4c07, 0x8e, 0x0d, 0x7a, 0x35, 0x84, 0x21, 0xcf, 0x06)]
interface IMFSignedLibrary(IMFSignedLibraryVtbl): IUnknown(IUnknownVtbl) {
    fn GetProcedureAddress(
        name: LPCSTR,
        address: *mut PVOID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xfff4af3a, 0x1fc1, 0x4ef9, 0xa2, 0x9b, 0xd2, 0x6c, 0x49, 0xe2, 0xf3, 0x1a)]
interface IMFSystemId(IMFSystemIdVtbl): IUnknown(IUnknownVtbl) {
    fn GetData(
        size: *mut UINT32,
        data: *mut *mut BYTE,
    ) -> HRESULT,
    fn Setup(
        stage: UINT32,
        cbIn: UINT32,
        pbIn: *const BYTE,
        pcbOut: *mut UINT32,
        ppbOut: *mut *mut BYTE,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateProtectedEnvironmentAccess(
        ppAccess: *mut *mut IMFProtectedEnvironmentAccess,
    ) -> HRESULT;
    pub fn MFLoadSignedLibrary(
        pszName: LPCWSTR,
        ppLib: *mut *mut IMFSignedLibrary,
    ) -> HRESULT;
    pub fn MFGetSystemId(
        ppId: *mut *mut IMFSystemId,
    ) -> HRESULT;
    pub fn MFGetLocalId(
        verifier: *const BYTE,
        size: UINT32,
        id: *mut LPWSTR,
    ) -> HRESULT;
}
DEFINE_GUID!{CLSID_MPEG2ByteStreamPlugin,
    0x40871c59, 0xab40, 0x471f, 0x8d, 0xc3, 0x1f, 0x25, 0x9d, 0x86, 0x24, 0x79}
DEFINE_GUID!{MF_MEDIASOURCE_SERVICE,
    0xf09992f7, 0x9fba, 0x4c4a, 0xa3, 0x7f, 0x8c, 0x47, 0xb4, 0xe1, 0xdf, 0xe7}
DEFINE_GUID!{MF_ACCESS_CONTROLLED_MEDIASOURCE_SERVICE,
    0x014a5031, 0x2f05, 0x4c6a, 0x9f, 0x9c, 0x7d, 0x0d, 0xc4, 0xed, 0xa5, 0xf4}
STRUCT!{struct MFCONTENTPROTECTIONDEVICE_INPUT_DATA {
    HWProtectionFunctionID: DWORD,
    PrivateDataByteCount: DWORD,
    HWProtectionDataByteCount: DWORD,
    Reserved: DWORD,
    InputData: [BYTE; 4],
}}
STRUCT!{struct MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA {
    PrivateDataByteCount: DWORD,
    MaxHWProtectionDataByteCount: DWORD,
    HWProtectionDataByteCount: DWORD,
    Status: HRESULT,
    TransportTimeInHundredsOfNanoseconds: LONGLONG,
    ExecutionTimeInHundredsOfNanoseconds: LONGLONG,
    OutputData: [BYTE; 4],
}}
pub const MFCONTENTPROTECTIONDEVICE_FUNCTIONID_START: DWORD = 0x0400_0000;
pub const MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA_FUNCTIONID: DWORD
 = MFCONTENTPROTECTIONDEVICE_FUNCTIONID_START;
STRUCT!{struct MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA {
    TaskIndex: DWORD,
    ClassName: [WCHAR; MAX_PATH],
    BasePriority: LONG,
}}
RIDL!{#[uuid(0xE6257174, 0xA060, 0x4C9A, 0xA0, 0x88, 0x3B, 0x1B, 0x47, 0x1C, 0xAD, 0x28)]
interface IMFContentProtectionDevice(IMFContentProtectionDeviceVtbl): IUnknown(IUnknownVtbl) {
    fn InvokeFunction(
        FunctionId: DWORD,
        InputBufferByteCount: DWORD,
        InputBuffer: *const BYTE,
        OutputBufferByteCount: *mut DWORD,
        OutputBuffer: *mut BYTE,
    ) -> HRESULT,
    fn GetPrivateDataByteCount(
        PrivateInputByteCount: *mut DWORD,
        PrivateOutputByteCount: *mut DWORD,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateContentProtectionDevice(
        ProtectionSystemId: REFGUID,
        ContentProtectionDevice: *mut *mut IMFContentProtectionDevice,
    ) -> HRESULT;
    pub fn MFIsContentProtectionDeviceSupported(
        ProtectionSystemId: REFGUID,
        isSupported: *mut BOOL,
    ) -> HRESULT;
}
RIDL!{#[uuid(0x7EC4B1BD, 0x43FB, 0x4763, 0x85, 0xD2, 0x64, 0xFC, 0xB5, 0xC5, 0xF4, 0xCB)]
interface IMFContentDecryptorContext(IMFContentDecryptorContextVtbl): IUnknown(IUnknownVtbl) {
    fn InitializeHardwareKey(
        InputPrivateDataByteCount: UINT,
        InputPrivateData: *const c_void,
        OutputPrivateData: *mut UINT64,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_CONTENT_DECRYPTOR_SERVICE,
    0x68a72927, 0xfc7b, 0x44ee, 0x85, 0xf4, 0x7c, 0x51, 0xbd, 0x55, 0xa6, 0x59}
DEFINE_GUID!{MF_CONTENT_PROTECTION_DEVICE_SERVICE,
    0xff58436f, 0x76a0, 0x41fe, 0xb5, 0x66, 0x10, 0xcc, 0x53, 0x96, 0x2e, 0xdd}
extern "system" {
    pub fn MFCreateContentDecryptorContext(
        guidMediaProtectionSystemId: REFGUID,
        pD3DManager: *mut IMFDXGIDeviceManager,
        pContentProtectionDevice: *mut IMFContentProtectionDevice,
        ppContentDecryptorContext: *mut *mut IMFContentDecryptorContext,
    ) -> HRESULT;
}
DEFINE_GUID!{MF_SD_AUDIO_ENCODER_DELAY,
    0x8e85422c, 0x73de, 0x403f, 0x9a, 0x35, 0x55, 0x0a, 0xd6, 0xe8, 0xb9, 0x51}
DEFINE_GUID!{MF_SD_AUDIO_ENCODER_PADDING,
    0x529c7f2c, 0xac4b, 0x4e3f, 0xbf, 0xc3, 0x09, 0x02, 0x19, 0x49, 0x82, 0xcb}
DEFINE_GUID!{CLSID_MSH264DecoderMFT,
    0x62ce7e72, 0x4c71, 0x4d20, 0xb1, 0x5d, 0x45, 0x28, 0x31, 0xa8, 0x7d, 0x9d}
DEFINE_GUID!{CLSID_MSH264EncoderMFT,
    0x6ca50344, 0x051a, 0x4ded, 0x97, 0x79, 0xa4, 0x33, 0x05, 0x16, 0x5e, 0x35}
DEFINE_GUID!{CLSID_MSDDPlusDecMFT,
    0x177C0AFE, 0x900B, 0x48d4, 0x9E, 0x4C, 0x57, 0xAD, 0xD2, 0x50, 0xB3, 0xD4}
DEFINE_GUID!{CLSID_MP3DecMediaObject,
    0xbbeea841, 0x0a63, 0x4f52, 0xa7, 0xab, 0xa9, 0xb3, 0xa8, 0x4e, 0xd3, 0x8a}
DEFINE_GUID!{CLSID_MSAACDecMFT,
    0x32d186a7, 0x218f, 0x4c75, 0x88, 0x76, 0xdd, 0x77, 0x27, 0x3a, 0x89, 0x99}
DEFINE_GUID!{CLSID_MSH265DecoderMFT,
    0x420A51A3, 0xD605, 0x430C, 0xB4, 0xFC, 0x45, 0x27, 0x4F, 0xA6, 0xC5, 0x62}
DEFINE_GUID!{CLSID_WMVDecoderMFT,
    0x82d353df, 0x90bd, 0x4382, 0x8b, 0xc2, 0x3f, 0x61, 0x92, 0xb7, 0x6e, 0x34}
DEFINE_GUID!{CLSID_WMADecMediaObject,
    0x2eeb4adf, 0x4578, 0x4d10, 0xbc, 0xa7, 0xbb, 0x95, 0x5f, 0x56, 0x32, 0x0a}
DEFINE_GUID!{CLSID_MSMPEGAudDecMFT,
    0x70707B39, 0xB2CA, 0x4015, 0xAB, 0xEA, 0xF8, 0x44, 0x7D, 0x22, 0xD8, 0x8B}
DEFINE_GUID!{CLSID_MSMPEGDecoderMFT,
    0x2D709E52, 0x123F, 0x49b5, 0x9C, 0xBC, 0x9A, 0xF5, 0xCD, 0xE2, 0x8F, 0xB9}
DEFINE_GUID!{CLSID_AudioResamplerMediaObject,
    0xf447b69e, 0x1884, 0x4a7e, 0x80, 0x55, 0x34, 0x6f, 0x74, 0xd6, 0xed, 0xb3}
DEFINE_GUID!{CLSID_MSVPxDecoder,
    0xE3AAF548, 0xC9A4, 0x4C6E, 0x23, 0x4D, 0x5A, 0xDA, 0x37, 0x4B, 0x00, 0x00}
DEFINE_GUID!{CLSID_MSOpusDecoder,
    0x63e17c10, 0x2d43, 0x4c42, 0x8f, 0xe3, 0x8d, 0x8b, 0x63, 0xe4, 0x6a, 0x6a}
DEFINE_GUID!{CLSID_VideoProcessorMFT,
    0x88753b26, 0x5b24, 0x49bd, 0xb2, 0xe7, 0x0c, 0x44, 0x5c, 0x78, 0xc9, 0x82}
ENUM!{enum MF_MEDIAKEYSESSION_TYPE {
    MF_MEDIAKEYSESSION_TYPE_TEMPORARY,
    MF_MEDIAKEYSESSION_TYPE_PERSISTENT_LICENSE,
    MF_MEDIAKEYSESSION_TYPE_PERSISTENT_RELEASE_MESSAGE,
    MF_MEDIAKEYSESSION_TYPE_PERSISTENT_USAGE_RECORD,
}}
ENUM!{enum MF_MEDIAKEY_STATUS {
    MF_MEDIAKEY_STATUS_USABLE,
    MF_MEDIAKEY_STATUS_EXPIRED,
    MF_MEDIAKEY_STATUS_OUTPUT_DOWNSCALED,
    MF_MEDIAKEY_STATUS_OUTPUT_NOT_ALLOWED,
    MF_MEDIAKEY_STATUS_STATUS_PENDING,
    MF_MEDIAKEY_STATUS_INTERNAL_ERROR,
    MF_MEDIAKEY_STATUS_RELEASED,
    MF_MEDIAKEY_STATUS_OUTPUT_RESTRICTED,
}}
STRUCT!{struct MFMediaKeyStatus {
    pbKeyId: *mut BYTE,
    cbKeyId: UINT,
    eMediaKeyStatus: MF_MEDIAKEY_STATUS,
}}
ENUM!{enum MF_MEDIAKEYSESSION_MESSAGETYPE {
    MF_MEDIAKEYSESSION_MESSAGETYPE_LICENSE_REQUEST,
    MF_MEDIAKEYSESSION_MESSAGETYPE_LICENSE_RENEWAL,
    MF_MEDIAKEYSESSION_MESSAGETYPE_LICENSE_RELEASE,
    MF_MEDIAKEYSESSION_MESSAGETYPE_INDIVIDUALIZATION_REQUEST,
}}
ENUM!{enum MF_CROSS_ORIGIN_POLICY {
    MF_CROSS_ORIGIN_POLICY_NONE,
    MF_CROSS_ORIGIN_POLICY_ANONYMOUS,
    MF_CROSS_ORIGIN_POLICY_USE_CREDENTIALS,
}}
RIDL!{#[uuid(0xbc2b7d44, 0xa72d, 0x49d5, 0x83, 0x76, 0x14, 0x80, 0xde, 0xe5, 0x8b, 0x22)]
interface IMFNetCrossOriginSupport(IMFNetCrossOriginSupportVtbl): IUnknown(IUnknownVtbl) {
    fn GetCrossOriginPolicy(
        pPolicy: *mut MF_CROSS_ORIGIN_POLICY,
    ) -> HRESULT,
    fn GetSourceOrigin(
        wszSourceOrigin: *mut LPWSTR,
    ) -> HRESULT,
    fn IsSameOrigin(
        wszURL: LPCWSTR,
        pfIsSameOrigin: *mut BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{MFNETSOURCE_CROSS_ORIGIN_SUPPORT,
    0x9842207c, 0xb02c, 0x4271, 0xa2, 0xfc, 0x72, 0xe4, 0x93, 0x08, 0xe5, 0xc2}
RIDL!{#[uuid(0xF779FDDF, 0x26E7, 0x4270, 0x8A, 0x8B, 0xB9, 0x83, 0xD1, 0x85, 0x9D, 0xE0)]
interface IMFHttpDownloadRequest(IMFHttpDownloadRequestVtbl): IUnknown(IUnknownVtbl) {
    fn AddHeader(
        szHeader: LPCWSTR,
    ) -> HRESULT,
    fn BeginSendRequest(
        pbPayload: *const BYTE,
        cbPayload: ULONG,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndSendRequest(
        pResult: *mut IMFAsyncResult,
    ) -> HRESULT,
    fn BeginReceiveResponse(
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndReceiveResponse(
        pResult: *mut IMFAsyncResult,
    ) -> HRESULT,
    fn BeginReadPayload(
        pb: *mut BYTE,
        cb: ULONG,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndReadPayload(
        pResult: *mut IMFAsyncResult,
        pqwOffset: *mut QWORD,
        pcbRead: *mut ULONG,
    ) -> HRESULT,
    fn QueryHeader(
        szHeaderName: LPCWSTR,
        dwIndex: DWORD,
        ppszHeaderValue: *mut LPWSTR,
    ) -> HRESULT,
    fn GetURL(
        ppszURL: *mut LPWSTR,
    ) -> HRESULT,
    fn HasNullSourceOrigin(
        pfNullSourceOrigin: *mut BOOL,
    ) -> HRESULT,
    fn GetTimeSeekResult(
        pqwStartTime: *mut QWORD,
        pqwStopTime: *mut QWORD,
        pqwDuration: *mut QWORD,
    ) -> HRESULT,
    fn GetHttpStatus(
        pdwHttpStatus: *mut DWORD,
    ) -> HRESULT,
    fn GetAtEndOfPayload(
        pfAtEndOfPayload: *mut BOOL,
    ) -> HRESULT,
    fn GetTotalLength(
        pqwTotalLength: *mut QWORD,
    ) -> HRESULT,
    fn GetRangeEndOffset(
        pqwRangeEnd: *mut QWORD,
    ) -> HRESULT,
    fn Close() -> HRESULT,
}}
RIDL!{#[uuid(0x71FA9A2C, 0x53CE, 0x4662, 0xA1, 0x32, 0x1A, 0x7E, 0x8C, 0xBF, 0x62, 0xDB)]
interface IMFHttpDownloadSession(IMFHttpDownloadSessionVtbl): IUnknown(IUnknownVtbl) {
    fn SetServer(
        szServerName: LPCWSTR,
        nPort: DWORD,
    ) -> HRESULT,
    fn CreateRequest(
        szObjectName: LPCWSTR,
        fBypassProxyCache: BOOL,
        fSecure: BOOL,
        szVerb: LPCWSTR,
        szReferrer: LPCWSTR,
        ppRequest: *mut *mut IMFHttpDownloadRequest,
    ) -> HRESULT,
    fn Close() -> HRESULT,
}}
RIDL!{#[uuid(0x1B4CF4B9, 0x3A16, 0x4115, 0x83, 0x9D, 0x03, 0xCC, 0x5C, 0x99, 0xDF, 0x01)]
interface IMFHttpDownloadSessionProvider(IMFHttpDownloadSessionProviderVtbl):
  IUnknown(IUnknownVtbl) {
    fn CreateHttpDownloadSession(
        wszScheme: LPCWSTR,
        ppDownloadSession: *mut *mut IMFHttpDownloadSession,
    ) -> HRESULT,
}}
DEFINE_GUID!{MFNETSOURCE_HTTP_DOWNLOAD_SESSION_PROVIDER,
    0x7d55081e, 0x307d, 0x4d6d, 0xa6, 0x63, 0xa9, 0x3b, 0xe9, 0x7c, 0x4b, 0x5c}
ENUM!{enum MF_MEDIASOURCE_STATUS_INFO {
    MF_MEDIASOURCE_STATUS_INFO_FULLYSUPPORTED,
    MF_MEDIASOURCE_STATUS_INFO_UNKNOWN,
}}
DEFINE_GUID!{MF_SD_MEDIASOURCE_STATUS,
    0x1913678b, 0xfc0f, 0x44da, 0x8f, 0x43, 0x1b, 0xa3, 0xb5, 0x26, 0xf4, 0xae}
STRUCT!{struct MF_VIDEO_SPHERICAL_VIEWDIRECTION {
    iHeading: c_int,
    iPitch: c_int,
    iRoll: c_int,
}}
pub const MF_UNKNOWN_DURATION: DWORD = 0;
DEFINE_GUID!{MF_SD_VIDEO_SPHERICAL,
    0xa51da449, 0x3fdc, 0x478c, 0xbc, 0xb5, 0x30, 0xbe, 0x76, 0x59, 0x5f, 0x55}
DEFINE_GUID!{MF_SD_VIDEO_SPHERICAL_FORMAT,
    0x4a8fc407, 0x6ea1, 0x46c8, 0xb5, 0x67, 0x69, 0x71, 0xd4, 0xa1, 0x39, 0xc3}
DEFINE_GUID!{MF_SD_VIDEO_SPHERICAL_INITIAL_VIEWDIRECTION,
    0x11d25a49, 0xbb62, 0x467f, 0x9d, 0xb1, 0xc1, 0x71, 0x65, 0x71, 0x6c, 0x49}
DEFINE_GUID!{MF_MEDIASOURCE_EXPOSE_ALL_STREAMS,
    0xe7f250b8, 0x8fd9, 0x4a09, 0xb6, 0xc1, 0x6a, 0x31, 0x5c, 0x7c, 0x72, 0x0e}
RIDL!{#[uuid(0xFBB03414, 0xD13B, 0x4786, 0x83, 0x19, 0x5A, 0xC5, 0x1F, 0xC0, 0xA1, 0x36)]
interface IMFMediaSource2(IMFMediaSource2Vtbl): IMFMediaSourceEx(IMFMediaSourceExVtbl) {
    fn SetMediaType(
        dwStreamID: DWORD,
        pMediaType: *mut IMFMediaType,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xC5BC37D6, 0x75C7, 0x46A1, 0xA1, 0x32, 0x81, 0xB5, 0xF7, 0x23, 0xC2, 0x0F)]
interface IMFMediaStream2(IMFMediaStream2Vtbl): IMFMediaStream(IMFMediaStreamVtbl) {
    fn SetStreamState(
        value: MF_STREAM_STATE,
    ) -> HRESULT,
    fn GetStreamState(
        value: *mut MF_STREAM_STATE,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_ST_MEDIASOURCE_COLLECTION,
    0x616de972, 0x83ad, 0x4950, 0x81, 0x70, 0x63, 0x0d, 0x19, 0xcb, 0xe3, 0x07}
DEFINE_GUID!{MF_DEVICESTREAM_FILTER_KSCONTROL,
    0x46783cca, 0x3df5, 0x4923, 0xa9, 0xef, 0x36, 0xb7, 0x22, 0x3e, 0xdd, 0xe0}
DEFINE_GUID!{MF_DEVICESTREAM_PIN_KSCONTROL,
    0xef3ef9a7, 0x87f2, 0x48ca, 0xbe, 0x02, 0x67, 0x48, 0x78, 0x91, 0x8e, 0x98}
DEFINE_GUID!{MF_DEVICESTREAM_SOURCE_ATTRIBUTES,
    0x2f8cb617, 0x361b, 0x434f, 0x85, 0xea, 0x99, 0xa0, 0x3e, 0x1c, 0xe4, 0xe0}
DEFINE_GUID!{MF_DEVICESTREAM_FRAMESERVER_HIDDEN,
    0xf402567b, 0x4d91, 0x4179, 0x96, 0xd1, 0x74, 0xc8, 0x48, 0x0c, 0x20, 0x34}
DEFINE_GUID!{MF_STF_VERSION_INFO,
    0x6770bd39, 0xef82, 0x44ee, 0xa4, 0x9b, 0x93, 0x4b, 0xeb, 0x24, 0xae, 0xf7}
DEFINE_GUID!{MF_STF_VERSION_DATE,
    0x31a165d5, 0xdf67, 0x4095, 0x8e, 0x44, 0x88, 0x68, 0xfc, 0x20, 0xdb, 0xfd}
DEFINE_GUID!{MF_DEVICESTREAM_REQUIRED_CAPABILITIES,
    0x6d8b957e, 0x7cf6, 0x43f4, 0xaf, 0x56, 0x9c, 0x0e, 0x1e, 0x4f, 0xcb, 0xe1}
DEFINE_GUID!{MF_DEVICESTREAM_REQUIRED_SDDL,
    0x331ae85d, 0xc0d3, 0x49ba, 0x83, 0xba, 0x82, 0xa1, 0x2d, 0x63, 0xcd, 0xd6}
DEFINE_GUID!{MF_DEVICEMFT_SENSORPROFILE_COLLECTION,
    0x36ebdc44, 0xb12c, 0x441b, 0x89, 0xf4, 0x08, 0xb2, 0xf4, 0x1a, 0x9c, 0xfc}
DEFINE_GUID!{MF_DEVICESTREAM_SENSORSTREAM_ID,
    0xe35b9fe4, 0x0659, 0x4cad, 0xbb, 0x51, 0x33, 0x16, 0x0b, 0xe7, 0xe4, 0x13}
ENUM!{enum MFSensorDeviceType {
    MFSensorDeviceType_Unknown,
    MFSensorDeviceType_Device,
    MFSensorDeviceType_MediaSource,
    MFSensorDeviceType_FrameProvider,
    MFSensorDeviceType_SensorTransform,
}}
ENUM!{enum MFSensorStreamType {
    MFSensorStreamType_Unknown,
    MFSensorStreamType_Input,
    MFSensorStreamType_Output,
}}
ENUM!{enum MFSensorDeviceMode {
    MFSensorDeviceMode_Controller,
    MFSensorDeviceMode_Shared,
}}
RIDL!{#[uuid(0xFB9F48F2, 0x2A18, 0x4E28, 0x97, 0x30, 0x78, 0x6F, 0x30, 0xF0, 0x4D, 0xC4)]
interface IMFSensorDevice(IMFSensorDeviceVtbl): IUnknown(IUnknownVtbl) {
    fn GetDeviceId(
        pDeviceId: *mut ULONGLONG,
    ) -> HRESULT,
    fn GetDeviceType(
        pType: *mut MFSensorDeviceType,
    ) -> HRESULT,
    fn GetFlags(
        pFlags: *mut ULONGLONG,
    ) -> HRESULT,
    fn GetSymbolicLink(
        SymbolicLink: LPWSTR,
        cchSymbolicLink: LONG,
        pcchWritten: *mut LONG,
    ) -> HRESULT,
    fn GetDeviceAttributes(
        ppAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT,
    fn GetStreamAttributesCount(
        eType: MFSensorStreamType,
        pdwCount: *mut DWORD,
    ) -> HRESULT,
    fn GetStreamAttributes(
        eType: MFSensorStreamType,
        dwIndex: DWORD,
        ppAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT,
    fn SetSensorDeviceMode(
        eMode: MFSensorDeviceMode,
    ) -> HRESULT,
    fn GetSensorDeviceMode(
        peMode: *mut MFSensorDeviceMode,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4110243A, 0x9757, 0x461F, 0x89, 0xF1, 0xF2, 0x23, 0x45, 0xBC, 0xAB, 0x4E)]
interface IMFSensorGroup(IMFSensorGroupVtbl): IUnknown(IUnknownVtbl) {
    fn GetSymbolicLink(
        SymbolicLink: LPWSTR,
        cchSymbolicLink: LONG,
        pcchWritten: *mut LONG,
    ) -> HRESULT,
    fn GetFlags(
        pFlags: *mut ULONGLONG,
    ) -> HRESULT,
    fn GetSensorGroupAttributes(
        ppAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT,
    fn GetSensorDeviceCount(
        pdwCount: *mut DWORD,
    ) -> HRESULT,
    fn GetSensorDevice(
        dwIndex: DWORD,
        ppDevice: *mut *mut IMFSensorDevice,
    ) -> HRESULT,
    fn SetDefaultSensorDeviceIndex(
        dwIndex: DWORD,
    ) -> HRESULT,
    fn GetDefaultSensorDeviceIndex(
        pdwIndex: *mut DWORD,
    ) -> HRESULT,
    fn CreateMediaSource(
        ppSource: *mut *mut IMFMediaSource,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xE9A42171, 0xC56E, 0x498A, 0x8B, 0x39, 0xED, 0xA5, 0xA0, 0x70, 0xB7, 0xFC)]
interface IMFSensorStream(IMFSensorStreamVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetMediaTypeCount(
        pdwCount: *mut DWORD,
    ) -> HRESULT,
    fn GetMediaType(
        dwIndex: DWORD,
        ppMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn CloneSensorStream(
        ppStream: *mut *mut IMFSensorStream,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xEED9C2EE, 0x66B4, 0x4F18, 0xA6, 0x97, 0xAC, 0x7D, 0x39, 0x60, 0x21, 0x5C)]
interface IMFSensorTransformFactory(IMFSensorTransformFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn GetFactoryAttributes(
        ppAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT,
    fn InitializeFactory(
        dwMaxTransformCount: DWORD,
        pSensorDevices: *mut IMFCollection,
        pAttributes: *mut IMFAttributes,
    ) -> HRESULT,
    fn GetTransformCount(
        pdwCount: *mut DWORD,
    ) -> HRESULT,
    fn GetTransformInformation(
        TransformIndex: DWORD,
        pguidTransformId: *mut GUID,
        ppAttributes: *mut *mut IMFAttributes,
        ppStreamInformation: *mut *mut IMFCollection,
    ) -> HRESULT,
    fn CreateTransform(
        guidSensorTransformID: REFGUID,
        pAttributes: *mut IMFAttributes,
        ppDeviceMFT: *mut *mut IMFDeviceTransform,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateSensorGroup(
        SensorGroupSymbolicLink: LPCWSTR,
        ppSensorGroup: *mut *mut IMFSensorGroup,
    ) -> HRESULT;
    pub fn MFCreateSensorStream(
        StreamId: DWORD,
        pAttributes: *mut IMFAttributes,
        pMediaTypeCollection: *mut IMFCollection,
        ppStream: *mut *mut IMFSensorStream,
    ) -> HRESULT;
}
STRUCT!{struct SENSORPROFILEID {
    Type: GUID,
    Index: UINT32,
    Unused: UINT32,
}}
RIDL!{#[uuid(0x22F765D1, 0x8DAB, 0x4107, 0x84, 0x6D, 0x56, 0xBA, 0xF7, 0x22, 0x15, 0xE7)]
interface IMFSensorProfile(IMFSensorProfileVtbl): IUnknown(IUnknownVtbl) {
    fn GetProfileId(
        pId: *mut SENSORPROFILEID,
    ) -> HRESULT,
    fn AddProfileFilter(
        StreamId: UINT32,
        wzFilterSetString: LPCWSTR,
    ) -> HRESULT,
    fn IsMediaTypeSupported(
        StreamId: UINT32,
        pMediaType: *mut IMFMediaType,
        pfSupported: *mut BOOL,
    ) -> HRESULT,
    fn AddBlockedControl(
        wzBlockedControl: LPCWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xC95EA55B, 0x0187, 0x48BE, 0x93, 0x53, 0x8D, 0x25, 0x07, 0x66, 0x23, 0x51)]
interface IMFSensorProfileCollection(IMFSensorProfileCollectionVtbl): IUnknown(IUnknownVtbl) {
    fn GetProfileCount() -> DWORD,
    fn GetProfile(
        Index: DWORD,
        ppProfile: *mut *mut IMFSensorProfile,
    ) -> HRESULT,
    fn AddProfile(
        pProfile: *mut IMFSensorProfile,
    ) -> HRESULT,
    fn FindProfile(
        ProfileId: *mut SENSORPROFILEID,
        ppProfile: *mut *mut IMFSensorProfile,
    ) -> HRESULT,
    fn RemoveProfileByIndex(
        Index: DWORD,
    ) -> (),
    fn RemoveProfile(
        ProfileId: *mut SENSORPROFILEID,
    ) -> (),
}}
extern "system" {
    pub fn MFCreateSensorProfile(
        ProfileType: REFGUID,
        ProfileIndex: UINT32,
        Constraints: LPCWSTR,
        ppProfile: *mut *mut IMFSensorProfile,
    ) -> HRESULT;
    pub fn MFCreateSensorProfileCollection(
        ppSensorProfile: *mut *mut IMFSensorProfileCollection,
    ) -> HRESULT;
}
RIDL!{#[uuid(0x39DC7F4A, 0xB141, 0x4719, 0x81, 0x3C, 0xA7, 0xF4, 0x61, 0x62, 0xA2, 0xB8)]
interface IMFSensorProcessActivity(IMFSensorProcessActivityVtbl): IUnknown(IUnknownVtbl) {
    fn GetProcessId(
        pPID: *mut ULONG,
    ) -> HRESULT,
    fn GetStreamingState(
        pfStreaming: *mut BOOL,
    ) -> HRESULT,
    fn GetStreamingMode(
        pMode: *mut MFSensorDeviceMode,
    ) -> HRESULT,
    fn GetReportTime(
        pft: *mut FILETIME,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3E8C4BE1, 0xA8C2, 0x4528, 0x90, 0xDE, 0x28, 0x51, 0xBD, 0xE5, 0xFE, 0xAD)]
interface IMFSensorActivityReport(IMFSensorActivityReportVtbl): IUnknown(IUnknownVtbl) {
    fn GetFriendlyName(
        FriendlyName: LPWSTR,
        cchFriendlyName: ULONG,
        pcchWritten: *mut ULONG,
    ) -> HRESULT,
    fn GetSymbolicLink(
        SymbolicLink: LPWSTR,
        cchSymbolicLink: ULONG,
        pcchWritten: *mut ULONG,
    ) -> HRESULT,
    fn GetProcessCount(
        pcCount: *mut ULONG,
    ) -> HRESULT,
    fn GetProcessActivity(
        Index: ULONG,
        ppProcessActivity: *mut *mut IMFSensorProcessActivity,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x683F7A5E, 0x4A19, 0x43CD, 0xB1, 0xA9, 0xDB, 0xF4, 0xAB, 0x3F, 0x77, 0x77)]
interface IMFSensorActivitiesReport(IMFSensorActivitiesReportVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(
        pcCount: *mut ULONG,
    ) -> HRESULT,
    fn GetActivityReport(
        Index: ULONG,
        sensorActivityReport: *mut *mut IMFSensorActivityReport,
    ) -> HRESULT,
    fn GetActivityReportByDeviceName(
        SymbolicName: LPCWSTR,
        sensorActivityReport: *mut *mut IMFSensorActivityReport,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xDE5072EE, 0xDBE3, 0x46DC, 0x8A, 0x87, 0xB6, 0xF6, 0x31, 0x19, 0x47, 0x51)]
interface IMFSensorActivitiesReportCallback(IMFSensorActivitiesReportCallbackVtbl):
  IUnknown(IUnknownVtbl) {
    fn OnActivitiesReport(
        sensorActivitiesReport: *mut IMFSensorActivitiesReport,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xD0CEF145, 0xB3F4, 0x4340, 0xA2, 0xE5, 0x7A, 0x50, 0x80, 0xCA, 0x05, 0xCB)]
interface IMFSensorActivityMonitor(IMFSensorActivityMonitorVtbl): IUnknown(IUnknownVtbl) {
    fn Start() -> HRESULT,
    fn Stop() -> HRESULT,
}}
extern "system" {
    pub fn MFCreateSensorActivityMonitor(
        pCallback: *mut IMFSensorActivitiesReportCallback,
        ppActivityMonitor: *mut *mut IMFSensorActivityMonitor,
    ) -> HRESULT;
}
STRUCT!{struct MFCameraIntrinsic_CameraModel {
    FocalLength_x: FLOAT,
    FocalLength_y: FLOAT,
    PrincipalPoint_x: FLOAT,
    PrincipalPoint_y: FLOAT,
}}
STRUCT!{struct MFCameraIntrinsic_DistortionModel6KT {
    Radial_k1: FLOAT,
    Radial_k2: FLOAT,
    Radial_k3: FLOAT,
    Radial_k4: FLOAT,
    Radial_k5: FLOAT,
    Radial_k6: FLOAT,
    Tangential_p1: FLOAT,
    Tangential_p2: FLOAT,
}}
STRUCT!{struct MFCameraIntrinsic_DistortionModelArcTan {
    Radial_k0: FLOAT,
    DistortionCenter_x: FLOAT,
    DistortionCenter_y: FLOAT,
    Tangential_x: FLOAT,
    Tangential_y: FLOAT,
}}
ENUM!{enum MFCameraIntrinsic_DistortionModelType {
    MFCameraIntrinsic_DistortionModelType_6KT,
    MFCameraIntrinsic_DistortionModelType_ArcTan,
}}
STRUCT!{struct MFExtendedCameraIntrinsic_IntrinsicModel {
    Width: UINT32,
    Height: UINT32,
    SplitFrameId: UINT32,
    CameraModel: MFCameraIntrinsic_CameraModel,
}}
RIDL!{#[uuid(0x5C595E64, 0x4630, 0x4231, 0x85, 0x5A, 0x12, 0x84, 0x2F, 0x73, 0x32, 0x45)]
interface IMFExtendedCameraIntrinsicModel(IMFExtendedCameraIntrinsicModelVtbl):
  IUnknown(IUnknownVtbl) {
    fn GetModel(
        pIntrinsicModel: *mut MFExtendedCameraIntrinsic_IntrinsicModel,
    ) -> HRESULT,
    fn SetModel(
        pIntrinsicModel: *const MFExtendedCameraIntrinsic_IntrinsicModel,
    ) -> HRESULT,
    fn GetDistortionModelType(
        pDistortionModelType: *mut MFCameraIntrinsic_DistortionModelType,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x74C2653B, 0x5F55, 0x4EB1, 0x9F, 0x0F, 0x18, 0xB8, 0xF6, 0x8B, 0x7D, 0x3D)]
interface IMFExtendedCameraIntrinsicsDistortionModel6KT(IMFExtendedCameraIntrinsicsDistortionModel6KTVtbl):
  IUnknown(IUnknownVtbl) {
    fn GetDistortionModel(
        pDistortionModel: *mut MFCameraIntrinsic_DistortionModel6KT,
    ) -> HRESULT,
    fn SetDistortionModel(
        pDistortionModel: *const MFCameraIntrinsic_DistortionModel6KT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x812D5F95, 0xB572, 0x45DC, 0xBA, 0xFC, 0xAE, 0x24, 0x19, 0x9D, 0xDD, 0xA8)]
interface IMFExtendedCameraIntrinsicsDistortionModelArcTan(IMFExtendedCameraIntrinsicsDistortionModelArcTanVtbl):
  IUnknown(IUnknownVtbl) {
    fn GetDistortionModel(
        pDistortionModel: *mut MFCameraIntrinsic_DistortionModelArcTan,
    ) -> HRESULT,
    fn SetDistortionModel(
        pDistortionModel: *const MFCameraIntrinsic_DistortionModelArcTan,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x687F6DAC, 0x6987, 0x4750, 0xA1, 0x6A, 0x73, 0x4D, 0x1E, 0x7A, 0x10, 0xFE)]
interface IMFExtendedCameraIntrinsics(IMFExtendedCameraIntrinsicsVtbl):
  IUnknown(IUnknownVtbl) {
    fn InitializeFromBuffer(
        pbBuffer: *mut BYTE,
        dwBufferSize: DWORD,
    ) -> HRESULT,
    fn GetBufferSize(
        pdwBufferSize: *mut DWORD,
    ) -> HRESULT,
    fn SerializeToBuffer(
        pbBuffer: *mut BYTE,
        pdwBufferSize: *mut DWORD,
    ) -> HRESULT,
    fn GetIntrinsicModelCount(
        pdwCount: *mut DWORD,
    ) -> HRESULT,
    fn GetIntrinsicModelByIndex(
        dwIndex: DWORD,
        ppIntrinsicModel: *mut *mut IMFExtendedCameraIntrinsicModel,
    ) -> HRESULT,
    fn AddIntrinsicModel(
        pIntrinsicModel: *mut IMFExtendedCameraIntrinsicModel,
    ) -> HRESULT,
}}
DEFINE_GUID!{MFStreamExtension_ExtendedCameraIntrinsics,
    0xaa74b3df, 0x9a2c, 0x48d6, 0x83, 0x93, 0x5b, 0xd1, 0xc1, 0xa8, 0x1e, 0x6e}
DEFINE_GUID!{MFSampleExtension_ExtendedCameraIntrinsics,
    0x560bc4a5, 0x4de0, 0x4113, 0x9c, 0xdc, 0x83, 0x2d, 0xb9, 0x74, 0x0f, 0x3d}
extern "system" {
    pub fn MFCreateExtendedCameraIntrinsics(
        ppExtendedCameraIntrinsics: *mut *mut IMFExtendedCameraIntrinsics,
    ) -> HRESULT;
    pub fn MFCreateExtendedCameraIntrinsicModel(
        distortionModelType: MFCameraIntrinsic_DistortionModelType,
        ppExtendedCameraIntrinsicModel: *mut *mut IMFExtendedCameraIntrinsicModel,
    ) -> HRESULT;
}
