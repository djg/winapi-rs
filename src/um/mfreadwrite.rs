// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of mfreadwrite.h
use shared::guiddef::{GUID, REFCLSID, REFGUID, REFIID};
use shared::minwindef::{BOOL, DWORD, LPVOID};
use um::mfidl::{IMFMediaSink, IMFMediaSource};
use um::mfobjects::{
    IMFAttributes, IMFByteStream, IMFMediaEvent, IMFMediaType, IMFSample,
    QWORD,
};
use um::mftransform::IMFTransform;
use um::propidl::{PROPVARIANT, REFPROPVARIANT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LONGLONG, LPCWSTR};
DEFINE_GUID!{CLSID_MFReadWriteClassFactory,
    0x48e2ed0f, 0x98c2, 0x4a37, 0xbe, 0xd5, 0x16, 0x63, 0x12, 0xdd, 0xd8, 0x3f}
RIDL!{#[uuid(0xE7FE2E12, 0x661C, 0x40DA, 0x92, 0xF9, 0x4F, 0x00, 0x2A, 0xB6, 0x76, 0x27)]
interface IMFReadWriteClassFactory(IMFReadWriteClassFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn CreateInstanceFromURL(
        clsid: REFCLSID,
        pwszURL: LPCWSTR,
        pAttributes: *mut IMFAttributes,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
    fn CreateInstanceFromObject(
        clsid: REFCLSID,
        punkObject: *mut IUnknown,
        pAttributes: *mut IMFAttributes,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_MFSourceReader,
    0x1777133c, 0x0881, 0x411b, 0xa5, 0x77, 0xad, 0x54, 0x5f, 0x07, 0x14, 0xc4}
extern "system" {
    pub fn MFCreateSourceReaderFromURL(
        pwszURL: LPCWSTR,
        pAttributes: *mut IMFAttributes,
        ppSourceReader: *mut *mut IMFSourceReader,
    ) -> HRESULT;
    pub fn MFCreateSourceReaderFromByteStream(
        pByteStream: *mut IMFByteStream,
        pAttributes: *mut IMFAttributes,
        ppSourceReader: *mut *mut IMFSourceReader,
    ) -> HRESULT;
    pub fn MFCreateSourceReaderFromMediaSource(
        pMediaSource: *mut IMFMediaSource,
        pAttributes: *mut IMFAttributes,
        ppSourceReader: *mut *mut IMFSourceReader,
    ) -> HRESULT;
}
DEFINE_GUID!{MF_SOURCE_READER_ASYNC_CALLBACK,
    0x1e3dbeac, 0xbb43, 0x4c35, 0xb5, 0x07, 0xcd, 0x64, 0x44, 0x64, 0xc9, 0x65}
DEFINE_GUID!{MF_SOURCE_READER_D3D_MANAGER,
    0xec822da2, 0xe1e9, 0x4b29, 0xa0, 0xd8, 0x56, 0x3c, 0x71, 0x9f, 0x52, 0x69}
DEFINE_GUID!{MF_SOURCE_READER_DISABLE_DXVA,
    0xaa456cfd, 0x3943, 0x4a1e, 0xa7, 0x7d, 0x18, 0x38, 0xc0, 0xea, 0x2e, 0x35}
DEFINE_GUID!{MF_SOURCE_READER_MEDIASOURCE_CONFIG,
    0x9085abeb, 0x0354, 0x48f9, 0xab, 0xb5, 0x20, 0x0d, 0xf8, 0x38, 0xc6, 0x8e}
DEFINE_GUID!{MF_SOURCE_READER_MEDIASOURCE_CHARACTERISTICS,
    0x6d23f5c8, 0xc5d7, 0x4a9b, 0x99, 0x71, 0x5d, 0x11, 0xf8, 0xbc, 0xa8, 0x80}
DEFINE_GUID!{MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING,
    0xfb394f3d, 0xccf1, 0x42ee, 0xbb, 0xb3, 0xf9, 0xb8, 0x45, 0xd5, 0x68, 0x1d}
DEFINE_GUID!{MF_SOURCE_READER_ENABLE_ADVANCED_VIDEO_PROCESSING,
    0x0f81da2c, 0xb537, 0x4672, 0xa8, 0xb2, 0xa6, 0x81, 0xb1, 0x73, 0x07, 0xa3}
DEFINE_GUID!{MF_SOURCE_READER_DISABLE_CAMERA_PLUGINS,
    0x9d3365dd, 0x058f, 0x4cfb, 0x9f, 0x97, 0xb3, 0x14, 0xcc, 0x99, 0xc8, 0xad}
DEFINE_GUID!{MF_SOURCE_READER_DISCONNECT_MEDIASOURCE_ON_SHUTDOWN,
    0x56b67165, 0x219e, 0x456d, 0xa2, 0x2e, 0x2d, 0x30, 0x04, 0xc7, 0xfe, 0x56}
DEFINE_GUID!{MF_SOURCE_READER_ENABLE_TRANSCODE_ONLY_TRANSFORMS,
    0xdfd4f008, 0xb5fd, 0x4e78, 0xae, 0x44, 0x62, 0xa1, 0xe6, 0x7b, 0xbe, 0x27}
DEFINE_GUID!{MF_SOURCE_READER_D3D11_BIND_FLAGS,
    0x33f3197b, 0xf73a, 0x4e14, 0x8d, 0x85, 0x0e, 0x4c, 0x43, 0x68, 0x78, 0x8d}
ENUM!{enum MF_SOURCE_READER_FLAG {
    MF_SOURCE_READERF_ERROR	= 0x1,
    MF_SOURCE_READERF_ENDOFSTREAM = 0x2,
    MF_SOURCE_READERF_NEWSTREAM	= 0x4,
    MF_SOURCE_READERF_NATIVEMEDIATYPECHANGED = 0x10,
    MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED = 0x20,
    MF_SOURCE_READERF_STREAMTICK = 0x100,
    MF_SOURCE_READERF_ALLEFFECTSREMOVED	= 0x200,
}}
ENUM!{enum MF_SOURCE_READER_CONTROL_FLAG {
    MF_SOURCE_READER_CONTROLF_DRAIN	= 0x1,
}}
pub const MF_SOURCE_READER_INVALID_STREAM_INDEX: DWORD = 0xffffffff;
pub const MF_SOURCE_READER_ALL_STREAMS: DWORD = 0xfffffffe;
pub const MF_SOURCE_READER_ANY_STREAM: DWORD = 0xfffffffe;
pub const MF_SOURCE_READER_FIRST_AUDIO_STREAM: DWORD = 0xfffffffd;
pub const MF_SOURCE_READER_FIRST_VIDEO_STREAM: DWORD = 0xfffffffc;
pub const MF_SOURCE_READER_MEDIASOURCE: DWORD = 0xffffffff;
pub const MF_SOURCE_READER_CURRENT_TYPE_INDEX: DWORD = 0xffffffff;
RIDL!{#[uuid(0x70ae66f2, 0xc809, 0x4e4f, 0x89, 0x15, 0xbd, 0xcb, 0x40, 0x6b, 0x79, 0x93)]
interface IMFSourceReader(IMFSourceReaderVtbl): IUnknown(IUnknownVtbl) {
    fn GetStreamSelection(
        dwStreamIndex: DWORD,
        pfSelected: *mut BOOL,
    ) -> HRESULT,
    fn SetStreamSelection(
        dwStreamIndex: DWORD,
        fSelected: BOOL,
    ) -> HRESULT,
    fn GetNativeMediaType(
        dwStreamIndex: DWORD,
        dwMediaTypeIndex: DWORD,
        ppMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetCurrentMediaType(
        dwStreamIndex: DWORD,
        ppMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn SetCurrentMediaType(
        dwStreamIndex: DWORD,
        pdwReserved: *mut DWORD,
        pMediaType: *mut IMFMediaType,
    ) -> HRESULT,
    fn SetCurrentPosition(
        guidTimeFormat: REFGUID,
        varPosition: REFPROPVARIANT,
    ) -> HRESULT,
    fn ReadSample(
        dwStreamIndex: DWORD,
        dwControlFlags: DWORD,
        pdwActualStreamIndex: *mut DWORD,
        pdwStreamFlags: *mut DWORD,
        pllTimestamp: *mut LONGLONG,
        ppSample: *mut *mut IMFSample,
    ) -> HRESULT,
    fn Flush(
        dwStreamIndex: DWORD,
    ) -> HRESULT,
    fn GetServiceForStream(
        dwStreamIndex: DWORD,
        guidService: REFGUID,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
    fn GetPresentationAttribute(
        dwStreamIndex: DWORD,
        guidAttribute: REFGUID,
        pvarAttribute: *mut PROPVARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7b981cf0, 0x560e, 0x4116, 0x98, 0x75, 0xb0, 0x99, 0x89, 0x5f, 0x23, 0xd7)]
interface IMFSourceReaderEx(IMFSourceReaderExVtbl): IMFSourceReader(IMFSourceReaderVtbl) {
    fn SetNativeMediaType(
        dwStreamIndex: DWORD,
        pMediaType: *mut IMFMediaType,
        pdwStreamFlags: *mut DWORD,
    ) -> HRESULT,
    fn AddTransformForStream(
        dwStreamIndex: DWORD,
        pTransformOrActivate: *mut IUnknown,
    ) -> HRESULT,
    fn RemoveAllTransformsForStream(
        dwStreamIndex: DWORD,
    ) -> HRESULT,
    fn GetTransformForStream(
        dwStreamIndex: DWORD,
        dwTransformIndex: DWORD,
        pGuidCategory: *mut GUID,
        ppTransform: *mut *mut IMFTransform,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdeec8d99, 0xfa1d, 0x4d82, 0x84, 0xc2, 0x2c, 0x89, 0x69, 0x94, 0x48, 0x67)]
interface IMFSourceReaderCallback(IMFSourceReaderCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn OnReadSample(
        hrStatus: HRESULT,
        dwStreamIndex: DWORD,
        dwStreamFlags: DWORD,
        llTimestamp: LONGLONG,
        pSample: *mut IMFSample,
    ) -> HRESULT,
    fn OnFlush(
        dwStreamIndex: DWORD,
    ) -> HRESULT,
    fn OnEvent(
        dwStreamIndex: DWORD,
        pEvent: *mut IMFMediaEvent,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xCF839FE6, 0x8C2A, 0x4DD2, 0xB6, 0xEA, 0xC2, 0x2D, 0x69, 0x61, 0xAF, 0x05)]
interface IMFSourceReaderCallback2(IMFSourceReaderCallback2Vtbl):
  IMFSourceReaderCallback(IMFSourceReaderCallbackVtbl) {
    fn OnTransformChange() -> HRESULT,
    fn OnStreamError(
        dwStreamIndex: DWORD,
        hrStatus: HRESULT,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_MFSinkWriter,
    0xa3bbfb17, 0x8273, 0x4e52, 0x9e, 0x0e, 0x97, 0x39, 0xdc, 0x88, 0x79, 0x90}
extern "system" {
    pub fn MFCreateSinkWriterFromURL(
        pwszOutputURL: LPCWSTR,
        pByteStream: *mut IMFByteStream,
        pAttributes: *mut IMFAttributes,
        ppSinkWriter: *mut *mut IMFSinkWriter,
    ) -> HRESULT;
    pub fn MFCreateSinkWriterFromMediaSink(
        pMediaSink: *mut IMFMediaSink,
        pAttributes: *mut IMFAttributes,
        ppSinkWriter: *mut *mut IMFSinkWriter,
    ) -> HRESULT;
}
DEFINE_GUID!{MF_SINK_WRITER_ASYNC_CALLBACK,
    0x48cb183e, 0x7b0b, 0x46f4, 0x82, 0x2e, 0x5e, 0x1d, 0x2d, 0xda, 0x43, 0x54}
DEFINE_GUID!{MF_SINK_WRITER_DISABLE_THROTTLING,
    0x08b845d8, 0x2b74, 0x4afe, 0x9d, 0x53, 0xbe, 0x16, 0xd2, 0xd5, 0xae, 0x4f}
DEFINE_GUID!{MF_SINK_WRITER_D3D_MANAGER,
    0xec822da2, 0xe1e9, 0x4b29, 0xa0, 0xd8, 0x56, 0x3c, 0x71, 0x9f, 0x52, 0x69}
DEFINE_GUID!{MF_SINK_WRITER_ENCODER_CONFIG,
    0xad91cd04, 0xa7cc, 0x4ac7, 0x99, 0xb6, 0xa5, 0x7b, 0x9a, 0x4a, 0x7c, 0x70}
pub const MF_SINK_WRITER_INVALID_STREAM_INDEX: DWORD = 0xffffffff;
pub const MF_SINK_WRITER_ALL_STREAMS: DWORD = 0xfffffffe;
pub const MF_SINK_WRITER_MEDIASINK: DWORD = 0xffffffff;
STRUCT!{struct MF_SINK_WRITER_STATISTICS {
    cb: DWORD,
    llLastTimestampReceived: LONGLONG,
    llLastTimestampEncoded: LONGLONG,
    llLastTimestampProcessed: LONGLONG,
    llLastStreamTickReceived: LONGLONG,
    llLastSinkSampleRequest: LONGLONG,
    qwNumSamplesReceived: QWORD,
    qwNumSamplesEncoded: QWORD,
    qwNumSamplesProcessed: QWORD,
    qwNumStreamTicksReceived: QWORD,
    dwByteCountQueued: DWORD,
    qwByteCountProcessed: QWORD,
    dwNumOutstandingSinkSampleRequests: DWORD,
    dwAverageSampleRateReceived: DWORD,
    dwAverageSampleRateEncoded: DWORD,
    dwAverageSampleRateProcessed: DWORD,
}}
RIDL!{#[uuid(0x3137f1cd, 0xfe5e, 0x4805, 0xa5, 0xd8, 0xfb, 0x47, 0x74, 0x48, 0xcb, 0x3d)]
interface IMFSinkWriter(IMFSinkWriterVtbl): IUnknown(IUnknownVtbl) {
    fn AddStream(
        pTargetMediaType: *mut IMFMediaType,
        pdwStreamIndex: *mut DWORD,
    ) -> HRESULT,
    fn SetInputMediaType(
        dwStreamIndex: DWORD,
        pInputMediaType: *mut IMFMediaType,
        pEncodingParameters: *mut IMFAttributes,
    ) -> HRESULT,
    fn BeginWriting() -> HRESULT,
    fn WriteSample(
        dwStreamIndex: DWORD,
        pSample: *mut IMFSample,
    ) -> HRESULT,
    fn SendStreamTick(
        dwStreamIndex: DWORD,
        llTimestamp: LONGLONG,
    ) -> HRESULT,
    fn PlaceMarker(
        dwStreamIndex: DWORD,
        pvContext: LPVOID,
    ) -> HRESULT,
    fn NotifyEndOfSegment(
        dwStreamIndex: DWORD,
    ) -> HRESULT,
    fn Flush(
        dwStreamIndex: DWORD,
    ) -> HRESULT,
    fn Finalize() -> HRESULT,
    fn GetServiceForStream(
        dwStreamIndex: DWORD,
        guidService: REFGUID,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
    fn GetStatistics(
        dwStreamIndex: DWORD,
        pStats: *mut MF_SINK_WRITER_STATISTICS,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x588d72ab, 0x5Bc1, 0x496a, 0x87, 0x14, 0xb7, 0x06, 0x17, 0x14, 0x1b, 0x25)]
interface IMFSinkWriterEx(IMFSinkWriterExVtbl): IMFSinkWriter(IMFSinkWriterVtbl) {
    fn GetTransformForStream(
        dwStreamIndex: DWORD,
        dwTransformIndex: DWORD,
        pGuidCategory: *mut GUID,
        ppTransform: *mut *mut IMFTransform,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x17C3779E, 0x3CDE, 0x4EDE, 0x8C, 0x60, 0x38, 0x99, 0xF5, 0xF5, 0x3A, 0xD6)]
interface IMFSinkWriterEncoderConfig(IMFSinkWriterEncoderConfigVtbl): IUnknown(IUnknownVtbl) {
    fn SetTargetMediaType(
        dwStreamIndex: DWORD,
        pTargetMediaType: *mut IMFMediaType,
        pEncodingParameters: *mut IMFAttributes,
    ) -> HRESULT,
    fn PlaceEncodingParameters(
        dwStreamIndex: DWORD,
        pEncodingParameters: *mut IMFAttributes,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x666f76de, 0x33d2, 0x41b9, 0xa4, 0x58, 0x29, 0xed, 0x0a, 0x97, 0x2c, 0x58)]
interface IMFSinkWriterCallback(IMFSinkWriterCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn OnFinalize(
        hrStatus: HRESULT,
    ) -> HRESULT,
    fn OnMarker(
        dwStreamIndex: DWORD,
        pvContext: LPVOID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2456BD58, 0xC067, 0x4513, 0x84, 0xFE, 0x8D, 0x0C, 0x88, 0xFF, 0xDC, 0x61)]
interface IMFSinkWriterCallback2(IMFSinkWriterCallback2Vtbl):
  IMFSinkWriterCallback(IMFSinkWriterCallbackVtbl) {
    fn OnTransformChange() -> HRESULT,
    fn OnStreamError(
        dwStreamIndex: DWORD,
        hrStatus: HRESULT,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_READWRITE_DISABLE_CONVERTERS,
    0x98d5b065, 0x1374, 0x4847, 0x8d, 0x5d, 0x31, 0x52, 0x0f, 0xee, 0x71, 0x56}
DEFINE_GUID!{MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS,
    0xa634a91c, 0x822b, 0x41b9, 0xa4, 0x94, 0x4d, 0xe4, 0x64, 0x36, 0x12, 0xb0}
DEFINE_GUID!{MF_READWRITE_MMCSS_CLASS,
    0x39384300, 0xd0eb, 0x40b1, 0x87, 0xa0, 0x33, 0x18, 0x87, 0x1b, 0x5a, 0x53}
DEFINE_GUID!{MF_READWRITE_MMCSS_PRIORITY,
    0x43ad19ce, 0xf33f, 0x4ba9, 0xa5, 0x80, 0xe4, 0xcd, 0x12, 0xf2, 0xd1, 0x44}
DEFINE_GUID!{MF_READWRITE_MMCSS_CLASS_AUDIO,
    0x430847da, 0x0890, 0x4b0e, 0x93, 0x8c, 0x05, 0x43, 0x32, 0xc5, 0x47, 0xe1}
DEFINE_GUID!{MF_READWRITE_MMCSS_PRIORITY_AUDIO,
    0x273db885, 0x2de2, 0x4db2, 0xa6, 0xa7, 0xfd, 0xb6, 0x6f, 0xb4, 0x0b, 0x61}
DEFINE_GUID!{MF_READWRITE_D3D_OPTIONAL,
    0x216479d9, 0x3071, 0x42ca, 0xbb, 0x6c, 0x4c, 0x22, 0x10, 0x2e, 0x1d, 0x18}
DEFINE_GUID!{MF_MEDIASINK_AUTOFINALIZE_SUPPORTED,
    0x48c131be, 0x135a, 0x41cb, 0x82, 0x90, 0x03, 0x65, 0x25, 0x09, 0xc9, 0x99}
DEFINE_GUID!{MF_MEDIASINK_ENABLE_AUTOFINALIZE,
    0x34014265, 0xcb7e, 0x4cde, 0xac, 0x7c, 0xef, 0xfd, 0x3b, 0x3c, 0x25, 0x30}
DEFINE_GUID!{MF_READWRITE_ENABLE_AUTOFINALIZE,
    0xdd7ca129, 0x8cd1, 0x4dc5, 0x9d, 0xde, 0xce, 0x16, 0x86, 0x75, 0xde, 0x61}
