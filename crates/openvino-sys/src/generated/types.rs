/* automatically generated by rust-bindgen 0.61.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ie_core {
    _unused: [u8; 0],
}
pub type ie_core_t = ie_core;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ie_network {
    _unused: [u8; 0],
}
pub type ie_network_t = ie_network;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ie_executable {
    _unused: [u8; 0],
}
pub type ie_executable_network_t = ie_executable;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ie_infer_request {
    _unused: [u8; 0],
}
pub type ie_infer_request_t = ie_infer_request;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ie_blob {
    _unused: [u8; 0],
}
pub type ie_blob_t = ie_blob;
#[doc = " @struct ie_version"]
#[doc = " @brief Represents an API version information that reflects the set of supported features"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ie_version {
    #[doc = "!< A string representing Inference Engine version"]
    pub api_version: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_ie_version() {
    const UNINIT: ::std::mem::MaybeUninit<ie_version> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ie_version>(),
        8usize,
        concat!("Size of: ", stringify!(ie_version))
    );
    assert_eq!(
        ::std::mem::align_of::<ie_version>(),
        8usize,
        concat!("Alignment of ", stringify!(ie_version))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).api_version) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_version),
            "::",
            stringify!(api_version)
        )
    );
}
#[doc = " @struct ie_version"]
#[doc = " @brief Represents an API version information that reflects the set of supported features"]
pub type ie_version_t = ie_version;
#[doc = " @struct ie_core_version"]
#[doc = " @brief  Represents version information that describes devices and the inference engine runtime library"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ie_core_version {
    #[doc = "!< A major version"]
    pub major: usize,
    #[doc = "!< A minor version"]
    pub minor: usize,
    #[doc = "!< A device name"]
    pub device_name: *const ::std::os::raw::c_char,
    #[doc = "!< A build number"]
    pub build_number: *const ::std::os::raw::c_char,
    #[doc = "!< A device description"]
    pub description: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_ie_core_version() {
    const UNINIT: ::std::mem::MaybeUninit<ie_core_version> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ie_core_version>(),
        40usize,
        concat!("Size of: ", stringify!(ie_core_version))
    );
    assert_eq!(
        ::std::mem::align_of::<ie_core_version>(),
        8usize,
        concat!("Alignment of ", stringify!(ie_core_version))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_core_version),
            "::",
            stringify!(major)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).minor) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_core_version),
            "::",
            stringify!(minor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).device_name) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_core_version),
            "::",
            stringify!(device_name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).build_number) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_core_version),
            "::",
            stringify!(build_number)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).description) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_core_version),
            "::",
            stringify!(description)
        )
    );
}
#[doc = " @struct ie_core_version"]
#[doc = " @brief  Represents version information that describes devices and the inference engine runtime library"]
pub type ie_core_version_t = ie_core_version;
#[doc = " @struct ie_core_versions"]
#[doc = " @brief Represents all versions information that describes all devices and the inference engine runtime library"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ie_core_versions {
    #[doc = "!< An array of device versions"]
    pub versions: *mut ie_core_version_t,
    #[doc = "!< A number of versions in the array"]
    pub num_vers: usize,
}
#[test]
fn bindgen_test_layout_ie_core_versions() {
    const UNINIT: ::std::mem::MaybeUninit<ie_core_versions> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ie_core_versions>(),
        16usize,
        concat!("Size of: ", stringify!(ie_core_versions))
    );
    assert_eq!(
        ::std::mem::align_of::<ie_core_versions>(),
        8usize,
        concat!("Alignment of ", stringify!(ie_core_versions))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).versions) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_core_versions),
            "::",
            stringify!(versions)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).num_vers) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_core_versions),
            "::",
            stringify!(num_vers)
        )
    );
}
#[doc = " @struct ie_core_versions"]
#[doc = " @brief Represents all versions information that describes all devices and the inference engine runtime library"]
pub type ie_core_versions_t = ie_core_versions;
#[doc = " @struct ie_config"]
#[doc = " @brief Represents configuration information that describes devices"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ie_config {
    #[doc = "!< A configuration key"]
    pub name: *const ::std::os::raw::c_char,
    #[doc = "!< A configuration value"]
    pub value: *const ::std::os::raw::c_char,
    #[doc = "!< A pointer to the next configuration value"]
    pub next: *mut ie_config,
}
#[test]
fn bindgen_test_layout_ie_config() {
    const UNINIT: ::std::mem::MaybeUninit<ie_config> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ie_config>(),
        24usize,
        concat!("Size of: ", stringify!(ie_config))
    );
    assert_eq!(
        ::std::mem::align_of::<ie_config>(),
        8usize,
        concat!("Alignment of ", stringify!(ie_config))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_config),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_config),
            "::",
            stringify!(value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_config),
            "::",
            stringify!(next)
        )
    );
}
#[doc = " @struct ie_config"]
#[doc = " @brief Represents configuration information that describes devices"]
pub type ie_config_t = ie_config;
#[doc = " @struct ie_param"]
#[doc = " @brief metric and config parameters."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ie_param {
    pub __bindgen_anon_1: ie_param__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ie_param__bindgen_ty_1 {
    pub params: *mut ::std::os::raw::c_char,
    pub number: ::std::os::raw::c_uint,
    pub range_for_async_infer_request: [::std::os::raw::c_uint; 3usize],
    pub range_for_streams: [::std::os::raw::c_uint; 2usize],
}
#[test]
fn bindgen_test_layout_ie_param__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<ie_param__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ie_param__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(ie_param__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<ie_param__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(ie_param__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).params) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_param__bindgen_ty_1),
            "::",
            stringify!(params)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).number) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_param__bindgen_ty_1),
            "::",
            stringify!(number)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).range_for_async_infer_request) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_param__bindgen_ty_1),
            "::",
            stringify!(range_for_async_infer_request)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).range_for_streams) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_param__bindgen_ty_1),
            "::",
            stringify!(range_for_streams)
        )
    );
}
#[test]
fn bindgen_test_layout_ie_param() {
    assert_eq!(
        ::std::mem::size_of::<ie_param>(),
        16usize,
        concat!("Size of: ", stringify!(ie_param))
    );
    assert_eq!(
        ::std::mem::align_of::<ie_param>(),
        8usize,
        concat!("Alignment of ", stringify!(ie_param))
    );
}
#[doc = " @struct ie_param"]
#[doc = " @brief metric and config parameters."]
pub type ie_param_t = ie_param;
#[doc = " @struct ie_param_config"]
#[doc = " @brief Represents configuration parameter information"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ie_param_config {
    pub name: *mut ::std::os::raw::c_char,
    pub param: *mut ie_param_t,
}
#[test]
fn bindgen_test_layout_ie_param_config() {
    const UNINIT: ::std::mem::MaybeUninit<ie_param_config> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ie_param_config>(),
        16usize,
        concat!("Size of: ", stringify!(ie_param_config))
    );
    assert_eq!(
        ::std::mem::align_of::<ie_param_config>(),
        8usize,
        concat!("Alignment of ", stringify!(ie_param_config))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_param_config),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).param) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_param_config),
            "::",
            stringify!(param)
        )
    );
}
#[doc = " @struct ie_param_config"]
#[doc = " @brief Represents configuration parameter information"]
pub type ie_param_config_t = ie_param_config;
#[doc = " @struct dimensions"]
#[doc = " @brief Represents dimensions for input or output data"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dimensions {
    #[doc = "!< A runk representing a number of dimensions"]
    pub ranks: usize,
    #[doc = "!< An array of dimensions"]
    pub dims: [usize; 8usize],
}
#[test]
fn bindgen_test_layout_dimensions() {
    const UNINIT: ::std::mem::MaybeUninit<dimensions> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<dimensions>(),
        72usize,
        concat!("Size of: ", stringify!(dimensions))
    );
    assert_eq!(
        ::std::mem::align_of::<dimensions>(),
        8usize,
        concat!("Alignment of ", stringify!(dimensions))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ranks) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(dimensions),
            "::",
            stringify!(ranks)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dims) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(dimensions),
            "::",
            stringify!(dims)
        )
    );
}
#[doc = " @struct dimensions"]
#[doc = " @brief Represents dimensions for input or output data"]
pub type dimensions_t = dimensions;
#[repr(u32)]
#[doc = " @enum layout_e"]
#[doc = " @brief Layouts that the inference engine supports"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum layout_e {
    #[doc = "!< \"ANY\" layout"]
    ANY = 0,
    #[doc = "!< \"NCHW\" layout"]
    NCHW = 1,
    #[doc = "!< \"NHWC\" layout"]
    NHWC = 2,
    #[doc = "!< \"NCDHW\" layout"]
    NCDHW = 3,
    #[doc = "!< \"NDHWC\" layout"]
    NDHWC = 4,
    #[doc = "!< \"OIHW\" layout"]
    OIHW = 64,
    #[doc = "!< \"SCALAR\" layout"]
    SCALAR = 95,
    #[doc = "!< \"C\" layout"]
    C = 96,
    #[doc = "!< \"CHW\" layout"]
    CHW = 128,
    #[doc = "!< \"HW\" layout"]
    HW = 192,
    #[doc = "!< \"NC\" layout"]
    NC = 193,
    #[doc = "!< \"CN\" layout"]
    CN = 194,
    #[doc = "!< \"BLOCKED\" layout"]
    BLOCKED = 200,
}
#[repr(u32)]
#[doc = " @enum precision_e"]
#[doc = " @brief Precisions that the inference engine supports"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum precision_e {
    #[doc = "< Unspecified value. Used by default"]
    UNSPECIFIED = 255,
    #[doc = "< Mixed value. Can be received from network. No applicable for tensors"]
    MIXED = 0,
    #[doc = "< 32bit floating point value"]
    FP32 = 10,
    #[doc = "< 16bit floating point value"]
    FP16 = 11,
    #[doc = "< 64bit floating point value"]
    FP64 = 13,
    #[doc = "< 16bit specific signed fixed point precision"]
    Q78 = 20,
    #[doc = "< 16bit signed integer value"]
    I16 = 30,
    #[doc = "< 4bit unsigned integer value"]
    U4 = 39,
    #[doc = "< 8bit unsigned integer value"]
    U8 = 40,
    #[doc = "< 4bit signed integer value"]
    I4 = 49,
    #[doc = "< 8bit signed integer value"]
    I8 = 50,
    #[doc = "< 16bit unsigned integer value"]
    U16 = 60,
    #[doc = "< 32bit signed integer value"]
    I32 = 70,
    #[doc = "< 64bit signed integer value"]
    I64 = 72,
    #[doc = "< 64bit unsigned integer value"]
    U64 = 73,
    #[doc = "< 32bit unsigned integer value"]
    U32 = 74,
    #[doc = "< 1bit integer value"]
    BIN = 71,
    #[doc = "< custom precision has it's own name and size of elements"]
    CUSTOM = 80,
}
#[doc = " @struct tensor_desc"]
#[doc = " @brief Represents detailed information for a tensor"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tensor_desc {
    pub layout: layout_e,
    pub dims: dimensions_t,
    pub precision: precision_e,
}
#[test]
fn bindgen_test_layout_tensor_desc() {
    const UNINIT: ::std::mem::MaybeUninit<tensor_desc> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tensor_desc>(),
        88usize,
        concat!("Size of: ", stringify!(tensor_desc))
    );
    assert_eq!(
        ::std::mem::align_of::<tensor_desc>(),
        8usize,
        concat!("Alignment of ", stringify!(tensor_desc))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).layout) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tensor_desc),
            "::",
            stringify!(layout)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dims) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tensor_desc),
            "::",
            stringify!(dims)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).precision) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(tensor_desc),
            "::",
            stringify!(precision)
        )
    );
}
#[doc = " @struct tensor_desc"]
#[doc = " @brief Represents detailed information for a tensor"]
pub type tensor_desc_t = tensor_desc;
#[repr(u32)]
#[doc = " @enum colorformat_e"]
#[doc = " @brief Extra information about input color format for preprocessing"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum colorformat_e {
    #[doc = "!< Plain blob (default), no extra color processing required"]
    RAW = 0,
    #[doc = "!< RGB color format"]
    RGB = 1,
    #[doc = "!< BGR color format, default in DLDT"]
    BGR = 2,
    #[doc = "!< RGBX color format with X ignored during inference"]
    RGBX = 3,
    #[doc = "!< BGRX color format with X ignored during inference"]
    BGRX = 4,
    #[doc = "!< NV12 color format represented as compound Y+UV blob"]
    NV12 = 5,
    #[doc = "!< I420 color format represented as compound Y+U+V blob"]
    I420 = 6,
}
#[repr(u32)]
#[doc = " @enum resize_alg_e"]
#[doc = " @brief Represents the list of supported resize algorithms."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum resize_alg_e {
    #[doc = "!< \"No resize\" mode"]
    NO_RESIZE = 0,
    #[doc = "!< \"Bilinear resize\" mode"]
    RESIZE_BILINEAR = 1,
    #[doc = "!< \"Area resize\" mode"]
    RESIZE_AREA = 2,
}
pub const IEStatusCode_OK: IEStatusCode = 0;
pub const IEStatusCode_GENERAL_ERROR: IEStatusCode = -1;
pub const IEStatusCode_NOT_IMPLEMENTED: IEStatusCode = -2;
pub const IEStatusCode_NETWORK_NOT_LOADED: IEStatusCode = -3;
pub const IEStatusCode_PARAMETER_MISMATCH: IEStatusCode = -4;
pub const IEStatusCode_NOT_FOUND: IEStatusCode = -5;
pub const IEStatusCode_OUT_OF_BOUNDS: IEStatusCode = -6;
pub const IEStatusCode_UNEXPECTED: IEStatusCode = -7;
pub const IEStatusCode_REQUEST_BUSY: IEStatusCode = -8;
pub const IEStatusCode_RESULT_NOT_READY: IEStatusCode = -9;
pub const IEStatusCode_NOT_ALLOCATED: IEStatusCode = -10;
pub const IEStatusCode_INFER_NOT_STARTED: IEStatusCode = -11;
pub const IEStatusCode_NETWORK_NOT_READ: IEStatusCode = -12;
pub const IEStatusCode_INFER_CANCELLED: IEStatusCode = -13;
#[doc = " @enum IEStatusCode"]
#[doc = " @brief This enum contains codes for all possible return values of the interface functions"]
pub type IEStatusCode = ::std::os::raw::c_int;
#[doc = " @struct roi_t"]
#[doc = " @brief This structure describes roi data."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct roi {
    #[doc = "!< ID of a roi"]
    pub id: usize,
    #[doc = "!< W upper left coordinate of roi"]
    pub posX: usize,
    #[doc = "!< H upper left coordinate of roi"]
    pub posY: usize,
    #[doc = "!< W size of roi"]
    pub sizeX: usize,
    #[doc = "!< H size of roi"]
    pub sizeY: usize,
}
#[test]
fn bindgen_test_layout_roi() {
    const UNINIT: ::std::mem::MaybeUninit<roi> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<roi>(),
        40usize,
        concat!("Size of: ", stringify!(roi))
    );
    assert_eq!(
        ::std::mem::align_of::<roi>(),
        8usize,
        concat!("Alignment of ", stringify!(roi))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(roi), "::", stringify!(id))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).posX) as usize - ptr as usize },
        8usize,
        concat!("Offset of field: ", stringify!(roi), "::", stringify!(posX))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).posY) as usize - ptr as usize },
        16usize,
        concat!("Offset of field: ", stringify!(roi), "::", stringify!(posY))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sizeX) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(roi),
            "::",
            stringify!(sizeX)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sizeY) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(roi),
            "::",
            stringify!(sizeY)
        )
    );
}
#[doc = " @struct roi_t"]
#[doc = " @brief This structure describes roi data."]
pub type roi_t = roi;
#[doc = " @struct input_shape"]
#[doc = " @brief Represents shape for input data"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct input_shape {
    pub name: *mut ::std::os::raw::c_char,
    pub shape: dimensions_t,
}
#[test]
fn bindgen_test_layout_input_shape() {
    const UNINIT: ::std::mem::MaybeUninit<input_shape> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<input_shape>(),
        80usize,
        concat!("Size of: ", stringify!(input_shape))
    );
    assert_eq!(
        ::std::mem::align_of::<input_shape>(),
        8usize,
        concat!("Alignment of ", stringify!(input_shape))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(input_shape),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).shape) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(input_shape),
            "::",
            stringify!(shape)
        )
    );
}
#[doc = " @struct input_shape"]
#[doc = " @brief Represents shape for input data"]
pub type input_shape_t = input_shape;
#[doc = " @struct input_shapes"]
#[doc = " @brief Represents shapes for all input data"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct input_shapes {
    pub shapes: *mut input_shape_t,
    pub shape_num: usize,
}
#[test]
fn bindgen_test_layout_input_shapes() {
    const UNINIT: ::std::mem::MaybeUninit<input_shapes> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<input_shapes>(),
        16usize,
        concat!("Size of: ", stringify!(input_shapes))
    );
    assert_eq!(
        ::std::mem::align_of::<input_shapes>(),
        8usize,
        concat!("Alignment of ", stringify!(input_shapes))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).shapes) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(input_shapes),
            "::",
            stringify!(shapes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).shape_num) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(input_shapes),
            "::",
            stringify!(shape_num)
        )
    );
}
#[doc = " @struct input_shapes"]
#[doc = " @brief Represents shapes for all input data"]
pub type input_shapes_t = input_shapes;
#[doc = " @struct ie_blob_buffer"]
#[doc = " @brief Represents copied data from the given blob."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ie_blob_buffer {
    pub __bindgen_anon_1: ie_blob_buffer__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ie_blob_buffer__bindgen_ty_1 {
    #[doc = "!< buffer can be written"]
    pub buffer: *mut ::std::os::raw::c_void,
    #[doc = "!< cbuffer is read-only"]
    pub cbuffer: *const ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_ie_blob_buffer__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<ie_blob_buffer__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ie_blob_buffer__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(ie_blob_buffer__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<ie_blob_buffer__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(ie_blob_buffer__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buffer) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_blob_buffer__bindgen_ty_1),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cbuffer) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_blob_buffer__bindgen_ty_1),
            "::",
            stringify!(cbuffer)
        )
    );
}
#[test]
fn bindgen_test_layout_ie_blob_buffer() {
    assert_eq!(
        ::std::mem::size_of::<ie_blob_buffer>(),
        8usize,
        concat!("Size of: ", stringify!(ie_blob_buffer))
    );
    assert_eq!(
        ::std::mem::align_of::<ie_blob_buffer>(),
        8usize,
        concat!("Alignment of ", stringify!(ie_blob_buffer))
    );
}
#[doc = " @struct ie_blob_buffer"]
#[doc = " @brief Represents copied data from the given blob."]
pub type ie_blob_buffer_t = ie_blob_buffer;
#[doc = " @struct ie_complete_call_back"]
#[doc = " @brief Completion callback definition about the function and args"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ie_complete_call_back {
    pub completeCallBackFunc:
        ::std::option::Option<unsafe extern "C" fn(args: *mut ::std::os::raw::c_void)>,
    pub args: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_ie_complete_call_back() {
    const UNINIT: ::std::mem::MaybeUninit<ie_complete_call_back> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ie_complete_call_back>(),
        16usize,
        concat!("Size of: ", stringify!(ie_complete_call_back))
    );
    assert_eq!(
        ::std::mem::align_of::<ie_complete_call_back>(),
        8usize,
        concat!("Alignment of ", stringify!(ie_complete_call_back))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).completeCallBackFunc) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_complete_call_back),
            "::",
            stringify!(completeCallBackFunc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).args) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_complete_call_back),
            "::",
            stringify!(args)
        )
    );
}
#[doc = " @struct ie_complete_call_back"]
#[doc = " @brief Completion callback definition about the function and args"]
pub type ie_complete_call_back_t = ie_complete_call_back;
#[doc = " @struct ie_available_devices"]
#[doc = " @brief Represent all available devices."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ie_available_devices {
    pub devices: *mut *mut ::std::os::raw::c_char,
    pub num_devices: usize,
}
#[test]
fn bindgen_test_layout_ie_available_devices() {
    const UNINIT: ::std::mem::MaybeUninit<ie_available_devices> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ie_available_devices>(),
        16usize,
        concat!("Size of: ", stringify!(ie_available_devices))
    );
    assert_eq!(
        ::std::mem::align_of::<ie_available_devices>(),
        8usize,
        concat!("Alignment of ", stringify!(ie_available_devices))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).devices) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_available_devices),
            "::",
            stringify!(devices)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).num_devices) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ie_available_devices),
            "::",
            stringify!(num_devices)
        )
    );
}
#[doc = " @struct ie_available_devices"]
#[doc = " @brief Represent all available devices."]
pub type ie_available_devices_t = ie_available_devices;
