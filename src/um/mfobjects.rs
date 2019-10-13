// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of mfobjects.h
use ctypes::{c_double, c_short, c_void};
use shared::basetsd::{UINT32, UINT64, UINT8};
use shared::guiddef::{CLSID, GUID, REFCLSID, REFGUID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, LPVOID, UINT, ULONG, WORD};
use shared::mmreg::WAVEFORMATEX;
use shared::windef::SIZE;
use shared::wtypes::{
    VT_CLSID, VT_LPWSTR, VT_R8, VT_UI1, VT_UI4, VT_UI8, VT_UNKNOWN, VT_VECTOR
};
use um::objidlbase::IStream;
use um::propidl::{PROPVARIANT, REFPROPVARIANT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HANDLE, HRESULT, LONG, LONGLONG, LPCWSTR, LPWSTR, ULONGLONG};
pub type QWORD = ULONGLONG;
ENUM!{enum MF_ATTRIBUTE_TYPE {
    MF_ATTRIBUTE_UINT32	= VT_UI4,
    MF_ATTRIBUTE_UINT64	= VT_UI8,
    MF_ATTRIBUTE_DOUBLE	= VT_R8,
    MF_ATTRIBUTE_GUID	= VT_CLSID,
    MF_ATTRIBUTE_STRING	= VT_LPWSTR,
    MF_ATTRIBUTE_BLOB	= VT_VECTOR | VT_UI1,
    MF_ATTRIBUTE_IUNKNOWN = VT_UNKNOWN,
}}
ENUM!{enum MF_ATTRIBUTES_MATCH_TYPE {
    MF_ATTRIBUTES_MATCH_OUR_ITEMS,
    MF_ATTRIBUTES_MATCH_THEIR_ITEMS,
    MF_ATTRIBUTES_MATCH_ALL_ITEMS,
    MF_ATTRIBUTES_MATCH_INTERSECTION,
    MF_ATTRIBUTES_MATCH_SMALLER,
}}
RIDL!{#[uuid(0x2cd2d921, 0xc447, 0x44a7, 0xa1, 0x3c, 0x4a, 0xda, 0xbf, 0xc2, 0x47, 0xe3)]
interface IMFAttributes(IMFAttributesVtbl): IUnknown(IUnknownVtbl) {
        fn GetItem(
            guidKey: REFGUID,
            pValue: *mut PROPVARIANT,
        ) -> HRESULT,
        fn GetItemType(
            guidKey: REFGUID,
            pType: *mut MF_ATTRIBUTE_TYPE,
        ) -> HRESULT,
        fn CompareItem(
            guidKey: REFGUID,
            Value: REFPROPVARIANT,
            pbResult: *mut BOOL,
        ) -> HRESULT,
        fn Compare(
            pTheirs: *mut IMFAttributes,
            MatchType: MF_ATTRIBUTES_MATCH_TYPE,
            pbResult: *mut BOOL,
        ) -> HRESULT,
        fn GetUINT32(
            guidKey: REFGUID,
            punValue: *mut UINT32,
        ) -> HRESULT,
        fn GetUINT64(
            guidKey: REFGUID,
            punValue: *mut UINT64,
        ) -> HRESULT,
        fn GetDouble(
            guidKey: REFGUID,
            pfValue: *mut c_double,
        ) -> HRESULT,
        fn GetGUID(
            guidKey: REFGUID,
            pguidValue: *mut GUID,
        ) -> HRESULT,
        fn GetStringLength(
            guidKey: REFGUID,
            pcchLength: *mut UINT32,
        ) -> HRESULT,
        fn GetString(
            guidKey: REFGUID,
            pwszValue: LPWSTR,
            cchBufSize: UINT32,
            pcchLength: *mut UINT32,
        ) -> HRESULT,
        fn GetAllocatedString(
            guidKey: REFGUID,
            ppwszValue: *mut LPWSTR,
            pcchLength: *mut UINT32,
        ) -> HRESULT,
        fn GetBlobSize(
            guidKey: REFGUID,
            pcbBlobSize: *mut UINT32,
        ) -> HRESULT,
        fn GetBlob(
            guidKey: REFGUID,
            pBuf: *mut UINT8,
            cbBufSize: UINT32,
            pcbBlobSize: *mut UINT32,
        ) -> HRESULT,
        fn GetAllocatedBlob(
            guidKey: REFGUID,
            ppBuf: *mut *mut UINT8,
            pcbSize: *mut UINT32,
        ) -> HRESULT,
        fn GetUnknown(
            guidKey: REFGUID,
            riid: REFIID,
            ppv: *mut LPVOID,
        ) -> HRESULT,
        fn SetItem(
            guidKey: REFGUID,
            Value: REFPROPVARIANT,
        ) -> HRESULT,
        fn DeleteItem(
            guidKey: REFGUID,
        ) -> HRESULT,
        fn DeleteAllItems() -> HRESULT,
        fn SetUINT32(
            guidKey: REFGUID,
            unValue: UINT32,
        ) -> HRESULT,
        fn SetUINT64(
            guidKey: REFGUID,
            unValue: UINT64,
        ) -> HRESULT,
        fn SetDouble(
            guidKey: REFGUID,
            fValue: c_double,
        ) -> HRESULT,
        fn SetGUID(
            guidKey: REFGUID,
            guidValue: REFGUID,
        ) -> HRESULT,
        fn SetString(
            guidKey: REFGUID,
            wszValue: LPCWSTR,
        ) -> HRESULT,
        fn SetBlob(
            guidKey: REFGUID,
            pBuf: *const UINT8,
            cbBufSize: UINT32,
        ) -> HRESULT,
        fn SetUnknown(
            guidKey: REFGUID,
            pUnknown: *mut IUnknown,
        ) -> HRESULT,
        fn LockStore() -> HRESULT,
        fn UnlockStore() -> HRESULT,
        fn GetCount(
            pcItems: *mut UINT32,
        ) -> HRESULT,
        fn GetItemByIndex(
            unIndex: UINT32,
            pguidKey: *mut GUID,
            pValue: *mut PROPVARIANT,
        ) -> HRESULT,
        fn CopyAllItems(
            pDest: *mut IMFAttributes,
        ) -> HRESULT,
}}
pub const MF_ATTRIBUTE_SERIALIZE_UNKNOWN_BYREF: DWORD = 0x1;
extern "system" {
    pub fn MFSerializeAttributesToStream(
        pAttr: *mut IMFAttributes,
        dwOptions: DWORD,
        pStm: *mut IStream,
    ) -> HRESULT;
    pub fn MFDeserializeAttributesFromStream(
        pAttr: *mut IMFAttributes,
        dwOptions: DWORD,
        pStm: *mut IStream,
    ) -> HRESULT;
}
RIDL!{#[uuid(0x045FA593, 0x8799, 0x42b8, 0xBC, 0x8D, 0x89, 0x68, 0xC6, 0x45, 0x35, 0x07)]
interface IMFMediaBuffer(IMFMediaBufferVtbl): IUnknown(IUnknownVtbl) {
    fn Lock(
        ppbBuffer: *mut *mut BYTE,
        pcbMaxLength: *mut DWORD,
        pcbCurrentLength: *mut DWORD,
    ) -> HRESULT,
    fn Unlock() -> HRESULT,
    fn GetCurrentLength(
        pcbCurrentLength: *mut DWORD,
    ) -> HRESULT,
    fn SetCurrentLength(
        cbCurrentLength: DWORD,
    ) -> HRESULT,
    fn GetMaxLength(
        pcbMaxLength: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc40a00f2, 0xb93a, 0x4d80, 0xae, 0x8c, 0x5a, 0x1c, 0x63, 0x4f, 0x58, 0xe4)]
interface IMFSample(IMFSampleVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetSampleFlags(
        pdwSampleFlags: *mut DWORD,
    ) -> HRESULT,
    fn SetSampleFlags(
        dwSampleFlags: DWORD,
    ) -> HRESULT,
    fn GetSampleTime(
        phnsSampleTime: *mut LONGLONG,
    ) -> HRESULT,
    fn SetSampleTime(
        hnsSampleTime: LONGLONG,
    ) -> HRESULT,
    fn GetSampleDuration(
        phnsSampleDuration: *mut LONGLONG,
    ) -> HRESULT,
    fn SetSampleDuration(
        hnsSampleDuration: LONGLONG,
    ) -> HRESULT,
    fn GetBufferCount(
        pdwBufferCount: *mut DWORD,
    ) -> HRESULT,
    fn GetBufferByIndex(
        dwIndex: DWORD,
        ppBuffer: *mut *mut IMFMediaBuffer,
    ) -> HRESULT,
    fn ConvertToContiguousBuffer(
        ppBuffer: *mut *mut IMFMediaBuffer,
    ) -> HRESULT,
    fn AddBuffer(
        pBuffer: *mut IMFMediaBuffer,
    ) -> HRESULT,
    fn RemoveBufferByIndex(
        dwIndex: DWORD,
    ) -> HRESULT,
    fn RemoveAllBuffers() -> HRESULT,
    fn GetTotalLength(
        pcbTotalLength: *mut DWORD,
    ) -> HRESULT,
    fn CopyToBuffer(
        pBuffer: *mut IMFMediaBuffer,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7DC9D5F9, 0x9ED9, 0x44ec, 0x9B, 0xBF, 0x06, 0x00, 0xBB, 0x58, 0x9F, 0xBB)]
interface IMF2DBuffer(IMF2DBufferVtbl): IUnknown(IUnknownVtbl) {
    fn Lock2D(
        ppbScanline0: *mut *mut BYTE,
        plPitch: *mut LONG,
    ) -> HRESULT,
    fn Unlock2D() -> HRESULT,
    fn GetScanline0AndPitch(
        pbScanline0: *mut *mut BYTE,
        plPitch: *mut LONG,
    ) -> HRESULT,
    fn IsContiguousFormat(
        pfIsContiguous: *mut BOOL,
    ) -> HRESULT,
    fn GetContiguousLength(
        pcbLength: *mut DWORD,
    ) -> HRESULT,
    fn ContiguousCopyTo(
        pbDestBuffer: *mut BYTE,
        cbDestBuffer: DWORD,
    ) -> HRESULT,
    fn ContiguousCopyFrom(
        pbSrcBuffer: *const BYTE,
        cbSrcBuffer: DWORD,
    ) -> HRESULT,
}}
pub type MF2DBuffer_LockFlags = DWORD;
pub const MF2DBuffer_LockFlags_LockTypeMask: DWORD = 0x3;
pub const MF2DBuffer_LockFlags_Read: DWORD = 0x1;
pub const MF2DBuffer_LockFlags_Write: DWORD = 0x2;
pub const MF2DBuffer_LockFlags_ReadWrite: DWORD = 0x3;
RIDL!{#[uuid(0x33ae5ea6, 0x4316, 0x436f, 0x8d, 0xdd, 0xd7, 0x3d, 0x22, 0xf8, 0x29, 0xec)]
interface IMF2DBuffer2(IMF2DBuffer2Vtbl): IMF2DBuffer(IMF2DBufferVtbl) {
    fn Lock2DSize(
        lockFlags: MF2DBuffer_LockFlags,
        ppbScanline0: *mut *mut BYTE,
        plPitch: *mut LONG,
        ppbBufferStart: *mut *mut BYTE,
        pcbBufferLength: *mut DWORD,
    ) -> HRESULT,
    fn Copy2DTo(
        pDestBuffer: *mut IMF2DBuffer2,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe7174cfa, 0x1c9e, 0x48b1, 0x88, 0x66, 0x62, 0x62, 0x26, 0xbf, 0xc2, 0x58)]
interface IMFDXGIBuffer(IMFDXGIBufferVtbl): IUnknown(IUnknownVtbl) {
    fn GetResource(
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
    fn GetSubresourceIndex(
        puSubresource: *mut UINT,
    ) -> HRESULT,
    fn GetUnknown(
        guid: REFIID,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
    fn SetUnknown(
        guid: REFIID,
        pUnkData: *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x44ae0fa8, 0xea31, 0x4109, 0x8d, 0x2e, 0x4c, 0xae, 0x49, 0x97, 0xc5, 0x55)]
interface IMFMediaType(IMFMediaTypeVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetMajorType(
        pguidMajorType: *mut GUID,
    ) -> HRESULT,
    fn IsCompressedFormat(
        pfCompressed: *mut BOOL,
    ) -> HRESULT,
    fn IsEqual(
        pIMediaType: *mut IMFMediaType,
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
    fn GetRepresentation(
        guidRepresentation: GUID,
        ppvRepresentation: *mut LPVOID,
    ) -> HRESULT,
    fn FreeRepresentation(
        guidRepresentation: GUID,
        pvRepresentation: LPVOID,
    ) -> HRESULT,
}}
pub const MF_MEDIATYPE_EQUAL_MAJOR_TYPES: DWORD = 0x00000001;
pub const MF_MEDIATYPE_EQUAL_FORMAT_TYPES: DWORD = 0x00000002;
pub const MF_MEDIATYPE_EQUAL_FORMAT_DATA: DWORD = 0x00000004;
pub const MF_MEDIATYPE_EQUAL_FORMAT_USER_DATA: DWORD = 0x00000008;
RIDL!{#[uuid(0x26a0adc3, 0xce26, 0x4672, 0x93, 0x04, 0x69, 0x55, 0x2e, 0xdd, 0x3f, 0xaf)]
interface IMFAudioMediaType(IMFAudioMediaTypeVtbl): IMFMediaType(IMFMediaTypeVtbl) {
    fn GetAudioFormat() -> *const WAVEFORMATEX,
}}
STRUCT!{struct MFT_REGISTER_TYPE_INFO {
    guidMajorType: GUID,
    guidSubtype: GUID,
}}
ENUM!{enum MFVideoInterlaceMode {
    MFVideoInterlace_Unknown	= 0,
    MFVideoInterlace_Progressive	= 2,
    MFVideoInterlace_FieldInterleavedUpperFirst	= 3,
    MFVideoInterlace_FieldInterleavedLowerFirst	= 4,
    MFVideoInterlace_FieldSingleUpper	= 5,
    MFVideoInterlace_FieldSingleLower	= 6,
    MFVideoInterlace_MixedInterlaceOrProgressive	= 7,
    MFVideoInterlace_Last	= ( MFVideoInterlace_MixedInterlaceOrProgressive + 1 ) ,
}}
pub const MFVideoInterlace_FieldSingleUpperFirst: u32 = MFVideoInterlace_FieldSingleUpper;
pub const MFVideoInterlace_FieldSingleLowerFirst: u32 = MFVideoInterlace_FieldSingleLower;
ENUM!{enum MFVideoTransferFunction {
    MFVideoTransFunc_Unknown	= 0,
    MFVideoTransFunc_10	= 1,
    MFVideoTransFunc_18	= 2,
    MFVideoTransFunc_20	= 3,
    MFVideoTransFunc_22	= 4,
    MFVideoTransFunc_709	= 5,
    MFVideoTransFunc_240M	= 6,
    MFVideoTransFunc_sRGB	= 7,
    MFVideoTransFunc_28	= 8,
    MFVideoTransFunc_Log_100	= 9,
    MFVideoTransFunc_Log_316	= 10,
    MFVideoTransFunc_709_sym	= 11,
    MFVideoTransFunc_2020_const	= 12,
    MFVideoTransFunc_2020	= 13,
    MFVideoTransFunc_26	= 14,
    MFVideoTransFunc_2084	= 15,
    MFVideoTransFunc_HLG	= 16,
    MFVideoTransFunc_10_rel	= 17,
    MFVideoTransFunc_Last	= ( MFVideoTransFunc_10_rel + 1 ) ,
}}
ENUM!{enum MFVideoPrimaries {
    MFVideoPrimaries_Unknown	= 0,
    MFVideoPrimaries_reserved	= 1,
    MFVideoPrimaries_BT709	= 2,
    MFVideoPrimaries_BT470_2_SysM	= 3,
    MFVideoPrimaries_BT470_2_SysBG	= 4,
    MFVideoPrimaries_SMPTE170M	= 5,
    MFVideoPrimaries_SMPTE240M	= 6,
    MFVideoPrimaries_EBU3213	= 7,
    MFVideoPrimaries_SMPTE_C	= 8,
    MFVideoPrimaries_BT2020	= 9,
    MFVideoPrimaries_XYZ	= 10,
    MFVideoPrimaries_DCI_P3	= 11,
    MFVideoPrimaries_ACES	= 12,
    MFVideoPrimaries_Last	= ( MFVideoPrimaries_ACES + 1 ) ,
}}
ENUM!{enum MFVideoLighting {
    MFVideoLighting_Unknown	= 0,
    MFVideoLighting_bright	= 1,
    MFVideoLighting_office	= 2,
    MFVideoLighting_dim	= 3,
    MFVideoLighting_dark	= 4,
    MFVideoLighting_Last	= ( MFVideoLighting_dark + 1 ) ,
}}
ENUM!{enum MFVideoTransferMatrix {
    MFVideoTransferMatrix_Unknown	= 0,
    MFVideoTransferMatrix_BT709	= 1,
    MFVideoTransferMatrix_BT601	= 2,
    MFVideoTransferMatrix_SMPTE240M	= 3,
    MFVideoTransferMatrix_BT2020_10	= 4,
    MFVideoTransferMatrix_BT2020_12	= 5,
    MFVideoTransferMatrix_Last	= ( MFVideoTransferMatrix_BT2020_12 + 1 ) ,
}}
ENUM!{enum MFVideoChromaSubsampling {
    MFVideoChromaSubsampling_Unknown = 0,
    MFVideoChromaSubsampling_Vertically_AlignedChromaPlanes	= 0x1,
    MFVideoChromaSubsampling_Vertically_Cosited	= 0x2,
    MFVideoChromaSubsampling_Horizontally_Cosited = 0x4,
    MFVideoChromaSubsampling_ProgressiveChroma = 0x8,
    MFVideoChromaSubsampling_MPEG2 =
        MFVideoChromaSubsampling_Horizontally_Cosited |
        MFVideoChromaSubsampling_Vertically_AlignedChromaPlanes,
    MFVideoChromaSubsampling_MPEG1 =
        MFVideoChromaSubsampling_Vertically_AlignedChromaPlanes,
    MFVideoChromaSubsampling_DV_PAL	=
        MFVideoChromaSubsampling_Horizontally_Cosited |
        MFVideoChromaSubsampling_Vertically_Cosited,
    MFVideoChromaSubsampling_Cosited =
        MFVideoChromaSubsampling_Horizontally_Cosited |
        MFVideoChromaSubsampling_Vertically_Cosited |
        MFVideoChromaSubsampling_Vertically_AlignedChromaPlanes,
    MFVideoChromaSubsampling_Last = MFVideoChromaSubsampling_Cosited + 1,
}}
ENUM!{enum MFNominalRange {
    MFNominalRange_Unknown	= 0,
    MFNominalRange_Normal	= 1,
    MFNominalRange_Wide	    = 2,
    MFNominalRange_0_255	= 1,
    MFNominalRange_16_235	= 2,
    MFNominalRange_48_208	= 3,
    MFNominalRange_64_127	= 4,
    MFNominalRange_Last	    = MFNominalRange_64_127 + 1,
}}
ENUM!{enum MFVideoFlags {
    MFVideoFlag_PAD_TO_Mask	= ( 0x1 | 0x2 ) ,
    MFVideoFlag_PAD_TO_None	= ( 0 * 0x1 ) ,
    MFVideoFlag_PAD_TO_4x3	= ( 1 * 0x1 ) ,
    MFVideoFlag_PAD_TO_16x9	= ( 2 * 0x1 ) ,
    MFVideoFlag_SrcContentHintMask	= ( ( 0x4 | 0x8 )  | 0x10 ) ,
    MFVideoFlag_SrcContentHintNone	= ( 0 * 0x4 ) ,
    MFVideoFlag_SrcContentHint16x9	= ( 1 * 0x4 ) ,
    MFVideoFlag_SrcContentHint235_1	= ( 2 * 0x4 ) ,
    MFVideoFlag_AnalogProtected	= 0x20,
    MFVideoFlag_DigitallyProtected	= 0x40,
    MFVideoFlag_ProgressiveContent	= 0x80,
    MFVideoFlag_FieldRepeatCountMask	= ( ( 0x100 | 0x200 )  | 0x400 ) ,
    MFVideoFlag_FieldRepeatCountShift	= 8,
    MFVideoFlag_ProgressiveSeqReset	= 0x800,
    MFVideoFlag_PanScanEnabled	= 0x2_0000,
    MFVideoFlag_LowerFieldFirst	= 0x4_0000,
    MFVideoFlag_BottomUpLinearRep	= 0x8_0000,
    MFVideoFlags_DXVASurface	= 0x10_0000,
    MFVideoFlags_RenderTargetSurface	= 0x40_0000,
}}
STRUCT!{struct MFRatio {
    Numerator: DWORD,
    Denominator: DWORD,
}}
STRUCT!{struct MFOffset {
    fract: WORD,
    value: c_short,
}}
STRUCT!{struct MFVideoArea {
    OffsetX: MFOffset,
    OffsetY: MFOffset,
    Area: SIZE,
}}
STRUCT!{struct MFVideoInfo {
    dwWidth: DWORD,
    dwHeight: DWORD,
    PixelAspectRatio: MFRatio,
    SourceChromaSubsampling: MFVideoChromaSubsampling,
    InterlaceMode: MFVideoInterlaceMode,
    TransferFunction: MFVideoTransferFunction,
    ColorPrimaries: MFVideoPrimaries,
    TransferMatrix: MFVideoTransferMatrix,
    SourceLighting: MFVideoLighting,
    FramesPerSecond: MFRatio,
    NominalRange: MFNominalRange,
    GeometricAperture: MFVideoArea,
    MinimumDisplayAperture: MFVideoArea,
    PanScanAperture: MFVideoArea,
    VideoFlags: u64,
}}
STRUCT!{struct MFAYUVSample {
    bCrValue: BYTE,
    bCbValue: BYTE,
    bYValue: BYTE,
    bSampleAlpha8: BYTE,
}}
STRUCT!{struct MFARGB {
    rgbBlue: BYTE,
    rgbGreen: BYTE,
    rgbRed: BYTE,
    rgbAlpha: BYTE,
}}
UNION! {union MFPaletteEntry {
    [u8; 4],
    ARGB ARGB_mut: MFARGB,
    AYCbCr AYCbCr_mut: MFAYUVSample,
}}
STRUCT!{struct MFVideoSurfaceInfo {
    Format: DWORD,
    PaletteEntries: DWORD,
    Palette: [MFPaletteEntry;1],
}}
STRUCT!{struct MFVideoCompressedInfo {
    AvgBitrate: LONGLONG,
    AvgBitErrorRate: LONGLONG,
    MaxKeyFrameSpacing: DWORD,
}}
STRUCT!{struct MFVIDEOFORMAT {
    dwSize: DWORD,
    videoInfo: MFVideoInfo,
    guidFormat: GUID,
    compressedInfo: MFVideoCompressedInfo,
    surfaceInfo: MFVideoSurfaceInfo,
}}
ENUM!{enum MFStandardVideoFormat {
    MFStdVideoFormat_reserved,
    MFStdVideoFormat_NTSC,
    MFStdVideoFormat_PAL,
    MFStdVideoFormat_DVD_NTSC,
    MFStdVideoFormat_DVD_PAL,
    MFStdVideoFormat_DV_PAL,
    MFStdVideoFormat_DV_NTSC,
    MFStdVideoFormat_ATSC_SD480i,
    MFStdVideoFormat_ATSC_HD1080i,
    MFStdVideoFormat_ATSC_HD720p,
}}
RIDL!{#[uuid(0xb99f381f, 0xa8f9, 0x47a2, 0xa5, 0xaf, 0xca, 0x3a, 0x22, 0x5a, 0x38, 0x90)]
interface IMFVideoMediaType(IMFVideoMediaTypeVtbl): IMFMediaType(IMFMediaTypeVtbl) {
    fn GetVideoFormat() -> *const MFVIDEOFORMAT,
    fn GetVideoRepresentation(
        guidRepresentation: GUID,
        ppvRepresentation: *mut LPVOID,
        lStride: LONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xac6b7889, 0x0740, 0x4d51, 0x86, 0x19, 0x90, 0x59, 0x94, 0xa5, 0x5c, 0xc6)]
interface IMFAsyncResult(IMFAsyncResultVtbl): IUnknown(IUnknownVtbl) {
    fn GetState(
        ppunkState: *mut *mut IUnknown,
    ) -> HRESULT,
    fn GetStatus() -> HRESULT,
    fn SetStatus(
        hrStatus: HRESULT,
    ) -> HRESULT,
    fn GetObject(
        ppObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn GetStateNoAddRef() -> *mut IUnknown,
}}
RIDL!{#[uuid(0xa27003cf, 0x2354, 0x4f2a, 0x8d, 0x6a, 0xab, 0x7c, 0xff, 0x15, 0x43, 0x7e)]
interface IMFAsyncCallback(IMFAsyncCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn GetParameters(
        pdwFlags: *mut DWORD,
        pdwQueue: *mut DWORD,
    ) -> HRESULT,
    fn Invoke(
        pAsyncResult: *mut IMFAsyncResult,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc7a4dca1, 0xf5f0, 0x47b6, 0xb9, 0x2b, 0xbf, 0x01, 0x06, 0xd2, 0x57, 0x91)]
interface IMFAsyncCallbackLogging(IMFAsyncCallbackLoggingVtbl):
  IMFAsyncCallback(IMFAsyncCallbackVtbl) {
    fn GetObjectPointer() -> *mut c_void,
    fn GetObjectTag() -> DWORD,
}}
pub const MFASYNC_FAST_IO_PROCESSING_CALLBACK: DWORD = 0x00000001;
pub const MFASYNC_SIGNAL_CALLBACK: DWORD = 0x00000002;
pub const MFASYNC_BLOCKING_CALLBACK: DWORD = 0x00000004;
pub const MFASYNC_REPLY_CALLBACK: DWORD = 0x00000008;
pub const MFASYNC_LOCALIZE_REMOTE_CALLBACK: DWORD = 0x00000010;
pub const MFASYNC_CALLBACK_QUEUE_UNDEFINED: DWORD = 0x00000000;
pub const MFASYNC_CALLBACK_QUEUE_STANDARD: DWORD = 0x00000001;
pub const MFASYNC_CALLBACK_QUEUE_RT: DWORD = 0x00000002;
pub const MFASYNC_CALLBACK_QUEUE_IO: DWORD = 0x00000003;
pub const MFASYNC_CALLBACK_QUEUE_TIMER: DWORD = 0x00000004;
pub const MFASYNC_CALLBACK_QUEUE_MULTITHREADED: DWORD = 0x00000005;
pub const MFASYNC_CALLBACK_QUEUE_LONG_FUNCTION: DWORD = 0x00000007;
pub const MFASYNC_CALLBACK_QUEUE_PRIVATE_MASK: DWORD = 0xFFFF0000;
pub const MFASYNC_CALLBACK_QUEUE_ALL: DWORD = 0xFFFFFFFF;
pub type MediaEventType = DWORD;
pub const MEUnknown: DWORD = 0;
pub const MEError: DWORD = 1;
pub const MEExtendedType: DWORD = 2;
pub const MENonFatalError: DWORD = 3;
pub const MEGenericV1Anchor: DWORD = MENonFatalError;
pub const MESessionUnknown: DWORD = 100;
pub const MESessionTopologySet: DWORD = 101;
pub const MESessionTopologiesCleared: DWORD = 102;
pub const MESessionStarted: DWORD = 103;
pub const MESessionPaused: DWORD = 104;
pub const MESessionStopped: DWORD = 105;
pub const MESessionClosed: DWORD = 106;
pub const MESessionEnded: DWORD = 107;
pub const MESessionRateChanged: DWORD = 108;
pub const MESessionScrubSampleComplete: DWORD = 109;
pub const MESessionCapabilitiesChanged: DWORD = 110;
pub const MESessionTopologyStatus: DWORD = 111;
pub const MESessionNotifyPresentationTime: DWORD = 112;
pub const MENewPresentation: DWORD = 113;
pub const MELicenseAcquisitionStart: DWORD = 114;
pub const MELicenseAcquisitionCompleted: DWORD = 115;
pub const MEIndividualizationStart: DWORD = 116;
pub const MEIndividualizationCompleted: DWORD = 117;
pub const MEEnablerProgress: DWORD = 118;
pub const MEEnablerCompleted: DWORD = 119;
pub const MEPolicyError: DWORD = 120;
pub const MEPolicyReport: DWORD = 121;
pub const MEBufferingStarted: DWORD = 122;
pub const MEBufferingStopped: DWORD = 123;
pub const MEConnectStart: DWORD = 124;
pub const MEConnectEnd: DWORD = 125;
pub const MEReconnectStart: DWORD = 126;
pub const MEReconnectEnd: DWORD = 127;
pub const MERendererEvent: DWORD = 128;
pub const MESessionStreamSinkFormatChanged: DWORD = 129;
pub const MESessionV1Anchor: DWORD = MESessionStreamSinkFormatChanged;
pub const MESourceUnknown: DWORD = 200;
pub const MESourceStarted: DWORD = 201;
pub const MEStreamStarted: DWORD = 202;
pub const MESourceSeeked: DWORD = 203;
pub const MEStreamSeeked: DWORD = 204;
pub const MENewStream: DWORD = 205;
pub const MEUpdatedStream: DWORD = 206;
pub const MESourceStopped: DWORD = 207;
pub const MEStreamStopped: DWORD = 208;
pub const MESourcePaused: DWORD = 209;
pub const MEStreamPaused: DWORD = 210;
pub const MEEndOfPresentation: DWORD = 211;
pub const MEEndOfStream: DWORD = 212;
pub const MEMediaSample: DWORD = 213;
pub const MEStreamTick: DWORD = 214;
pub const MEStreamThinMode: DWORD = 215;
pub const MEStreamFormatChanged: DWORD = 216;
pub const MESourceRateChanged: DWORD = 217;
pub const MEEndOfPresentationSegment: DWORD = 218;
pub const MESourceCharacteristicsChanged: DWORD = 219;
pub const MESourceRateChangeRequested: DWORD = 220;
pub const MESourceMetadataChanged: DWORD = 221;
pub const MESequencerSourceTopologyUpdated: DWORD = 222;
pub const MESourceV1Anchor: DWORD = MESequencerSourceTopologyUpdated;
pub const MESinkUnknown: DWORD = 300;
pub const MEStreamSinkStarted: DWORD = 301;
pub const MEStreamSinkStopped: DWORD = 302;
pub const MEStreamSinkPaused: DWORD = 303;
pub const MEStreamSinkRateChanged: DWORD = 304;
pub const MEStreamSinkRequestSample: DWORD = 305;
pub const MEStreamSinkMarker: DWORD = 306;
pub const MEStreamSinkPrerolled: DWORD = 307;
pub const MEStreamSinkScrubSampleComplete: DWORD = 308;
pub const MEStreamSinkFormatChanged: DWORD = 309;
pub const MEStreamSinkDeviceChanged: DWORD = 310;
pub const MEQualityNotify: DWORD = 311;
pub const MESinkInvalidated: DWORD = 312;
pub const MEAudioSessionNameChanged: DWORD = 313;
pub const MEAudioSessionVolumeChanged: DWORD = 314;
pub const MEAudioSessionDeviceRemoved: DWORD = 315;
pub const MEAudioSessionServerShutdown: DWORD = 316;
pub const MEAudioSessionGroupingParamChanged: DWORD = 317;
pub const MEAudioSessionIconChanged: DWORD = 318;
pub const MEAudioSessionFormatChanged: DWORD = 319;
pub const MEAudioSessionDisconnected: DWORD = 320;
pub const MEAudioSessionExclusiveModeOverride: DWORD = 321;
pub const MESinkV1Anchor: DWORD = MEAudioSessionExclusiveModeOverride;
pub const MECaptureAudioSessionVolumeChanged: DWORD = 322;
pub const MECaptureAudioSessionDeviceRemoved: DWORD = 323;
pub const MECaptureAudioSessionFormatChanged: DWORD = 324;
pub const MECaptureAudioSessionDisconnected: DWORD = 325;
pub const MECaptureAudioSessionExclusiveModeOverride: DWORD = 326;
pub const MECaptureAudioSessionServerShutdown: DWORD = 327;
pub const MESinkV2Anchor: DWORD = MECaptureAudioSessionServerShutdown;
pub const METrustUnknown: DWORD = 400;
pub const MEPolicyChanged: DWORD = 401;
pub const MEContentProtectionMessage: DWORD = 402;
pub const MEPolicySet: DWORD = 403;
pub const METrustV1Anchor: DWORD = MEPolicySet;
pub const MEWMDRMLicenseBackupCompleted: DWORD = 500;
pub const MEWMDRMLicenseBackupProgress: DWORD = 501;
pub const MEWMDRMLicenseRestoreCompleted: DWORD = 502;
pub const MEWMDRMLicenseRestoreProgress: DWORD = 503;
pub const MEWMDRMLicenseAcquisitionCompleted: DWORD = 506;
pub const MEWMDRMIndividualizationCompleted: DWORD = 508;
pub const MEWMDRMIndividualizationProgress: DWORD = 513;
pub const MEWMDRMProximityCompleted: DWORD = 514;
pub const MEWMDRMLicenseStoreCleaned: DWORD = 515;
pub const MEWMDRMRevocationDownloadCompleted: DWORD = 516;
pub const MEWMDRMV1Anchor: DWORD = MEWMDRMRevocationDownloadCompleted;
pub const METransformUnknown: DWORD = 600;
pub const METransformNeedInput: DWORD = (METransformUnknown + 1);
pub const METransformHaveOutput: DWORD = (METransformNeedInput + 1);
pub const METransformDrainComplete: DWORD = (METransformHaveOutput + 1);
pub const METransformMarker: DWORD = (METransformDrainComplete + 1);
pub const METransformInputStreamStateChanged: DWORD = (METransformMarker + 1);
pub const MEByteStreamCharacteristicsChanged: DWORD = 700;
pub const MEVideoCaptureDeviceRemoved: DWORD = 800;
pub const MEVideoCaptureDevicePreempted: DWORD = 801;
pub const MEStreamSinkFormatInvalidated: DWORD = 802;
pub const MEEncodingParameters: DWORD = 803;
pub const MEContentProtectionMetadata: DWORD = 900;
pub const MEDeviceThermalStateChanged: DWORD = 950;
pub const MEReservedMax: DWORD = 10000;
RIDL!{#[uuid(0xDF598932, 0xF10C, 0x4E39, 0xBB, 0xA2, 0xC3, 0x08, 0xF1, 0x01, 0xDA, 0xA3)]
interface IMFMediaEvent(IMFMediaEventVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetType(
        pmet: *mut MediaEventType,
    ) -> HRESULT,
    fn GetExtendedType(
        pguidExtendedType: *mut GUID,
    ) -> HRESULT,
    fn GetStatus(
        phrStatus: *mut HRESULT,
    ) -> HRESULT,
    fn GetValue(
        pvValue: *mut PROPVARIANT,
    ) -> HRESULT,
}}
pub const MF_EVENT_FLAG_NO_WAIT: DWORD = 0x00000001;
RIDL!{#[uuid(0x2CD0BD52, 0xBCD5, 0x4B89, 0xB6, 0x2C, 0xEA, 0xDC, 0x0C, 0x03, 0x1E, 0x7D)]
interface IMFMediaEventGenerator(IMFMediaEventGeneratorVtbl): IUnknown(IUnknownVtbl) {
    fn GetEvent(
        dwFlags: DWORD,
        ppEvent: *mut *mut IMFMediaEvent,
    ) -> HRESULT,
    fn BeginGetEvent(
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndGetEvent(
        pResult: *mut IMFAsyncResult,
        ppEvent: *mut *mut IMFMediaEvent,
    ) -> HRESULT,
    fn QueueEvent(
        met: MediaEventType,
        guidExtendedType: REFGUID,
        hrStatus: HRESULT,
        pvValue: *const PROPVARIANT,
    ) -> HRESULT,
}}
/*
/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaEventGenerator_RemoteBeginGetEvent_Proxy(
    __RPC__in IMFMediaEventGenerator * This,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);
void __RPC_STUB IMFMediaEventGenerator_RemoteBeginGetEvent_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);
/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaEventGenerator_RemoteEndGetEvent_Proxy(
    __RPC__in IMFMediaEventGenerator * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult,
    /* [out] */ __RPC__out DWORD *pcbEvent,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbEvent) BYTE **ppbEvent);
void __RPC_STUB IMFMediaEventGenerator_RemoteEndGetEvent_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);
*/
RIDL!{#[uuid(0xa27003d0, 0x2354, 0x4f2a, 0x8d, 0x6a, 0xab, 0x7c, 0xff, 0x15, 0x43, 0x7e)]
interface IMFRemoteAsyncCallback(IMFRemoteAsyncCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn Invoke(
        hr: HRESULT,
        pRemoteResult: *mut IUnknown,
    ) -> HRESULT,
}}
ENUM!{enum MFBYTESTREAM_SEEK_ORIGIN {
    msoBegin,
    msoCurrent,
}}
RIDL!{#[uuid(0xad4c1b00, 0x4bf7, 0x422f, 0x91, 0x75, 0x75, 0x66, 0x93, 0xd9, 0x13, 0x0d)]
interface IMFByteStream(IMFByteStreamVtbl): IUnknown(IUnknownVtbl) {
    fn GetCapabilities(
        pdwCapabilities: *mut DWORD,
    ) -> HRESULT,
    fn GetLength(
        pqwLength: *mut QWORD,
    ) -> HRESULT,
    fn SetLength(
        qwLength: QWORD,
    ) -> HRESULT,
    fn GetCurrentPosition(
        pqwPosition: *mut QWORD,
    ) -> HRESULT,
    fn SetCurrentPosition(
        qwPosition: QWORD,
    ) -> HRESULT,
    fn IsEndOfStream(
        pfEndOfStream: *mut BOOL,
    ) -> HRESULT,
    fn Read(
        pb: *mut BYTE,
        cb: ULONG,
        pcbRead: *mut ULONG,
    ) -> HRESULT,
    fn BeginRead(
        pb: *mut BYTE,
        cb: ULONG,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndRead(
        pResult: *mut IMFAsyncResult,
        pcbRead: *mut ULONG,
    ) -> HRESULT,
    fn Write(
        pb: *const BYTE,
        cb: ULONG,
        pcbWritten: *mut ULONG,
    ) -> HRESULT,
    fn BeginWrite(
        pb: *const BYTE,
        cb: ULONG,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndWrite(
        pResult: *mut IMFAsyncResult,
        pcbWritten: *mut ULONG,
    ) -> HRESULT,
    fn Seek(
        SeekOrigin: MFBYTESTREAM_SEEK_ORIGIN,
        llSeekOffset: LONGLONG,
        dwSeekFlags: DWORD,
        pqwCurrentPosition: *mut QWORD,
    ) -> HRESULT,
    fn Flush() -> HRESULT,
    fn Close() -> HRESULT,
}}
pub const MFBYTESTREAM_IS_READABLE: DWORD = 0x00000001;
pub const MFBYTESTREAM_IS_WRITABLE: DWORD = 0x00000002;
pub const MFBYTESTREAM_IS_SEEKABLE: DWORD = 0x00000004;
pub const MFBYTESTREAM_IS_REMOTE: DWORD = 0x00000008;
pub const MFBYTESTREAM_IS_DIRECTORY: DWORD = 0x00000080;
pub const MFBYTESTREAM_HAS_SLOW_SEEK: DWORD = 0x00000100;
pub const MFBYTESTREAM_IS_PARTIALLY_DOWNLOADED: DWORD = 0x00000200;
pub const MFBYTESTREAM_SHARE_WRITE: DWORD = 0x00000400;
pub const MFBYTESTREAM_DOES_NOT_USE_NETWORK: DWORD = 0x00000800;
pub const MFBYTESTREAM_SEEK_FLAG_CANCEL_PENDING_IO: DWORD = 0x00000001;
DEFINE_GUID!{MF_BYTESTREAM_ORIGIN_NAME,
    0xfc358288, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a}
DEFINE_GUID!{MF_BYTESTREAM_CONTENT_TYPE,
    0xfc358289, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a}
DEFINE_GUID!{MF_BYTESTREAM_DURATION,
    0xfc35828a, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a}
DEFINE_GUID!{MF_BYTESTREAM_LAST_MODIFIED_TIME,
    0xfc35828b, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a}
DEFINE_GUID!{MF_BYTESTREAM_IFO_FILE_URI,
    0xfc35828c, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a}
DEFINE_GUID!{MF_BYTESTREAM_DLNA_PROFILE_ID,
    0xfc35828d, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a}
DEFINE_GUID!{MF_BYTESTREAM_EFFECTIVE_URL,
    0x9afa0209, 0x89d1, 0x42af, 0x84, 0x56, 0x1d, 0xe6, 0xb5, 0x62, 0xd6, 0x91}
DEFINE_GUID!{MF_BYTESTREAM_TRANSCODED,
    0xb6c5c282, 0x4dc9, 0x4db9, 0xab, 0x48, 0xcf, 0x3b, 0x6d, 0x8b, 0xc5, 0xe0}
DEFINE_GUID!{CLSID_MFByteStreamProxyClassFactory,
    0x770e8e77, 0x4916, 0x441c, 0xa9, 0xa7, 0xb3, 0x42, 0xd0, 0xee, 0xbc, 0x71}
RIDL!{#[uuid(0xa6b43f84, 0x5c0a, 0x42e8, 0xa4, 0x4d, 0xb1, 0x85, 0x7a, 0x76, 0x99, 0x2f)]
interface IMFByteStreamProxyClassFactory(IMFByteStreamProxyClassFactoryVtbl):
  IUnknown(IUnknownVtbl) {
    fn CreateByteStreamProxy(
        pByteStream: *mut IMFByteStream,
        pAttributes: *mut IMFAttributes,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
}}
ENUM!{enum MF_FILE_ACCESSMODE {
    MF_ACCESSMODE_READ,
    MF_ACCESSMODE_WRITE,
    MF_ACCESSMODE_READWRITE,
}}
ENUM!{enum MF_FILE_OPENMODE {
    MF_OPENMODE_FAIL_IF_NOT_EXIST = 0,
    MF_OPENMODE_FAIL_IF_EXIST	= 1,
    MF_OPENMODE_RESET_IF_EXIST	= 2,
    MF_OPENMODE_APPEND_IF_EXIST	= 3,
    MF_OPENMODE_DELETE_IF_EXIST	= 4,
}}
ENUM!{enum MF_FILE_FLAGS {
    MF_FILEFLAGS_NONE	= 0,
    MF_FILEFLAGS_NOBUFFERING	= 0x1,
    MF_FILEFLAGS_ALLOW_WRITE_SHARING	= 0x2,
}}
RIDL!{#[uuid(0x8feed468, 0x6f7e, 0x440d, 0x86, 0x9a, 0x49, 0xbd, 0xd2, 0x83, 0xad, 0x0d)]
interface IMFSampleOutputStream(IMFSampleOutputStreamVtbl): IUnknown(IUnknownVtbl) {
    fn BeginWriteSample(
        pSample: *mut IMFSample,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndWriteSample(
        pResult: *mut IMFAsyncResult,
    ) -> HRESULT,
    fn Close() -> HRESULT,
}}
RIDL!{#[uuid(0x5BC8A76B, 0x869A, 0x46a3, 0x9B, 0x03, 0xFA, 0x21, 0x8A, 0x66, 0xAE, 0xBE)]
interface IMFCollection(IMFCollectionVtbl): IUnknown(IUnknownVtbl) {
    fn GetElementCount(
        pcElements: *mut DWORD,
    ) -> HRESULT,
    fn GetElement(
        dwElementIndex: DWORD,
        ppUnkElement: *mut *mut IUnknown,
    ) -> HRESULT,
    fn AddElement(
        pUnkElement: *mut IUnknown,
    ) -> HRESULT,
    fn RemoveElement(
        dwElementIndex: DWORD,
        ppUnkElement: *mut *mut IUnknown,
    ) -> HRESULT,
    fn InsertElementAt(
        dwIndex: DWORD,
        pUnknown: *mut IUnknown,
    ) -> HRESULT,
    fn RemoveAllElements() -> HRESULT,
}}
RIDL!{#[uuid(0x36f846fc, 0x2256, 0x48b6, 0xb5, 0x8e, 0xe2, 0xb6, 0x38, 0x31, 0x65, 0x81)]
interface IMFMediaEventQueue(IMFMediaEventQueueVtbl): IUnknown(IUnknownVtbl) {
    fn GetEvent(
        dwFlags: DWORD,
        ppEvent: *mut *mut IMFMediaEvent,
    ) -> HRESULT,
    fn BeginGetEvent(
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndGetEvent(
        pResult: *mut IMFAsyncResult,
        ppEvent: *mut *mut IMFMediaEvent,
    ) -> HRESULT,
    fn QueueEvent(
        pEvent: *mut IMFMediaEvent,
    ) -> HRESULT,
    fn QueueEventParamVar(
        met: MediaEventType,
        guidExtendedType: REFGUID,
        hrStatus: HRESULT,
        pvValue: *const PROPVARIANT,
    ) -> HRESULT,
    fn QueueEventParamUnk(
        met: MediaEventType,
        guidExtendedType: REFGUID,
        hrStatus: HRESULT,
        pUnk: *mut IUnknown,
    ) -> HRESULT,
    fn Shutdown() -> HRESULT,
}}
RIDL!{#[uuid(0x7FEE9E9A, 0x4A89, 0x47a6, 0x89, 0x9C, 0xB6, 0xA5, 0x3A, 0x70, 0xFB, 0x67)]
interface IMFActivate(IMFActivateVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn ActivateObject(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn ShutdownObject() -> HRESULT,
    fn DetachObject() -> HRESULT,
}}
ENUM!{enum MF_Plugin_Type {
    MF_Plugin_Type_MFT	= 0,
    MF_Plugin_Type_MediaSource	= 1,
    MF_Plugin_Type_MFT_MatchOutputType	= 2,
    MF_Plugin_Type_Other	= -1i32 as u32,
}}
RIDL!{#[uuid(0x5c6c44bf, 0x1db6, 0x435b, 0x92, 0x49, 0xe8, 0xcd, 0x10, 0xfd, 0xec, 0x96)]
interface IMFPluginControl(IMFPluginControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetPreferredClsid(
        pluginType: DWORD,
        selector: LPCWSTR,
        clsid: *mut CLSID,
    ) -> HRESULT,
    fn GetPreferredClsidByIndex(
        pluginType: DWORD,
        index: DWORD,
        selector: *mut LPWSTR,
        clsid: *mut CLSID,
    ) -> HRESULT,
    fn SetPreferredClsid(
        pluginType: DWORD,
        selector: LPCWSTR,
        clsid: *const CLSID,
    ) -> HRESULT,
    fn IsDisabled(
        pluginType: DWORD,
        clsid: REFCLSID,
    ) -> HRESULT,
    fn GetDisabledByIndex(
        pluginType: DWORD,
        index: DWORD,
        clsid: *mut CLSID,
    ) -> HRESULT,
    fn SetDisabled(
        pluginType: DWORD,
        clsid: REFCLSID,
        disabled: BOOL,
    ) -> HRESULT,
}}
ENUM!{enum MF_PLUGIN_CONTROL_POLICY {
        MF_PLUGIN_CONTROL_POLICY_USE_ALL_PLUGINS,
        MF_PLUGIN_CONTROL_POLICY_USE_APPROVED_PLUGINS,
        MF_PLUGIN_CONTROL_POLICY_USE_WEB_PLUGINS,
        MF_PLUGIN_CONTROL_POLICY_USE_WEB_PLUGINS_EDGEMODE,
}}
RIDL!{#[uuid(0xC6982083, 0x3DDC, 0x45CB, 0xAF, 0x5E, 0x0F, 0x7A, 0x8C, 0xE4, 0xDE, 0x77)]
interface IMFPluginControl2(IMFPluginControl2Vtbl):
  IMFPluginControl(IMFPluginControlVtbl) {
    fn SetPolicy(
        policy: MF_PLUGIN_CONTROL_POLICY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xeb533d5d, 0x2db6, 0x40f8, 0x97, 0xa9, 0x49, 0x46, 0x92, 0x01, 0x4f, 0x07)]
interface IMFDXGIDeviceManager(IMFDXGIDeviceManagerVtbl): IUnknown(IUnknownVtbl) {
    fn CloseDeviceHandle(
        hDevice: HANDLE,
    ) -> HRESULT,
    fn GetVideoService(
        hDevice: HANDLE,
        riid: REFIID,
        ppService: *mut *mut c_void,
    ) -> HRESULT,
    fn LockDevice(
        hDevice: HANDLE,
        riid: REFIID,
        ppUnkDevice: *mut *mut c_void,
        fBlock: BOOL,
    ) -> HRESULT,
    fn OpenDeviceHandle(
        phDevice: *mut HANDLE,
    ) -> HRESULT,
    fn ResetDevice(
        pUnkDevice: *mut IUnknown,
        resetToken: UINT,
    ) -> HRESULT,
    fn TestDevice(
        hDevice: HANDLE,
    ) -> HRESULT,
    fn UnlockDevice(
        hDevice: HANDLE,
        fSaveState: BOOL,
    ) -> HRESULT,
}}
ENUM!{enum MF_STREAM_STATE {
    MF_STREAM_STATE_STOPPED,
    MF_STREAM_STATE_PAUSED,
    MF_STREAM_STATE_RUNNING,
}}
RIDL!{#[uuid(0xCE8BD576, 0xE440, 0x43B3, 0xBE, 0x34, 0x1E, 0x53, 0xF5, 0x65, 0xF7, 0xE8)]
interface IMFMuxStreamAttributesManager(IMFMuxStreamAttributesManagerVtbl):
  IUnknown(IUnknownVtbl) {
    fn GetStreamCount(
        pdwMuxStreamCount: *mut DWORD,
    ) -> HRESULT,
    fn GetAttributes(
        dwMuxStreamIndex: DWORD,
        ppStreamAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x505A2C72, 0x42F7, 0x4690, 0xAE, 0xAB, 0x8F, 0x51, 0x3D, 0x0F, 0xFD, 0xB8)]
interface IMFMuxStreamMediaTypeManager(IMFMuxStreamMediaTypeManagerVtbl):
  IUnknown(IUnknownVtbl) {
    fn GetStreamCount(
        pdwMuxStreamCount: *mut DWORD,
    ) -> HRESULT,
    fn GetMediaType(
        dwMuxStreamIndex: DWORD,
        ppMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetStreamConfigurationCount(
        pdwCount: *mut DWORD,
    ) -> HRESULT,
    fn AddStreamConfiguration(
        ullStreamMask: ULONGLONG,
    ) -> HRESULT,
    fn RemoveStreamConfiguration(
        ullStreamMask: ULONGLONG,
    ) -> HRESULT,
    fn GetStreamConfiguration(
        ulIndex: DWORD,
        pullStreamMask: *mut ULONGLONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x74ABBC19, 0xB1CC, 0x4E41, 0xBB, 0x8B, 0x9D, 0x9B, 0x86, 0xA8, 0xF6, 0xCA)]
interface IMFMuxStreamSampleManager(IMFMuxStreamSampleManagerVtbl):
  IUnknown(IUnknownVtbl) {
    fn GetStreamCount(
        pdwMuxStreamCount: *mut DWORD,
    ) -> HRESULT,
    fn GetSample(
        dwMuxStreamIndex: DWORD,
        ppSample: *mut *mut IMFSample,
    ) -> HRESULT,
    fn GetStreamConfiguration() -> ULONGLONG,
}}
