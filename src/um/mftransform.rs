// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of mftransform.h
use shared::basetsd::{UINT32, ULONG_PTR};
use shared::guiddef::GUID;
use shared::minwindef::DWORD;
use shared::wtypes::PROPERTYKEY;
use um::mfobjects::{
    IMFActivate, IMFAttributes, IMFCollection, IMFMediaEvent, IMFMediaType,
    IMFSample, MF_STREAM_STATE
};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LONGLONG, MAXLONGLONG};
ENUM!{enum MFT_INPUT_DATA_BUFFER_FLAGS {
    MFT_INPUT_DATA_BUFFER_PLACEHOLDER = -1i32 as u32,
}}
ENUM!{enum MFT_OUTPUT_DATA_BUFFER_FLAGS {
    MFT_OUTPUT_DATA_BUFFER_INCOMPLETE = 0x100_0000,
    MFT_OUTPUT_DATA_BUFFER_FORMAT_CHANGE = 0x100,
    MFT_OUTPUT_DATA_BUFFER_STREAM_END = 0x200,
    MFT_OUTPUT_DATA_BUFFER_NO_SAMPLE = 0x300,
}}
ENUM!{enum MFT_INPUT_STATUS_FLAGS {
    MFT_INPUT_STATUS_ACCEPT_DATA = 0x1,
}}
ENUM!{enum FT_OUTPUT_STATUS_FLAGS {
    MFT_OUTPUT_STATUS_SAMPLE_READY = 0x1,
}}
ENUM!{enum MFT_INPUT_STREAM_INFO_FLAGS {
    MFT_INPUT_STREAM_WHOLE_SAMPLES = 0x1,
    MFT_INPUT_STREAM_SINGLE_SAMPLE_PER_BUFFER = 0x2,
    MFT_INPUT_STREAM_FIXED_SAMPLE_SIZE = 0x4,
    MFT_INPUT_STREAM_HOLDS_BUFFERS = 0x8,
    MFT_INPUT_STREAM_DOES_NOT_ADDREF = 0x100,
    MFT_INPUT_STREAM_REMOVABLE = 0x200,
    MFT_INPUT_STREAM_OPTIONAL = 0x400,
    MFT_INPUT_STREAM_PROCESSES_IN_PLACE = 0x800,
}}
ENUM!{enum MFT_OUTPUT_STREAM_INFO_FLAGS {
    MFT_OUTPUT_STREAM_WHOLE_SAMPLES = 0x1,
    MFT_OUTPUT_STREAM_SINGLE_SAMPLE_PER_BUFFER = 0x2,
    MFT_OUTPUT_STREAM_FIXED_SAMPLE_SIZE = 0x4,
    MFT_OUTPUT_STREAM_DISCARDABLE = 0x8,
    MFT_OUTPUT_STREAM_OPTIONAL = 0x10,
    MFT_OUTPUT_STREAM_PROVIDES_SAMPLES = 0x100,
    MFT_OUTPUT_STREAM_CAN_PROVIDE_SAMPLES = 0x200,
    MFT_OUTPUT_STREAM_LAZY_READ = 0x400,
    MFT_OUTPUT_STREAM_REMOVABLE = 0x800,
}}
ENUM!{enum MFT_SET_TYPE_FLAGS {
    MFT_SET_TYPE_TEST_ONLY = 0x1,
}}
ENUM!{enum MFT_PROCESS_OUTPUT_FLAGS {
    MFT_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER = 0x1,
    MFT_PROCESS_OUTPUT_REGENERATE_LAST_OUTPUT = 0x2,
}}
ENUM!{enum MFT_PROCESS_OUTPUT_STATUS {
    MFT_PROCESS_OUTPUT_STATUS_NEW_STREAMS = 0x100,
}}
ENUM!{enum MFT_DRAIN_TYPE {
    MFT_DRAIN_PRODUCE_TAILS = 0,
    MFT_DRAIN_NO_TAILS = 0x1,
}}
pub const MFT_STREAMS_UNLIMITED: DWORD = -1i32 as DWORD;
pub const MFT_OUTPUT_BOUND_LOWER_UNBOUNDED: LONGLONG = !MAXLONGLONG;
pub const MFT_OUTPUT_BOUND_UPPER_UNBOUNDED: LONGLONG = MAXLONGLONG;
ENUM!{enum MFT_MESSAGE_TYPE {
    MFT_MESSAGE_COMMAND_FLUSH = 0x0,
    MFT_MESSAGE_COMMAND_DRAIN = 0x1,
    MFT_MESSAGE_SET_D3D_MANAGER = 0x2,
    MFT_MESSAGE_DROP_SAMPLES = 0x3,
    MFT_MESSAGE_COMMAND_TICK = 0x4,
    MFT_MESSAGE_NOTIFY_BEGIN_STREAMING = 0x1000_0000,
    MFT_MESSAGE_NOTIFY_END_STREAMING = 0x1000_0001,
    MFT_MESSAGE_NOTIFY_END_OF_STREAM = 0x1000_0002,
    MFT_MESSAGE_NOTIFY_START_OF_STREAM = 0x1000_0003,
    MFT_MESSAGE_NOTIFY_RELEASE_RESOURCES = 0x1000_0004,
    MFT_MESSAGE_NOTIFY_REACQUIRE_RESOURCES = 0x1000_0005,
    MFT_MESSAGE_NOTIFY_EVENT = 0x1000_0006,
    MFT_MESSAGE_COMMAND_SET_OUTPUT_STREAM_STATE = 0x1000_0007,
    MFT_MESSAGE_COMMAND_FLUSH_OUTPUT_STREAM = 0x1000_0008,
    MFT_MESSAGE_COMMAND_MARKER = 0x2000_0000,
}}
STRUCT!{struct MFT_INPUT_STREAM_INFO {
    hnsMaxLatency: LONGLONG,
    dwFlags: DWORD,
    cbSize: DWORD,
    cbMaxLookahead: DWORD,
    cbAlignment: DWORD,
}}
STRUCT!{struct MFT_OUTPUT_STREAM_INFO {
    dwFlags: DWORD,
    cbSize: DWORD,
    cbAlignment: DWORD,
}}
STRUCT!{struct MFT_OUTPUT_DATA_BUFFER {
    dwStreamID: DWORD,
    pSample: *mut IMFSample,
    dwStatus: DWORD,
    pEvents: *mut IMFCollection,
}}
pub type PMFT_OUTPUT_DATA_BUFFER = *mut MFT_OUTPUT_DATA_BUFFER;
RIDL!{#[uuid(0xbf94c121, 0x5b05, 0x4e6f, 0x80, 0x00, 0xba, 0x59, 0x89, 0x61, 0x41, 0x4d)]
interface IMFTransform(IMFTransformVtbl): IUnknown(IUnknownVtbl) {
    fn GetStreamLimits( 
        pdwInputMinimum: *mut DWORD,
        pdwInputMaximum: *mut DWORD,
        pdwOutputMinimum: *mut DWORD,
        pdwOutputMaximum: *mut DWORD,
    ) -> HRESULT,
    fn GetStreamCount( 
        pcInputStreams: *mut DWORD,
        pcOutputStreams: *mut DWORD,
    ) -> HRESULT,
    fn GetStreamIDs( 
        dwInputIDArraySize: DWORD,
        pdwInputIDs: *mut DWORD,
        dwOutputIDArraySize: DWORD,
        pdwOutputIDs: *mut DWORD,
    ) -> HRESULT,
    fn GetInputStreamInfo( 
        dwInputStreamID: DWORD,
        pStreamInfo: *mut MFT_INPUT_STREAM_INFO,
    ) -> HRESULT,
    fn GetOutputStreamInfo( 
        dwOutputStreamID: DWORD,
        pStreamInfo: *mut MFT_OUTPUT_STREAM_INFO,
    ) -> HRESULT,
    fn GetAttributes( 
        pAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT,
    fn GetInputStreamAttributes( 
        dwInputStreamID: DWORD,
        pAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT,
    fn GetOutputStreamAttributes( 
        dwOutputStreamID: DWORD,
        pAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT,
    fn DeleteInputStream( 
        dwStreamID: DWORD,
    ) -> HRESULT,
    fn AddInputStreams( 
        cStreams: DWORD,
        adwStreamIDs: *mut DWORD,
    ) -> HRESULT,
    fn GetInputAvailableType( 
        dwInputStreamID: DWORD,
        dwTypeIndex: DWORD,
        ppType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetOutputAvailableType( 
        dwOutputStreamID: DWORD,
        dwTypeIndex: DWORD,
        ppType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn SetInputType( 
        dwInputStreamID: DWORD,
        pType: *mut IMFMediaType,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetOutputType( 
        dwOutputStreamID: DWORD,
        pType: *mut IMFMediaType,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetInputCurrentType( 
        dwInputStreamID: DWORD,
        ppType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetOutputCurrentType( 
        dwOutputStreamID: DWORD,
        ppType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetInputStatus( 
        dwInputStreamID: DWORD,
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
    fn GetOutputStatus( 
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
    fn SetOutputBounds( 
        hnsLowerBound: LONGLONG,
        hnsUpperBound: LONGLONG,
    ) -> HRESULT,
    fn ProcessEvent( 
        dwInputStreamID: DWORD,
        pEvent: *mut IMFMediaEvent,
    ) -> HRESULT,
    fn ProcessMessage( 
        eMessage: MFT_MESSAGE_TYPE,
        ulParam: ULONG_PTR,
    ) -> HRESULT,
    fn ProcessInput( 
        dwInputStreamID: DWORD,
        pSample: *mut IMFSample,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn ProcessOutput( 
        dwFlags: DWORD,
        cOutputBufferCount: DWORD,
        pOutputSamples: *mut MFT_OUTPUT_DATA_BUFFER,
        pdwStatus: *mut DWORD,
    ) -> HRESULT,
}}
ENUM!{enum DeviceStreamState {
    DeviceStreamState_Stop,
    DeviceStreamState_Pause,
    DeviceStreamState_Run,
    DeviceStreamState_Disabled,
}}
pub type PDeviceStreamState = *mut DeviceStreamState;
DEFINE_GUID!{MEDeviceStreamCreated,
    0x0252a1cf, 0x3540, 0x43b4, 0x91, 0x64, 0xd7, 0x2e, 0xb4, 0x05, 0xfa, 0x40}
STRUCT!{struct STREAM_MEDIUM {
    gidMedium: GUID,
    unMediumInstance: UINT32,
}}
pub type PSTREAM_MEDIUM = *mut STREAM_MEDIUM;
DEFINE_PROPERTYKEY!{MFPKEY_CLSID,
    0xc57a84c0, 0x1a80, 0x40a3, 0x97, 0xb5, 0x92, 0x72, 0xa4, 0x03, 0xc8, 0xae, 1}
DEFINE_PROPERTYKEY!{MFPKEY_CATEGORY,
    0xc57a84c0, 0x1a80, 0x40a3, 0x97, 0xb5, 0x92, 0x72, 0xa4, 0x03, 0xc8, 0xae, 2}
DEFINE_PROPERTYKEY!{MFPKEY_EXATTRIBUTE_SUPPORTED,
    0x456fe843, 0x3c87, 0x40c0, 0x94, 0x9d, 0x14, 0x09, 0xc9, 0x7d, 0xab, 0x2c, 1}
DEFINE_PROPERTYKEY!{MFPKEY_MULTICHANNEL_CHANNEL_MASK,
    0x58bdaf8c, 0x3224, 0x4692, 0x86, 0xd0, 0x44, 0xd6, 0x5c, 0x5b, 0xf8, 0x2b, 1}
DEFINE_GUID!{MF_SA_D3D_AWARE,
    0xeaa35c29, 0x775e, 0x488e, 0x9b, 0x61, 0xb3, 0x28, 0x3e, 0x49, 0x58, 0x3b}
DEFINE_GUID!{MF_SA_REQUIRED_SAMPLE_COUNT, 
    0x18802c61, 0x324b, 0x4952, 0xab, 0xd0, 0x17, 0x6f, 0xf5, 0xc6, 0x96, 0xff}
DEFINE_GUID!{MFT_END_STREAMING_AWARE, 
    0x70fbc845, 0xb07e, 0x4089, 0xb0, 0x64, 0x39, 0x9d, 0xc6, 0x11, 0x0f, 0x29}
DEFINE_GUID!{MF_SA_AUDIO_ENDPOINT_AWARE, 
    0xc0381701, 0x805c, 0x42b2, 0xac, 0x8d, 0xe2, 0xb4, 0xbf, 0x21, 0xf4, 0xf8}
DEFINE_GUID!{MFT_AUDIO_DECODER_AUDIO_ENDPOINT_ID, 
    0xc7ccdd6e, 0x5398, 0x4695, 0x8b, 0xe7, 0x51, 0xb3, 0xe9, 0x51, 0x11, 0xbd}
DEFINE_GUID!{MFT_AUDIO_DECODER_SPATIAL_METADATA_CLIENT, 
    0x05987df4, 0x1270, 0x4999, 0x92, 0x5f, 0x8e, 0x93, 0x9a, 0x7c, 0x0a, 0xf7}
RIDL!{#[uuid(0xD818FBD8, 0xFC46, 0x42F2, 0x87, 0xAC, 0x1E, 0xA2, 0xD1, 0xF9, 0xBF, 0x32)]
interface IMFDeviceTransform(IMFDeviceTransformVtbl): IUnknown(IUnknownVtbl) {
    fn InitializeTransform( 
        pAttributes: *mut IMFAttributes,
    ) -> HRESULT,
    fn GetInputAvailableType( 
        dwInputStreamID: DWORD,
        dwTypeIndex: DWORD,
        pMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetInputCurrentType( 
        dwInputStreamID: DWORD,
        pMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetInputStreamAttributes( 
        dwInputStreamID: DWORD,
        ppAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT,
    fn GetOutputAvailableType( 
        dwOutputStreamID: DWORD,
        dwTypeIndex: DWORD,
        pMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetOutputCurrentType( 
        dwOutputStreamID: DWORD,
        pMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetOutputStreamAttributes( 
        dwOutputStreamID: DWORD,
        ppAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT,
    fn GetStreamCount( 
        pcInputStreams: *mut DWORD,
        pcOutputStreams: *mut DWORD,
    ) -> HRESULT,
    fn GetStreamIDs( 
        dwInputIDArraySize: DWORD,
        pdwInputStreamIds: *mut DWORD,
        dwOutputIDArraySize: DWORD,
        pdwOutputStreamIds: *mut DWORD,
    ) -> HRESULT,
    fn ProcessEvent( 
        dwInputStreamID: DWORD,
        pEvent: *mut IMFMediaEvent,
    ) -> HRESULT,
    fn ProcessInput( 
        dwInputStreamID: DWORD,
        pSample: *mut IMFSample,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn ProcessMessage( 
        eMessage: MFT_MESSAGE_TYPE,
        ulParam: ULONG_PTR,
    ) -> HRESULT,
    fn ProcessOutput( 
        dwFlags: DWORD,
        cOutputBufferCount: DWORD,
        pOutputSample: *mut MFT_OUTPUT_DATA_BUFFER,
        pdwStatus: *mut DWORD,
    ) -> HRESULT,
    fn SetInputStreamState( 
        dwStreamID: DWORD,
        pMediaType: *mut IMFMediaType,
        value: DeviceStreamState,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetInputStreamState( 
        dwStreamID: DWORD,
        value: *mut DeviceStreamState,
    ) -> HRESULT,
    fn SetOutputStreamState( 
        dwStreamID: DWORD,
        pMediaType: *mut IMFMediaType,
        value: DeviceStreamState,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetOutputStreamState( 
        dwStreamID: DWORD,
        value: *mut DeviceStreamState,
    ) -> HRESULT,
    fn GetInputStreamPreferredState( 
        dwStreamID: DWORD,
        value: *mut DeviceStreamState,
        ppMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn FlushInputStream( 
        dwStreamIndex: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn FlushOutputStream( 
        dwStreamIndex: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6D5CB646, 0x29EC, 0x41FB, 0x81, 0x79, 0x8C, 0x4C, 0x6D, 0x75, 0x08, 0x11)]
interface IMFDeviceTransformCallback(IMFDeviceTransformCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn OnBufferSent( 
        pCallbackAttributes: *mut IMFAttributes,
        pinId: DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{MF_SA_REQUIRED_SAMPLE_COUNT_PROGRESSIVE,
    0xb172d58e, 0xfa77, 0x4e48, 0x8d, 0x2a, 0x1d, 0xf2, 0xd8, 0x50, 0xea, 0xc2}
DEFINE_GUID!{MF_SA_MINIMUM_OUTPUT_SAMPLE_COUNT,
    0x851745d5, 0xc3d6, 0x476d, 0x95, 0x27, 0x49, 0x8e, 0xf2, 0xd1, 0x0d, 0x18}
DEFINE_GUID!{MF_SA_MINIMUM_OUTPUT_SAMPLE_COUNT_PROGRESSIVE,
    0x0f5523a5, 0x1cb2, 0x47c5, 0xa5, 0x50, 0x2e, 0xeb, 0x84, 0xb4, 0xd1, 0x4a}
DEFINE_GUID!{MFT_SUPPORT_3DVIDEO,
    0x093f81b1, 0x4f2e, 0x4631, 0x81, 0x68, 0x79, 0x34, 0x03, 0x2a, 0x01, 0xd3}
ENUM!{enum MF3DVideoOutputType {
    MF3DVideoOutputType_BaseView,
    MF3DVideoOutputType_Stereo,
}}
DEFINE_GUID!{MF_ENABLE_3DVIDEO_OUTPUT,
    0xbdad7bca, 0x0e5f, 0x4b10, 0xab, 0x16, 0x26, 0xde, 0x38, 0x1b, 0x62, 0x93}
DEFINE_GUID!{MF_SA_D3D11_BINDFLAGS,
    0xeacf97ad, 0x065c, 0x4408, 0xbe, 0xe3, 0xfd, 0xcb, 0xfd, 0x12, 0x8b, 0xe2}
DEFINE_GUID!{MF_SA_D3D11_USAGE,
    0xe85fe442, 0x2ca3, 0x486e, 0xa9, 0xc7, 0x10, 0x9d, 0xda, 0x60, 0x98, 0x80}
DEFINE_GUID!{MF_SA_D3D11_AWARE,
    0x206b4fc8, 0xfcf9, 0x4c51, 0xaf, 0xe3, 0x97, 0x64, 0x36, 0x9e, 0x33, 0xa0}
DEFINE_GUID!{MF_SA_D3D11_SHARED,
    0x7b8f32c3, 0x6d96, 0x4b89, 0x92, 0x03, 0xdd, 0x38, 0xb6, 0x14, 0x14, 0xf3}
DEFINE_GUID!{MF_SA_D3D11_SHARED_WITHOUT_MUTEX,
    0x39dbd44d, 0x2e44, 0x4931, 0xa4, 0xc8, 0x35, 0x2d, 0x3d, 0xc4, 0x21, 0x15}
DEFINE_GUID!{MF_SA_D3D11_ALLOW_DYNAMIC_YUV_TEXTURE,
    0xce06d49f, 0x0613, 0x4b9d, 0x86, 0xa6, 0xd8, 0xc4, 0xf9, 0xc1, 0x00, 0x75}
DEFINE_GUID!{MF_SA_D3D11_HW_PROTECTED,
    0x3a8ba9d9, 0x92ca, 0x4307, 0xa3, 0x91, 0x69, 0x99, 0xdb, 0xf3, 0xb6, 0xce}
DEFINE_GUID!{MF_SA_BUFFERS_PER_SAMPLE,
    0x873c5171, 0x1e3d, 0x4e25, 0x98, 0x8d, 0xb4, 0x33, 0xce, 0x04, 0x19, 0x83}
DEFINE_GUID!{MFT_DECODER_EXPOSE_OUTPUT_TYPES_IN_NATIVE_ORDER,
    0xef80833f, 0xf8fa, 0x44d9, 0x80, 0xd8, 0x41, 0xed, 0x62, 0x32, 0x67, 0x0c}
DEFINE_GUID!{MFT_DECODER_QUALITY_MANAGEMENT_CUSTOM_CONTROL,
    0xa24e30d7, 0xde25, 0x4558, 0xbb, 0xfb, 0x71, 0x07, 0x0a, 0x2d, 0x33, 0x2e}
DEFINE_GUID!{MFT_DECODER_QUALITY_MANAGEMENT_RECOVERY_WITHOUT_ARTIFACTS,
    0xd8980deb, 0x0a48, 0x425f, 0x86, 0x23, 0x61, 0x1d, 0xb4, 0x1d, 0x38, 0x10}
DEFINE_GUID!{MFT_REMUX_MARK_I_PICTURE_AS_CLEAN_POINT,
    0x364e8f85, 0x3f2e, 0x436c, 0xb2, 0xa2, 0x44, 0x40, 0xa0, 0x12, 0xa9, 0xe8}
DEFINE_GUID!{MFT_DECODER_FINAL_VIDEO_RESOLUTION_HINT,
    0xdc2f8496, 0x15c4, 0x407a, 0xb6, 0xf0, 0x1b, 0x66, 0xab, 0x5f, 0xbf, 0x53}
DEFINE_GUID!{MFT_ENCODER_SUPPORTS_CONFIG_EVENT,
    0x86a355ae, 0x3a77, 0x4ec4, 0x9f, 0x31, 0x01, 0x14, 0x9a, 0x4e, 0x92, 0xde}
DEFINE_GUID!{MFT_ENUM_HARDWARE_VENDOR_ID_Attribute,
    0x3aecb0cc, 0x035b, 0x4bcc, 0x81, 0x85, 0x2b, 0x8d, 0x55, 0x1e, 0xf3, 0xaf}
DEFINE_GUID!{MF_TRANSFORM_ASYNC,
    0xf81a699a, 0x649a, 0x497d, 0x8c, 0x73, 0x29, 0xf8, 0xfe, 0xd6, 0xad, 0x7a}
DEFINE_GUID!{MF_TRANSFORM_ASYNC_UNLOCK,
    0xe5666d6b, 0x3422, 0x4eb6, 0xa4, 0x21, 0xda, 0x7d, 0xb1, 0xf8, 0xe2, 0x07}
DEFINE_GUID!{MF_TRANSFORM_FLAGS_Attribute,
    0x9359bb7e, 0x6275, 0x46c4, 0xa0, 0x25, 0x1c, 0x01, 0xe4, 0x5f, 0x1a, 0x86}
DEFINE_GUID!{MF_TRANSFORM_CATEGORY_Attribute,
    0xceabba49, 0x506d, 0x4757, 0xa6, 0xff, 0x66, 0xc1, 0x84, 0x98, 0x7e, 0x4e}
DEFINE_GUID!{MFT_TRANSFORM_CLSID_Attribute,
    0x6821c42b, 0x65a4, 0x4e82, 0x99, 0xbc, 0x9a, 0x88, 0x20, 0x5e, 0xcd, 0x0c}
DEFINE_GUID!{MFT_INPUT_TYPES_Attributes,
    0x4276c9b1, 0x759d, 0x4bf3, 0x9c, 0xd0, 0x0d, 0x72, 0x3d, 0x13, 0x8f, 0x96}
DEFINE_GUID!{MFT_OUTPUT_TYPES_Attributes,
    0x8eae8cf3, 0xa44f, 0x4306, 0xba, 0x5c, 0xbf, 0x5d, 0xda, 0x24, 0x28, 0x18}
DEFINE_GUID!{MFT_ENUM_HARDWARE_URL_Attribute,
    0x2fb866ac, 0xb078, 0x4942, 0xab, 0x6c, 0x00, 0x3d, 0x05, 0xcd, 0xa6, 0x74}
DEFINE_GUID!{MFT_FRIENDLY_NAME_Attribute,
    0x314ffbae, 0x5b41, 0x4c95, 0x9c, 0x19, 0x4e, 0x7d, 0x58, 0x6f, 0xac, 0xe3}
DEFINE_GUID!{MFT_CONNECTED_STREAM_ATTRIBUTE,
    0x71eeb820, 0xa59f, 0x4de2, 0xbc, 0xec, 0x38, 0xdb, 0x1d, 0xd6, 0x11, 0xa4}
DEFINE_GUID!{MFT_CONNECTED_TO_HW_STREAM,
    0x34e6e728, 0x06d6, 0x4491, 0xa5, 0x53, 0x47, 0x95, 0x65, 0x0d, 0xb9, 0x12}
DEFINE_GUID!{MFT_PREFERRED_OUTPUTTYPE_Attribute,
    0x7e700499, 0x396a, 0x49ee, 0xb1, 0xb4, 0xf6, 0x28, 0x02, 0x1e, 0x8c, 0x9d}
DEFINE_GUID!{MFT_PROCESS_LOCAL_Attribute,
    0x543186e4, 0x4649, 0x4e65, 0xb5, 0x88, 0x4a, 0xa3, 0x52, 0xaf, 0xf3, 0x79}
DEFINE_GUID!{MFT_PREFERRED_ENCODER_PROFILE,
    0x53004909, 0x1ef5, 0x46d7, 0xa1, 0x8e, 0x5a, 0x75, 0xf8, 0xb5, 0x90, 0x5f}
DEFINE_GUID!{MFT_HW_TIMESTAMP_WITH_QPC_Attribute,
    0x8d030fb8, 0xcc43, 0x4258, 0xa2, 0x2e, 0x92, 0x10, 0xbe, 0xf8, 0x9b, 0xe4}
DEFINE_GUID!{MFT_FIELDOFUSE_UNLOCK_Attribute,
    0x8ec2e9fd, 0x9148, 0x410d, 0x83, 0x1e, 0x70, 0x24, 0x39, 0x46, 0x1a, 0x8e}
DEFINE_GUID!{MFT_CODEC_MERIT_Attribute,
    0x88a7cb15, 0x7b07, 0x4a34, 0x91, 0x28, 0xe6, 0x4c, 0x67, 0x03, 0xc4, 0xd3}
DEFINE_GUID!{MFT_ENUM_TRANSCODE_ONLY_ATTRIBUTE,
    0x111ea8cd, 0xb62a, 0x4bdb, 0x89, 0xf6, 0x67, 0xff, 0xcd, 0xc2, 0x45, 0x8b}
extern "system" {
    pub fn MFCreateTransformActivate(
        ppActivate: *mut *mut IMFActivate,
    ) -> HRESULT;
}
DEFINE_GUID!{MFT_AUDIO_DECODER_DEGRADATION_INFO_ATTRIBUTE,
    0x6c3386ad, 0xec20, 0x430d, 0xb2, 0xa5, 0x50, 0x5c, 0x71, 0x78, 0xd9, 0xc4}
ENUM!{enum MFT_AUDIO_DECODER_DEGRADATION_REASON {
    MFT_AUDIO_DECODER_DEGRADATION_REASON_NONE,
    MFT_AUDIO_DECODER_DEGRADATION_REASON_LICENSING_REQUIREMENT,
}}
ENUM!{enum MFT_AUDIO_DECODER_DEGRADATION_TYPE {
    MFT_AUDIO_DECODER_DEGRADATION_TYPE_NONE,
    MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX2CHANNEL,
    MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX6CHANNEL,
    MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX8CHANNEL,
}}
STRUCT!{struct MFAudioDecoderDegradationInfo {
    eDegradationReason: MFT_AUDIO_DECODER_DEGRADATION_REASON,
    eType: MFT_AUDIO_DECODER_DEGRADATION_TYPE,
}}
STRUCT!{struct MFT_STREAM_STATE_PARAM {
    StreamId: DWORD,
    State: MF_STREAM_STATE,
}}
pub type PMFT_STREAM_STATE_PARAM = *mut MFT_STREAM_STATE_PARAM;
