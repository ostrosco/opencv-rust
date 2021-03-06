//! # Extra 2D Features Framework
//! # Experimental 2D Features Algorithms
//!
//! This section describes experimental algorithms for 2d feature detection.
//!
//! # Non-free 2D Features Algorithms
//!
//! This section describes two popular algorithms for 2d feature detection, SIFT and SURF, that are
//! known to be patented. You need to set the OPENCV_ENABLE_NONFREE option in cmake to use those. Use them at your own risk.
//!
//! # Experimental 2D Features Matching Algorithm
//!
//! This section describes the GMS (Grid-based Motion Statistics) matching strategy.
use crate::{mod_prelude::*, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};

pub const DAISY_NRM_FULL: i32 = 102;
pub const DAISY_NRM_NONE: i32 = 100;
pub const DAISY_NRM_PARTIAL: i32 = 101;
pub const DAISY_NRM_SIFT: i32 = 103;
pub const FREAK_NB_ORIENPAIRS: i32 = 45;
pub const FREAK_NB_PAIRS: i32 = 512;
pub const FREAK_NB_SCALES: i32 = 64;
/// ![block formula](https://latex.codecogs.com/png.latex?%20e%5E%7B%20-%5Calpha%20%2A%20d%5E2%28c_i%2C%20c_j%29%7D%20)
pub const PCTSignatures_GAUSSIAN: i32 = 1;
/// ![block formula](https://latex.codecogs.com/png.latex?%20%5Cfrac%7B1%7D%7B%5Calpha%20%2B%20d%28c_i%2C%20c_j%29%7D%20)
pub const PCTSignatures_HEURISTIC: i32 = 2;
pub const PCTSignatures_L0_25: i32 = 0;
pub const PCTSignatures_L0_5: i32 = 1;
pub const PCTSignatures_L1: i32 = 2;
pub const PCTSignatures_L2: i32 = 3;
pub const PCTSignatures_L2SQUARED: i32 = 4;
pub const PCTSignatures_L5: i32 = 5;
pub const PCTSignatures_L_INFINITY: i32 = 6;
/// ![block formula](https://latex.codecogs.com/png.latex?%20-d%28c_i%2C%20c_j%29%20)
pub const PCTSignatures_MINUS: i32 = 0;
/// Generate points with normal (gaussian) distribution.
pub const PCTSignatures_NORMAL: i32 = 2;
/// Generate points in a regular grid.
pub const PCTSignatures_REGULAR: i32 = 1;
/// Generate numbers uniformly.
pub const PCTSignatures_UNIFORM: i32 = 0;

/// Estimates cornerness for prespecified KeyPoints using the FAST algorithm
///
/// ## Parameters
/// * image: grayscale image where keypoints (corners) are detected.
/// * keypoints: keypoints which should be tested to fit the FAST criteria. Keypoints not being
/// detected as corners are removed.
/// * threshold: threshold on difference between intensity of the central pixel and pixels of a
/// circle around this pixel.
/// * nonmaxSuppression: if true, non-maximum suppression is applied to detected corners
/// (keypoints).
/// * type: one of the three neighborhoods as defined in the paper:
/// FastFeatureDetector::TYPE_9_16, FastFeatureDetector::TYPE_7_12,
/// FastFeatureDetector::TYPE_5_8
///
/// Detects corners using the FAST algorithm by [Rosten06](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Rosten06) .
///
/// ## C++ default parameters
/// * nonmax_suppression: true
/// * _type: FastFeatureDetector::TYPE_9_16
pub fn fast_for_point_set(image: &dyn core::ToInputArray, keypoints: &mut types::VectorOfKeyPoint, threshold: i32, nonmax_suppression: bool, _type: i32) -> Result<()> {
    input_array_arg!(image);
    unsafe { sys::cv_xfeatures2d_FASTForPointSet__InputArray_VectorOfKeyPoint_int_bool_int(image.as_raw__InputArray(), keypoints.as_raw_VectorOfKeyPoint(), threshold, nonmax_suppression, _type) }.into_result()
}

/// GMS  (Grid-based Motion Statistics) feature matching strategy by [Bian2017gms](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Bian2017gms) .
/// ## Parameters
/// * size1: Input size of image1.
/// * size2: Input size of image2.
/// * keypoints1: Input keypoints of image1.
/// * keypoints2: Input keypoints of image2.
/// * matches1to2: Input 1-nearest neighbor matches.
/// * matchesGMS: Matches returned by the GMS matching strategy.
/// * withRotation: Take rotation transformation into account.
/// * withScale: Take scale transformation into account.
/// * thresholdFactor: The higher, the less matches.
///
/// Note:
/// Since GMS works well when the number of features is large, we recommend to use the ORB feature and set FastThreshold to 0 to get as many as possible features quickly.
/// If matching results are not satisfying, please add more features. (We use 10000 for images with 640 X 480).
/// If your images have big rotation and scale changes, please set withRotation or withScale to true.
///
/// ## C++ default parameters
/// * with_rotation: false
/// * with_scale: false
/// * threshold_factor: 6.0
pub fn match_gms(size1: core::Size, size2: core::Size, keypoints1: &types::VectorOfKeyPoint, keypoints2: &types::VectorOfKeyPoint, matches1to2: &types::VectorOfDMatch, matches_gms: &mut types::VectorOfDMatch, with_rotation: bool, with_scale: bool, threshold_factor: f64) -> Result<()> {
    unsafe { sys::cv_xfeatures2d_matchGMS_Size_Size_VectorOfKeyPoint_VectorOfKeyPoint_VectorOfDMatch_VectorOfDMatch_bool_bool_double(size1, size2, keypoints1.as_raw_VectorOfKeyPoint(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), matches_gms.as_raw_VectorOfDMatch(), with_rotation, with_scale, threshold_factor) }.into_result()
}

// Generating impl for trait crate::xfeatures2d::AffineFeature2D
/// Class implementing affine adaptation for key points.
///
/// A @ref FeatureDetector and a @ref DescriptorExtractor are wrapped to augment the
/// detected points with their affine invariant elliptic region and to compute
/// the feature descriptors on the regions after warping them into circles.
///
/// The interface is equivalent to @ref Feature2D, adding operations for
/// @ref Elliptic_KeyPoint "Elliptic_KeyPoints" instead of @ref KeyPoint "KeyPoints".
pub trait AffineFeature2D: crate::features2d::Feature2DTrait {
    fn as_raw_AffineFeature2D(&self) -> *mut c_void;
    /// Detects keypoints in the image using the wrapped detector and
    /// performs affine adaptation to augment them with their elliptic regions.
    ///
    /// ## C++ default parameters
    /// * mask: noArray()
    fn detect(&mut self, image: &dyn core::ToInputArray, keypoints: &mut types::VectorOfElliptic_KeyPoint, mask: &dyn core::ToInputArray) -> Result<()> {
        input_array_arg!(image);
        input_array_arg!(mask);
        unsafe { sys::cv_xfeatures2d_AffineFeature2D_detect__InputArray_VectorOfElliptic_KeyPoint__InputArray(self.as_raw_AffineFeature2D(), image.as_raw__InputArray(), keypoints.as_raw_VectorOfElliptic_KeyPoint(), mask.as_raw__InputArray()) }.into_result()
    }
    
    /// Detects keypoints and computes descriptors for their surrounding
    /// regions, after warping them into circles.
    ///
    /// ## C++ default parameters
    /// * use_provided_keypoints: false
    fn detect_and_compute(&mut self, image: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, keypoints: &mut types::VectorOfElliptic_KeyPoint, descriptors: &mut dyn core::ToOutputArray, use_provided_keypoints: bool) -> Result<()> {
        input_array_arg!(image);
        input_array_arg!(mask);
        output_array_arg!(descriptors);
        unsafe { sys::cv_xfeatures2d_AffineFeature2D_detectAndCompute__InputArray__InputArray_VectorOfElliptic_KeyPoint__OutputArray_bool(self.as_raw_AffineFeature2D(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keypoints.as_raw_VectorOfElliptic_KeyPoint(), descriptors.as_raw__OutputArray(), use_provided_keypoints) }.into_result()
    }
    
}

impl dyn AffineFeature2D + '_ {
    /// Creates an instance wrapping the given keypoint detector and
    /// descriptor extractor.
    pub fn create_with_extrator(keypoint_detector: &types::PtrOfFeature2D, descriptor_extractor: &types::PtrOfFeature2D) -> Result<types::PtrOfAffineFeature2D> {
        unsafe { sys::cv_xfeatures2d_AffineFeature2D_create_PtrOfFeature2D_PtrOfFeature2D(keypoint_detector.as_raw_PtrOfFeature2D(), descriptor_extractor.as_raw_PtrOfFeature2D()) }.into_result().map(|ptr| types::PtrOfAffineFeature2D { ptr })
    }
    
    /// Creates an instance where keypoint detector and descriptor
    /// extractor are identical.
    pub fn create(keypoint_detector: &types::PtrOfFeature2D) -> Result<types::PtrOfAffineFeature2D> {
        unsafe { sys::cv_xfeatures2d_AffineFeature2D_create_PtrOfFeature2D(keypoint_detector.as_raw_PtrOfFeature2D()) }.into_result().map(|ptr| types::PtrOfAffineFeature2D { ptr })
    }
    
}

// Generating impl for trait crate::xfeatures2d::BoostDesc
/// Class implementing BoostDesc (Learning Image Descriptors with Boosting), described in
/// [Trzcinski13a](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Trzcinski13a) and [Trzcinski13b](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Trzcinski13b).
///
/// ## Parameters
/// * desc: type of descriptor to use, BoostDesc::BINBOOST_256 is default (256 bit long dimension)
/// Available types are: BoostDesc::BGM, BoostDesc::BGM_HARD, BoostDesc::BGM_BILINEAR, BoostDesc::LBGM,
/// BoostDesc::BINBOOST_64, BoostDesc::BINBOOST_128, BoostDesc::BINBOOST_256
/// * use_orientation: sample patterns using keypoints orientation, enabled by default
/// * scale_factor: adjust the sampling window of detected keypoints
/// 6.25f is default and fits for KAZE, SURF detected keypoints window ratio
/// 6.75f should be the scale for SIFT detected keypoints window ratio
/// 5.00f should be the scale for AKAZE, MSD, AGAST, FAST, BRISK keypoints window ratio
/// 0.75f should be the scale for ORB keypoints ratio
/// 1.50f was the default in original implementation
///
///
/// Note: BGM is the base descriptor where each binary dimension is computed as the output of a single weak learner.
/// BGM_HARD and BGM_BILINEAR refers to same BGM but use different type of gradient binning. In the BGM_HARD that
/// use ASSIGN_HARD binning type the gradient is assigned to the nearest orientation bin. In the BGM_BILINEAR that use
/// ASSIGN_BILINEAR binning type the gradient is assigned to the two neighbouring bins. In the BGM and all other modes that use
/// ASSIGN_SOFT binning type the gradient is assigned to 8 nearest bins according to the cosine value between the gradient
/// angle and the bin center. LBGM (alias FP-Boost) is the floating point extension where each dimension is computed
/// as a linear combination of the weak learner responses. BINBOOST and subvariants are the binary extensions of LBGM
/// where each bit is computed as a thresholded linear combination of a set of weak learners.
/// BoostDesc header files (boostdesc_*.i) was exported from original binaries with export-boostdesc.py script from
/// samples subfolder.
pub trait BoostDesc: crate::features2d::Feature2DTrait {
    fn as_raw_BoostDesc(&self) -> *mut c_void;
    fn set_use_scale_orientation(&mut self, use_scale_orientation: bool) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_BoostDesc_setUseScaleOrientation_bool(self.as_raw_BoostDesc(), use_scale_orientation) }.into_result()
    }
    
    fn get_use_scale_orientation(&self) -> Result<bool> {
        unsafe { sys::cv_xfeatures2d_BoostDesc_getUseScaleOrientation_const(self.as_raw_BoostDesc()) }.into_result()
    }
    
    fn set_scale_factor(&mut self, scale_factor: f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_BoostDesc_setScaleFactor_float(self.as_raw_BoostDesc(), scale_factor) }.into_result()
    }
    
    fn get_scale_factor(&self) -> Result<f32> {
        unsafe { sys::cv_xfeatures2d_BoostDesc_getScaleFactor_const(self.as_raw_BoostDesc()) }.into_result()
    }
    
}

impl dyn BoostDesc + '_ {
    ///
    /// ## C++ default parameters
    /// * desc: BoostDesc::BINBOOST_256
    /// * use_scale_orientation: true
    /// * scale_factor: 6.25f
    pub fn create(desc: i32, use_scale_orientation: bool, scale_factor: f32) -> Result<types::PtrOfBoostDesc> {
        unsafe { sys::cv_xfeatures2d_BoostDesc_create_int_bool_float(desc, use_scale_orientation, scale_factor) }.into_result().map(|ptr| types::PtrOfBoostDesc { ptr })
    }
    
}

// boxed class cv::xfeatures2d::BriefDescriptorExtractor
/// Class for computing BRIEF descriptors described in [calon2010](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_calon2010) .
///
/// ## Parameters
/// * bytes: legth of the descriptor in bytes, valid values are: 16, 32 (default) or 64 .
/// * use_orientation: sample patterns using keypoints orientation, disabled by default.
pub struct BriefDescriptorExtractor {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BriefDescriptorExtractor {
    fn drop(&mut self) {
        unsafe { sys::cv_BriefDescriptorExtractor_delete(self.ptr) };
    }
}

impl BriefDescriptorExtractor {
    #[inline(always)] pub fn as_raw_BriefDescriptorExtractor(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BriefDescriptorExtractor {}

impl core::AlgorithmTrait for BriefDescriptorExtractor {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl BriefDescriptorExtractor {
    ///
    /// ## C++ default parameters
    /// * bytes: 32
    /// * use_orientation: false
    pub fn create(bytes: i32, use_orientation: bool) -> Result<types::PtrOfBriefDescriptorExtractor> {
        unsafe { sys::cv_xfeatures2d_BriefDescriptorExtractor_create_int_bool(bytes, use_orientation) }.into_result().map(|ptr| types::PtrOfBriefDescriptorExtractor { ptr })
    }
    
}

// Generating impl for trait crate::xfeatures2d::DAISY
/// Class implementing DAISY descriptor, described in [Tola10](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Tola10)
///
/// ## Parameters
/// * radius: radius of the descriptor at the initial scale
/// * q_radius: amount of radial range division quantity
/// * q_theta: amount of angular range division quantity
/// * q_hist: amount of gradient orientations range division quantity
/// * norm: choose descriptors normalization type, where
/// DAISY::NRM_NONE will not do any normalization (default),
/// DAISY::NRM_PARTIAL mean that histograms are normalized independently for L2 norm equal to 1.0,
/// DAISY::NRM_FULL mean that descriptors are normalized for L2 norm equal to 1.0,
/// DAISY::NRM_SIFT mean that descriptors are normalized for L2 norm equal to 1.0 but no individual one is bigger than 0.154 as in SIFT
/// * H: optional 3x3 homography matrix used to warp the grid of daisy but sampling keypoints remains unwarped on image
/// * interpolation: switch to disable interpolation for speed improvement at minor quality loss
/// * use_orientation: sample patterns using keypoints orientation, disabled by default.
pub trait DAISY: crate::features2d::Feature2DTrait {
    fn as_raw_DAISY(&self) -> *mut c_void;
    /// ## Parameters
    /// * image: image to extract descriptors
    /// * keypoints: of interest within image
    /// * descriptors: resulted descriptors array
    fn compute(&mut self, image: &dyn core::ToInputArray, keypoints: &mut types::VectorOfKeyPoint, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
        input_array_arg!(image);
        output_array_arg!(descriptors);
        unsafe { sys::cv_xfeatures2d_DAISY_compute__InputArray_VectorOfKeyPoint__OutputArray(self.as_raw_DAISY(), image.as_raw__InputArray(), keypoints.as_raw_VectorOfKeyPoint(), descriptors.as_raw__OutputArray()) }.into_result()
    }
    
    fn compute_1(&mut self, images: &dyn core::ToInputArray, keypoints: &mut types::VectorOfVectorOfKeyPoint, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
        input_array_arg!(images);
        output_array_arg!(descriptors);
        unsafe { sys::cv_xfeatures2d_DAISY_compute__InputArray_VectorOfVectorOfKeyPoint__OutputArray(self.as_raw_DAISY(), images.as_raw__InputArray(), keypoints.as_raw_VectorOfVectorOfKeyPoint(), descriptors.as_raw__OutputArray()) }.into_result()
    }
    
    /// ## Parameters
    /// * image: image to extract descriptors
    /// * roi: region of interest within image
    /// * descriptors: resulted descriptors array for roi image pixels
    fn compute_2(&mut self, image: &dyn core::ToInputArray, roi: core::Rect, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
        input_array_arg!(image);
        output_array_arg!(descriptors);
        unsafe { sys::cv_xfeatures2d_DAISY_compute__InputArray_Rect__OutputArray(self.as_raw_DAISY(), image.as_raw__InputArray(), roi, descriptors.as_raw__OutputArray()) }.into_result()
    }
    
    /// ## Parameters
    /// * image: image to extract descriptors
    /// * descriptors: resulted descriptors array for all image pixels
    fn compute_3(&mut self, image: &dyn core::ToInputArray, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
        input_array_arg!(image);
        output_array_arg!(descriptors);
        unsafe { sys::cv_xfeatures2d_DAISY_compute__InputArray__OutputArray(self.as_raw_DAISY(), image.as_raw__InputArray(), descriptors.as_raw__OutputArray()) }.into_result()
    }
    
    /// ## Parameters
    /// * y: position y on image
    /// * x: position x on image
    /// * orientation: orientation on image (0->360)
    /// * descriptor: supplied array for descriptor storage
    fn get_descriptor(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_float_X(self.as_raw_DAISY(), y, x, orientation, descriptor) }.into_result()
    }
    
    /// ## Parameters
    /// * y: position y on image
    /// * x: position x on image
    /// * orientation: orientation on image (0->360)
    /// * descriptor: supplied array for descriptor storage
    /// * H: homography matrix for warped grid
    fn get_descriptor_1(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32, h: &mut f64) -> Result<bool> {
        unsafe { sys::cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_float_X_double_X(self.as_raw_DAISY(), y, x, orientation, descriptor, h) }.into_result()
    }
    
    /// ## Parameters
    /// * y: position y on image
    /// * x: position x on image
    /// * orientation: orientation on image (0->360)
    /// * descriptor: supplied array for descriptor storage
    fn get_unnormalized_descriptor(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_float_X(self.as_raw_DAISY(), y, x, orientation, descriptor) }.into_result()
    }
    
    /// ## Parameters
    /// * y: position y on image
    /// * x: position x on image
    /// * orientation: orientation on image (0->360)
    /// * descriptor: supplied array for descriptor storage
    /// * H: homography matrix for warped grid
    fn get_unnormalized_descriptor_1(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32, h: &mut f64) -> Result<bool> {
        unsafe { sys::cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_float_X_double_X(self.as_raw_DAISY(), y, x, orientation, descriptor, h) }.into_result()
    }
    
}

impl dyn DAISY + '_ {
    ///
    /// ## C++ default parameters
    /// * radius: 15
    /// * q_radius: 3
    /// * q_theta: 8
    /// * q_hist: 8
    /// * norm: DAISY::NRM_NONE
    /// * h: noArray()
    /// * interpolation: true
    /// * use_orientation: false
    pub fn create(radius: f32, q_radius: i32, q_theta: i32, q_hist: i32, norm: i32, h: &dyn core::ToInputArray, interpolation: bool, use_orientation: bool) -> Result<types::PtrOfDAISY> {
        input_array_arg!(h);
        unsafe { sys::cv_xfeatures2d_DAISY_create_float_int_int_int_int__InputArray_bool_bool(radius, q_radius, q_theta, q_hist, norm, h.as_raw__InputArray(), interpolation, use_orientation) }.into_result().map(|ptr| types::PtrOfDAISY { ptr })
    }
    
}

// boxed class cv::xfeatures2d::Elliptic_KeyPoint
/// Elliptic region around an interest point.
pub struct Elliptic_KeyPoint {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for Elliptic_KeyPoint {
    fn drop(&mut self) {
        unsafe { sys::cv_Elliptic_KeyPoint_delete(self.ptr) };
    }
}

impl Elliptic_KeyPoint {
    #[inline(always)] pub fn as_raw_Elliptic_KeyPoint(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Elliptic_KeyPoint {}

impl Elliptic_KeyPoint {
    pub fn default() -> Result<crate::xfeatures2d::Elliptic_KeyPoint> {
        unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint() }.into_result().map(|ptr| crate::xfeatures2d::Elliptic_KeyPoint { ptr })
    }
    
    pub fn new(pt: core::Point2f, angle: f32, axes: core::Size, size: f32, si: f32) -> Result<crate::xfeatures2d::Elliptic_KeyPoint> {
        unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint_Point2f_float_Size_float_float(pt, angle, axes, size, si) }.into_result().map(|ptr| crate::xfeatures2d::Elliptic_KeyPoint { ptr })
    }
    
}

// boxed class cv::xfeatures2d::FREAK
/// Class implementing the FREAK (*Fast Retina Keypoint*) keypoint descriptor, described in [AOV12](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_AOV12) .
///
/// The algorithm propose a novel keypoint descriptor inspired by the human visual system and more
/// precisely the retina, coined Fast Retina Key- point (FREAK). A cascade of binary strings is
/// computed by efficiently comparing image intensities over a retinal sampling pattern. FREAKs are in
/// general faster to compute with lower memory load and also more robust than SIFT, SURF or BRISK.
/// They are competitive alternatives to existing keypoints in particular for embedded applications.
///
///
/// Note:
/// *   An example on how to use the FREAK descriptor can be found at
/// opencv_source_code/samples/cpp/freak_demo.cpp
pub struct FREAK {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for FREAK {
    fn drop(&mut self) {
        unsafe { sys::cv_FREAK_delete(self.ptr) };
    }
}

impl FREAK {
    #[inline(always)] pub fn as_raw_FREAK(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FREAK {}

impl core::AlgorithmTrait for FREAK {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl FREAK {
    /// ## Parameters
    /// * orientationNormalized: Enable orientation normalization.
    /// * scaleNormalized: Enable scale normalization.
    /// * patternScale: Scaling of the description pattern.
    /// * nOctaves: Number of octaves covered by the detected keypoints.
    /// * selectedPairs: (Optional) user defined selected pairs indexes,
    ///
    /// ## C++ default parameters
    /// * orientation_normalized: true
    /// * scale_normalized: true
    /// * pattern_scale: 22.0f
    /// * n_octaves: 4
    /// * selected_pairs: std::vector<int>()
    pub fn create(orientation_normalized: bool, scale_normalized: bool, pattern_scale: f32, n_octaves: i32, selected_pairs: &types::VectorOfint) -> Result<types::PtrOfFREAK> {
        unsafe { sys::cv_xfeatures2d_FREAK_create_bool_bool_float_int_VectorOfint(orientation_normalized, scale_normalized, pattern_scale, n_octaves, selected_pairs.as_raw_VectorOfint()) }.into_result().map(|ptr| types::PtrOfFREAK { ptr })
    }
    
}

// boxed class cv::xfeatures2d::HarrisLaplaceFeatureDetector
/// Class implementing the Harris-Laplace feature detector as described in [Mikolajczyk2004](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Mikolajczyk2004).
pub struct HarrisLaplaceFeatureDetector {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for HarrisLaplaceFeatureDetector {
    fn drop(&mut self) {
        unsafe { sys::cv_HarrisLaplaceFeatureDetector_delete(self.ptr) };
    }
}

impl HarrisLaplaceFeatureDetector {
    #[inline(always)] pub fn as_raw_HarrisLaplaceFeatureDetector(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for HarrisLaplaceFeatureDetector {}

impl core::AlgorithmTrait for HarrisLaplaceFeatureDetector {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl HarrisLaplaceFeatureDetector {
    /// Creates a new implementation instance.
    ///
    /// ## Parameters
    /// * numOctaves: the number of octaves in the scale-space pyramid
    /// * corn_thresh: the threshold for the Harris cornerness measure
    /// * DOG_thresh: the threshold for the Difference-of-Gaussians scale selection
    /// * maxCorners: the maximum number of corners to consider
    /// * num_layers: the number of intermediate scales per octave
    ///
    /// ## C++ default parameters
    /// * num_octaves: 6
    /// * corn_thresh: 0.01f
    /// * dog_thresh: 0.01f
    /// * max_corners: 5000
    /// * num_layers: 4
    pub fn create(num_octaves: i32, corn_thresh: f32, dog_thresh: f32, max_corners: i32, num_layers: i32) -> Result<types::PtrOfHarrisLaplaceFeatureDetector> {
        unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_create_int_float_float_int_int(num_octaves, corn_thresh, dog_thresh, max_corners, num_layers) }.into_result().map(|ptr| types::PtrOfHarrisLaplaceFeatureDetector { ptr })
    }
    
}

// boxed class cv::xfeatures2d::LATCH
/// latch Class for computing the LATCH descriptor.
/// If you find this code useful, please add a reference to the following paper in your work:
/// Gil Levi and Tal Hassner, "LATCH: Learned Arrangements of Three Patch Codes", arXiv preprint arXiv:1501.03719, 15 Jan. 2015
///
/// LATCH is a binary descriptor based on learned comparisons of triplets of image patches.
///
/// bytes is the size of the descriptor - can be 64, 32, 16, 8, 4, 2 or 1
/// rotationInvariance - whether or not the descriptor should compansate for orientation changes.
/// half_ssd_size - the size of half of the mini-patches size. For example, if we would like to compare triplets of patches of size 7x7x
/// then the half_ssd_size should be (7-1)/2 = 3.
/// sigma - sigma value for GaussianBlur smoothing of the source image. Source image will be used without smoothing in case sigma value is 0.
///
/// Note: the descriptor can be coupled with any keypoint extractor. The only demand is that if you use set rotationInvariance = True then
/// you will have to use an extractor which estimates the patch orientation (in degrees). Examples for such extractors are ORB and SIFT.
///
/// Note: a complete example can be found under /samples/cpp/tutorial_code/xfeatures2D/latch_match.cpp
pub struct LATCH {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for LATCH {
    fn drop(&mut self) {
        unsafe { sys::cv_LATCH_delete(self.ptr) };
    }
}

impl LATCH {
    #[inline(always)] pub fn as_raw_LATCH(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for LATCH {}

impl core::AlgorithmTrait for LATCH {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl LATCH {
    ///
    /// ## C++ default parameters
    /// * bytes: 32
    /// * rotation_invariance: true
    /// * half_ssd_size: 3
    /// * sigma: 2.0
    pub fn create(bytes: i32, rotation_invariance: bool, half_ssd_size: i32, sigma: f64) -> Result<types::PtrOfLATCH> {
        unsafe { sys::cv_xfeatures2d_LATCH_create_int_bool_int_double(bytes, rotation_invariance, half_ssd_size, sigma) }.into_result().map(|ptr| types::PtrOfLATCH { ptr })
    }
    
}

// boxed class cv::xfeatures2d::LUCID
/// Class implementing the locally uniform comparison image descriptor, described in [LUCID](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_LUCID)
///
/// An image descriptor that can be computed very fast, while being
/// about as robust as, for example, SURF or BRIEF.
///
///
/// Note: It requires a color image as input.
pub struct LUCID {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for LUCID {
    fn drop(&mut self) {
        unsafe { sys::cv_LUCID_delete(self.ptr) };
    }
}

impl LUCID {
    #[inline(always)] pub fn as_raw_LUCID(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for LUCID {}

impl core::AlgorithmTrait for LUCID {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl LUCID {
    /// ## Parameters
    /// * lucid_kernel: kernel for descriptor construction, where 1=3x3, 2=5x5, 3=7x7 and so forth
    /// * blur_kernel: kernel for blurring image prior to descriptor construction, where 1=3x3, 2=5x5, 3=7x7 and so forth
    ///
    /// ## C++ default parameters
    /// * lucid_kernel: 1
    /// * blur_kernel: 2
    pub fn create(lucid_kernel: i32, blur_kernel: i32) -> Result<types::PtrOfLUCID> {
        unsafe { sys::cv_xfeatures2d_LUCID_create_int_int(lucid_kernel, blur_kernel) }.into_result().map(|ptr| types::PtrOfLUCID { ptr })
    }
    
}

// boxed class cv::xfeatures2d::MSDDetector
/// Class implementing the MSD (*Maximal Self-Dissimilarity*) keypoint detector, described in [Tombari14](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Tombari14).
///
/// The algorithm implements a novel interest point detector stemming from the intuition that image patches
/// which are highly dissimilar over a relatively large extent of their surroundings hold the property of
/// being repeatable and distinctive. This concept of "contextual self-dissimilarity" reverses the key
/// paradigm of recent successful techniques such as the Local Self-Similarity descriptor and the Non-Local
/// Means filter, which build upon the presence of similar - rather than dissimilar - patches. Moreover,
/// it extends to contextual information the local self-dissimilarity notion embedded in established
/// detectors of corner-like interest points, thereby achieving enhanced repeatability, distinctiveness and
/// localization accuracy.
pub struct MSDDetector {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for MSDDetector {
    fn drop(&mut self) {
        unsafe { sys::cv_MSDDetector_delete(self.ptr) };
    }
}

impl MSDDetector {
    #[inline(always)] pub fn as_raw_MSDDetector(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MSDDetector {}

impl core::AlgorithmTrait for MSDDetector {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl MSDDetector {
    ///
    /// ## C++ default parameters
    /// * m_patch_radius: 3
    /// * m_search_area_radius: 5
    /// * m_nms_radius: 5
    /// * m_nms_scale_radius: 0
    /// * m_th_saliency: 250.0f
    /// * m_k_nn: 4
    /// * m_scale_factor: 1.25f
    /// * m_n_scales: -1
    /// * m_compute_orientation: false
    pub fn create(m_patch_radius: i32, m_search_area_radius: i32, m_nms_radius: i32, m_nms_scale_radius: i32, m_th_saliency: f32, m_k_nn: i32, m_scale_factor: f32, m_n_scales: i32, m_compute_orientation: bool) -> Result<types::PtrOfMSDDetector> {
        unsafe { sys::cv_xfeatures2d_MSDDetector_create_int_int_int_int_float_int_float_int_bool(m_patch_radius, m_search_area_radius, m_nms_radius, m_nms_scale_radius, m_th_saliency, m_k_nn, m_scale_factor, m_n_scales, m_compute_orientation) }.into_result().map(|ptr| types::PtrOfMSDDetector { ptr })
    }
    
}

// Generating impl for trait crate::xfeatures2d::PCTSignatures
/// Class implementing PCT (position-color-texture) signature extraction
///       as described in [KrulisLS16](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_KrulisLS16).
///       The algorithm is divided to a feature sampler and a clusterizer.
///       Feature sampler produces samples at given set of coordinates.
///       Clusterizer then produces clusters of these samples using k-means algorithm.
///       Resulting set of clusters is the signature of the input image.
///
///       A signature is an array of SIGNATURE_DIMENSION-dimensional points.
///       Used dimensions are:
///       weight, x, y position; lab color, contrast, entropy.
/// [KrulisLS16](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_KrulisLS16)
/// [BeecksUS10](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_BeecksUS10)
pub trait PCTSignatures: core::AlgorithmTrait {
    fn as_raw_PCTSignatures(&self) -> *mut c_void;
    /// Computes signature of given image.
    /// ## Parameters
    /// * image: Input image of CV_8U type.
    /// * signature: Output computed signature.
    fn compute_signature(&self, image: &dyn core::ToInputArray, signature: &mut dyn core::ToOutputArray) -> Result<()> {
        input_array_arg!(image);
        output_array_arg!(signature);
        unsafe { sys::cv_xfeatures2d_PCTSignatures_computeSignature_const__InputArray__OutputArray(self.as_raw_PCTSignatures(), image.as_raw__InputArray(), signature.as_raw__OutputArray()) }.into_result()
    }
    
    /// Computes signatures for multiple images in parallel.
    /// ## Parameters
    /// * images: Vector of input images of CV_8U type.
    /// * signatures: Vector of computed signatures.
    fn compute_signatures(&self, images: &types::VectorOfMat, signatures: &mut types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_computeSignatures_const_VectorOfMat_VectorOfMat(self.as_raw_PCTSignatures(), images.as_raw_VectorOfMat(), signatures.as_raw_VectorOfMat()) }.into_result()
    }
    
    /// Number of initial samples taken from the image.
    fn get_sample_count(&self) -> Result<i32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getSampleCount_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Color resolution of the greyscale bitmap represented in allocated bits
    ///       (i.e., value 4 means that 16 shades of grey are used).
    ///       The greyscale bitmap is used for computing contrast and entropy values.
    fn get_grayscale_bits(&self) -> Result<i32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getGrayscaleBits_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Color resolution of the greyscale bitmap represented in allocated bits
    ///       (i.e., value 4 means that 16 shades of grey are used).
    ///       The greyscale bitmap is used for computing contrast and entropy values.
    fn set_grayscale_bits(&mut self, grayscale_bits: i32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setGrayscaleBits_int(self.as_raw_PCTSignatures(), grayscale_bits) }.into_result()
    }
    
    /// Size of the texture sampling window used to compute contrast and entropy
    ///       (center of the window is always in the pixel selected by x,y coordinates
    ///       of the corresponding feature sample).
    fn get_window_radius(&self) -> Result<i32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getWindowRadius_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Size of the texture sampling window used to compute contrast and entropy
    ///       (center of the window is always in the pixel selected by x,y coordinates
    ///       of the corresponding feature sample).
    fn set_window_radius(&mut self, radius: i32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setWindowRadius_int(self.as_raw_PCTSignatures(), radius) }.into_result()
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
    ///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
    fn get_weight_x(&self) -> Result<f32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightX_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
    ///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
    fn set_weight_x(&mut self, weight: f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightX_float(self.as_raw_PCTSignatures(), weight) }.into_result()
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
    ///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
    fn get_weight_y(&self) -> Result<f32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightY_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
    ///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
    fn set_weight_y(&mut self, weight: f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightY_float(self.as_raw_PCTSignatures(), weight) }.into_result()
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
    ///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
    fn get_weight_l(&self) -> Result<f32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightL_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
    ///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
    fn set_weight_l(&mut self, weight: f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightL_float(self.as_raw_PCTSignatures(), weight) }.into_result()
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
    ///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
    fn get_weight_a(&self) -> Result<f32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightA_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
    ///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
    fn set_weight_a(&mut self, weight: f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightA_float(self.as_raw_PCTSignatures(), weight) }.into_result()
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
    ///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
    fn get_weight_b(&self) -> Result<f32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightB_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
    ///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
    fn set_weight_b(&mut self, weight: f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightB_float(self.as_raw_PCTSignatures(), weight) }.into_result()
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
    ///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
    fn get_weight_contrast(&self) -> Result<f32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightContrast_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
    ///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
    fn set_weight_contrast(&mut self, weight: f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightContrast_float(self.as_raw_PCTSignatures(), weight) }.into_result()
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
    ///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
    fn get_weight_entropy(&self) -> Result<f32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightEntropy_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
    ///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
    fn set_weight_entropy(&mut self, weight: f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightEntropy_float(self.as_raw_PCTSignatures(), weight) }.into_result()
    }
    
    /// Initial samples taken from the image.
    ///       These sampled features become the input for clustering.
    fn get_sampling_points(&self) -> Result<types::VectorOfPoint2f> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getSamplingPoints_const(self.as_raw_PCTSignatures()) }.into_result().map(|ptr| types::VectorOfPoint2f { ptr })
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space.
    /// ## Parameters
    /// * idx: ID of the weight
    /// * value: Value of the weight
    ///
    /// Note:
    ///       WEIGHT_IDX = 0;
    ///       X_IDX = 1;
    ///       Y_IDX = 2;
    ///       L_IDX = 3;
    ///       A_IDX = 4;
    ///       B_IDX = 5;
    ///       CONTRAST_IDX = 6;
    ///       ENTROPY_IDX = 7;
    fn set_weight(&mut self, idx: i32, value: f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeight_int_float(self.as_raw_PCTSignatures(), idx, value) }.into_result()
    }
    
    /// Weights (multiplicative constants) that linearly stretch individual axes of the feature space.
    /// ## Parameters
    /// * weights: Values of all weights.
    ///
    /// Note:
    ///       WEIGHT_IDX = 0;
    ///       X_IDX = 1;
    ///       Y_IDX = 2;
    ///       L_IDX = 3;
    ///       A_IDX = 4;
    ///       B_IDX = 5;
    ///       CONTRAST_IDX = 6;
    ///       ENTROPY_IDX = 7;
    fn set_weights(&mut self, weights: &types::VectorOffloat) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeights_VectorOffloat(self.as_raw_PCTSignatures(), weights.as_raw_VectorOffloat()) }.into_result()
    }
    
    /// Translations of the individual axes of the feature space.
    /// ## Parameters
    /// * idx: ID of the translation
    /// * value: Value of the translation
    ///
    /// Note:
    ///       WEIGHT_IDX = 0;
    ///       X_IDX = 1;
    ///       Y_IDX = 2;
    ///       L_IDX = 3;
    ///       A_IDX = 4;
    ///       B_IDX = 5;
    ///       CONTRAST_IDX = 6;
    ///       ENTROPY_IDX = 7;
    fn set_translation(&mut self, idx: i32, value: f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setTranslation_int_float(self.as_raw_PCTSignatures(), idx, value) }.into_result()
    }
    
    /// Translations of the individual axes of the feature space.
    /// ## Parameters
    /// * translations: Values of all translations.
    ///
    /// Note:
    ///       WEIGHT_IDX = 0;
    ///       X_IDX = 1;
    ///       Y_IDX = 2;
    ///       L_IDX = 3;
    ///       A_IDX = 4;
    ///       B_IDX = 5;
    ///       CONTRAST_IDX = 6;
    ///       ENTROPY_IDX = 7;
    fn set_translations(&mut self, translations: &types::VectorOffloat) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setTranslations_VectorOffloat(self.as_raw_PCTSignatures(), translations.as_raw_VectorOffloat()) }.into_result()
    }
    
    /// Sets sampling points used to sample the input image.
    /// ## Parameters
    /// * samplingPoints: Vector of sampling points in range [0..1)
    ///
    /// Note: Number of sampling points must be greater or equal to clusterization seed count.
    fn set_sampling_points(&mut self, sampling_points: &types::VectorOfPoint2f) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setSamplingPoints_VectorOfPoint2f(self.as_raw_PCTSignatures(), sampling_points.as_raw_VectorOfPoint2f()) }.into_result()
    }
    
    /// Initial seeds (initial number of clusters) for the k-means algorithm.
    fn get_init_seed_indexes(&self) -> Result<types::VectorOfint> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getInitSeedIndexes_const(self.as_raw_PCTSignatures()) }.into_result().map(|ptr| types::VectorOfint { ptr })
    }
    
    /// Initial seed indexes for the k-means algorithm.
    fn set_init_seed_indexes(&mut self, init_seed_indexes: &types::VectorOfint) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setInitSeedIndexes_VectorOfint(self.as_raw_PCTSignatures(), init_seed_indexes.as_raw_VectorOfint()) }.into_result()
    }
    
    /// Number of initial seeds (initial number of clusters) for the k-means algorithm.
    fn get_init_seed_count(&self) -> Result<i32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getInitSeedCount_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Number of iterations of the k-means clustering.
    ///       We use fixed number of iterations, since the modified clustering is pruning clusters
    ///       (not iteratively refining k clusters).
    fn get_iteration_count(&self) -> Result<i32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getIterationCount_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Number of iterations of the k-means clustering.
    ///       We use fixed number of iterations, since the modified clustering is pruning clusters
    ///       (not iteratively refining k clusters).
    fn set_iteration_count(&mut self, iteration_count: i32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setIterationCount_int(self.as_raw_PCTSignatures(), iteration_count) }.into_result()
    }
    
    /// Maximal number of generated clusters. If the number is exceeded,
    ///       the clusters are sorted by their weights and the smallest clusters are cropped.
    fn get_max_clusters_count(&self) -> Result<i32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getMaxClustersCount_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Maximal number of generated clusters. If the number is exceeded,
    ///       the clusters are sorted by their weights and the smallest clusters are cropped.
    fn set_max_clusters_count(&mut self, max_clusters_count: i32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setMaxClustersCount_int(self.as_raw_PCTSignatures(), max_clusters_count) }.into_result()
    }
    
    /// This parameter multiplied by the index of iteration gives lower limit for cluster size.
    ///       Clusters containing fewer points than specified by the limit have their centroid dismissed
    ///       and points are reassigned.
    fn get_cluster_min_size(&self) -> Result<i32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getClusterMinSize_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// This parameter multiplied by the index of iteration gives lower limit for cluster size.
    ///       Clusters containing fewer points than specified by the limit have their centroid dismissed
    ///       and points are reassigned.
    fn set_cluster_min_size(&mut self, cluster_min_size: i32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setClusterMinSize_int(self.as_raw_PCTSignatures(), cluster_min_size) }.into_result()
    }
    
    /// Threshold euclidean distance between two centroids.
    ///       If two cluster centers are closer than this distance,
    ///       one of the centroid is dismissed and points are reassigned.
    fn get_joining_distance(&self) -> Result<f32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getJoiningDistance_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Threshold euclidean distance between two centroids.
    ///       If two cluster centers are closer than this distance,
    ///       one of the centroid is dismissed and points are reassigned.
    fn set_joining_distance(&mut self, joining_distance: f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setJoiningDistance_float(self.as_raw_PCTSignatures(), joining_distance) }.into_result()
    }
    
    /// Remove centroids in k-means whose weight is lesser or equal to given threshold.
    fn get_drop_threshold(&self) -> Result<f32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getDropThreshold_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Remove centroids in k-means whose weight is lesser or equal to given threshold.
    fn set_drop_threshold(&mut self, drop_threshold: f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setDropThreshold_float(self.as_raw_PCTSignatures(), drop_threshold) }.into_result()
    }
    
    /// Distance function selector used for measuring distance between two points in k-means.
    fn get_distance_function(&self) -> Result<i32> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_getDistanceFunction_const(self.as_raw_PCTSignatures()) }.into_result()
    }
    
    /// Distance function selector used for measuring distance between two points in k-means.
    ///       Available: L0_25, L0_5, L1, L2, L2SQUARED, L5, L_INFINITY.
    fn set_distance_function(&mut self, distance_function: i32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_setDistanceFunction_int(self.as_raw_PCTSignatures(), distance_function) }.into_result()
    }
    
}

impl dyn PCTSignatures + '_ {
    /// Creates PCTSignatures algorithm using sample and seed count.
    ///       It generates its own sets of sampling points and clusterization seed indexes.
    /// ## Parameters
    /// * initSampleCount: Number of points used for image sampling.
    /// * initSeedCount: Number of initial clusterization seeds.
    ///       Must be lower or equal to initSampleCount
    /// * pointDistribution: Distribution of generated points. Default: UNIFORM.
    ///       Available: UNIFORM, REGULAR, NORMAL.
    /// ## Returns
    /// Created algorithm.
    ///
    /// ## C++ default parameters
    /// * init_sample_count: 2000
    /// * init_seed_count: 400
    /// * point_distribution: 0
    pub fn create(init_sample_count: i32, init_seed_count: i32, point_distribution: i32) -> Result<types::PtrOfPCTSignatures> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_create_int_int_int(init_sample_count, init_seed_count, point_distribution) }.into_result().map(|ptr| types::PtrOfPCTSignatures { ptr })
    }
    
    /// Creates PCTSignatures algorithm using pre-generated sampling points
    ///       and number of clusterization seeds. It uses the provided
    ///       sampling points and generates its own clusterization seed indexes.
    /// ## Parameters
    /// * initSamplingPoints: Sampling points used in image sampling.
    /// * initSeedCount: Number of initial clusterization seeds.
    ///       Must be lower or equal to initSamplingPoints.size().
    /// ## Returns
    /// Created algorithm.
    pub fn create_1(init_sampling_points: &types::VectorOfPoint2f, init_seed_count: i32) -> Result<types::PtrOfPCTSignatures> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_create_VectorOfPoint2f_int(init_sampling_points.as_raw_VectorOfPoint2f(), init_seed_count) }.into_result().map(|ptr| types::PtrOfPCTSignatures { ptr })
    }
    
    /// Creates PCTSignatures algorithm using pre-generated sampling points
    ///       and clusterization seeds indexes.
    /// ## Parameters
    /// * initSamplingPoints: Sampling points used in image sampling.
    /// * initClusterSeedIndexes: Indexes of initial clusterization seeds.
    ///       Its size must be lower or equal to initSamplingPoints.size().
    /// ## Returns
    /// Created algorithm.
    pub fn create_2(init_sampling_points: &types::VectorOfPoint2f, init_cluster_seed_indexes: &types::VectorOfint) -> Result<types::PtrOfPCTSignatures> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_create_VectorOfPoint2f_VectorOfint(init_sampling_points.as_raw_VectorOfPoint2f(), init_cluster_seed_indexes.as_raw_VectorOfint()) }.into_result().map(|ptr| types::PtrOfPCTSignatures { ptr })
    }
    
    /// Draws signature in the source image and outputs the result.
    ///       Signatures are visualized as a circle
    ///       with radius based on signature weight
    ///       and color based on signature color.
    ///       Contrast and entropy are not visualized.
    /// ## Parameters
    /// * source: Source image.
    /// * signature: Image signature.
    /// * result: Output result.
    /// * radiusToShorterSideRatio: Determines maximal radius of signature in the output image.
    /// * borderThickness: Border thickness of the visualized signature.
    ///
    /// ## C++ default parameters
    /// * radius_to_shorter_side_ratio: 1.0 / 8
    /// * border_thickness: 1
    pub fn draw_signature(source: &dyn core::ToInputArray, signature: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray, radius_to_shorter_side_ratio: f32, border_thickness: i32) -> Result<()> {
        input_array_arg!(source);
        input_array_arg!(signature);
        output_array_arg!(result);
        unsafe { sys::cv_xfeatures2d_PCTSignatures_drawSignature__InputArray__InputArray__OutputArray_float_int(source.as_raw__InputArray(), signature.as_raw__InputArray(), result.as_raw__OutputArray(), radius_to_shorter_side_ratio, border_thickness) }.into_result()
    }
    
    /// Generates initial sampling points according to selected point distribution.
    /// ## Parameters
    /// * initPoints: Output vector where the generated points will be saved.
    /// * count: Number of points to generate.
    /// * pointDistribution: Point distribution selector.
    ///       Available: UNIFORM, REGULAR, NORMAL.
    ///
    /// Note: Generated coordinates are in range [0..1)
    pub fn generate_init_points(init_points: &mut types::VectorOfPoint2f, count: i32, point_distribution: i32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignatures_generateInitPoints_VectorOfPoint2f_int_int(init_points.as_raw_VectorOfPoint2f(), count, point_distribution) }.into_result()
    }
    
}

// Generating impl for trait crate::xfeatures2d::PCTSignaturesSQFD
/// Class implementing Signature Quadratic Form Distance (SQFD).
/// @see Christian Beecks, Merih Seran Uysal, Thomas Seidl.
///   Signature quadratic form distance.
///   In Proceedings of the ACM International Conference on Image and Video Retrieval, pages 438-445.
///   ACM, 2010.
/// [BeecksUS10](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_BeecksUS10)
pub trait PCTSignaturesSQFD: core::AlgorithmTrait {
    fn as_raw_PCTSignaturesSQFD(&self) -> *mut c_void;
    /// Computes Signature Quadratic Form Distance of two signatures.
    /// ## Parameters
    /// * _signature0: The first signature.
    /// * _signature1: The second signature.
    fn compute_quadratic_form_distance(&self, _signature0: &dyn core::ToInputArray, _signature1: &dyn core::ToInputArray) -> Result<f32> {
        input_array_arg!(_signature0);
        input_array_arg!(_signature1);
        unsafe { sys::cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistance_const__InputArray__InputArray(self.as_raw_PCTSignaturesSQFD(), _signature0.as_raw__InputArray(), _signature1.as_raw__InputArray()) }.into_result()
    }
    
    /// Computes Signature Quadratic Form Distance between the reference signature
    ///       and each of the other image signatures.
    /// ## Parameters
    /// * sourceSignature: The signature to measure distance of other signatures from.
    /// * imageSignatures: Vector of signatures to measure distance from the source signature.
    /// * distances: Output vector of measured distances.
    fn compute_quadratic_form_distances(&self, source_signature: &core::Mat, image_signatures: &types::VectorOfMat, distances: &mut types::VectorOffloat) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistances_const_Mat_VectorOfMat_VectorOffloat(self.as_raw_PCTSignaturesSQFD(), source_signature.as_raw_Mat(), image_signatures.as_raw_VectorOfMat(), distances.as_raw_VectorOffloat()) }.into_result()
    }
    
}

impl dyn PCTSignaturesSQFD + '_ {
    /// Creates the algorithm instance using selected distance function,
    ///       similarity function and similarity function parameter.
    /// ## Parameters
    /// * distanceFunction: Distance function selector. Default: L2
    ///       Available: L0_25, L0_5, L1, L2, L2SQUARED, L5, L_INFINITY
    /// * similarityFunction: Similarity function selector. Default: HEURISTIC
    ///       Available: MINUS, GAUSSIAN, HEURISTIC
    /// * similarityParameter: Parameter of the similarity function.
    ///
    /// ## C++ default parameters
    /// * distance_function: 3
    /// * similarity_function: 2
    /// * similarity_parameter: 1.0f
    pub fn create(distance_function: i32, similarity_function: i32, similarity_parameter: f32) -> Result<types::PtrOfPCTSignaturesSQFD> {
        unsafe { sys::cv_xfeatures2d_PCTSignaturesSQFD_create_int_int_float(distance_function, similarity_function, similarity_parameter) }.into_result().map(|ptr| types::PtrOfPCTSignaturesSQFD { ptr })
    }
    
}

// boxed class cv::xfeatures2d::SIFT
/// Class for extracting keypoints and computing descriptors using the Scale Invariant Feature Transform
/// (SIFT) algorithm by D. Lowe [Lowe04](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Lowe04) .
pub struct SIFT {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for SIFT {
    fn drop(&mut self) {
        unsafe { sys::cv_SIFT_delete(self.ptr) };
    }
}

impl SIFT {
    #[inline(always)] pub fn as_raw_SIFT(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SIFT {}

impl core::AlgorithmTrait for SIFT {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl SIFT {
    /// ## Parameters
    /// * nfeatures: The number of best features to retain. The features are ranked by their scores
    /// (measured in SIFT algorithm as the local contrast)
    ///
    /// * nOctaveLayers: The number of layers in each octave. 3 is the value used in D. Lowe paper. The
    /// number of octaves is computed automatically from the image resolution.
    ///
    /// * contrastThreshold: The contrast threshold used to filter out weak features in semi-uniform
    /// (low-contrast) regions. The larger the threshold, the less features are produced by the detector.
    ///
    /// * edgeThreshold: The threshold used to filter out edge-like features. Note that the its meaning
    /// is different from the contrastThreshold, i.e. the larger the edgeThreshold, the less features are
    /// filtered out (more features are retained).
    ///
    /// * sigma: The sigma of the Gaussian applied to the input image at the octave \#0. If your image
    /// is captured with a weak camera with soft lenses, you might want to reduce the number.
    ///
    /// ## C++ default parameters
    /// * nfeatures: 0
    /// * n_octave_layers: 3
    /// * contrast_threshold: 0.04
    /// * edge_threshold: 10
    /// * sigma: 1.6
    pub fn create(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64) -> Result<types::PtrOfSIFT> {
        unsafe { sys::cv_xfeatures2d_SIFT_create_int_int_double_double_double(nfeatures, n_octave_layers, contrast_threshold, edge_threshold, sigma) }.into_result().map(|ptr| types::PtrOfSIFT { ptr })
    }
    
}

// Generating impl for trait crate::xfeatures2d::SURF
/// Class for extracting Speeded Up Robust Features from an image [Bay06](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Bay06) .
///
/// The algorithm parameters:
/// *   member int extended
/// *   0 means that the basic descriptors (64 elements each) shall be computed
/// *   1 means that the extended descriptors (128 elements each) shall be computed
/// *   member int upright
/// *   0 means that detector computes orientation of each feature.
/// *   1 means that the orientation is not computed (which is much, much faster). For example,
/// if you match images from a stereo pair, or do image stitching, the matched features
/// likely have very similar angles, and you can speed up feature extraction by setting
/// upright=1.
/// *   member double hessianThreshold
/// Threshold for the keypoint detector. Only features, whose hessian is larger than
/// hessianThreshold are retained by the detector. Therefore, the larger the value, the less
/// keypoints you will get. A good default value could be from 300 to 500, depending from the
/// image contrast.
/// *   member int nOctaves
/// The number of a gaussian pyramid octaves that the detector uses. It is set to 4 by default.
/// If you want to get very large features, use the larger value. If you want just small
/// features, decrease it.
/// *   member int nOctaveLayers
/// The number of images within each octave of a gaussian pyramid. It is set to 2 by default.
///
/// Note:
/// *   An example using the SURF feature detector can be found at
/// opencv_source_code/samples/cpp/generic_descriptor_match.cpp
/// *   Another example using the SURF feature detector, extractor and matcher can be found at
/// opencv_source_code/samples/cpp/matcher_simple.cpp
pub trait SURF: crate::features2d::Feature2DTrait {
    fn as_raw_SURF(&self) -> *mut c_void;
    fn set_hessian_threshold(&mut self, hessian_threshold: f64) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_SURF_setHessianThreshold_double(self.as_raw_SURF(), hessian_threshold) }.into_result()
    }
    
    fn get_hessian_threshold(&self) -> Result<f64> {
        unsafe { sys::cv_xfeatures2d_SURF_getHessianThreshold_const(self.as_raw_SURF()) }.into_result()
    }
    
    fn set_n_octaves(&mut self, n_octaves: i32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_SURF_setNOctaves_int(self.as_raw_SURF(), n_octaves) }.into_result()
    }
    
    fn get_n_octaves(&self) -> Result<i32> {
        unsafe { sys::cv_xfeatures2d_SURF_getNOctaves_const(self.as_raw_SURF()) }.into_result()
    }
    
    fn set_n_octave_layers(&mut self, n_octave_layers: i32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_SURF_setNOctaveLayers_int(self.as_raw_SURF(), n_octave_layers) }.into_result()
    }
    
    fn get_n_octave_layers(&self) -> Result<i32> {
        unsafe { sys::cv_xfeatures2d_SURF_getNOctaveLayers_const(self.as_raw_SURF()) }.into_result()
    }
    
    fn set_extended(&mut self, extended: bool) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_SURF_setExtended_bool(self.as_raw_SURF(), extended) }.into_result()
    }
    
    fn get_extended(&self) -> Result<bool> {
        unsafe { sys::cv_xfeatures2d_SURF_getExtended_const(self.as_raw_SURF()) }.into_result()
    }
    
    fn set_upright(&mut self, upright: bool) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_SURF_setUpright_bool(self.as_raw_SURF(), upright) }.into_result()
    }
    
    fn get_upright(&self) -> Result<bool> {
        unsafe { sys::cv_xfeatures2d_SURF_getUpright_const(self.as_raw_SURF()) }.into_result()
    }
    
}

impl dyn SURF + '_ {
    /// ## Parameters
    /// * hessianThreshold: Threshold for hessian keypoint detector used in SURF.
    /// * nOctaves: Number of pyramid octaves the keypoint detector will use.
    /// * nOctaveLayers: Number of octave layers within each octave.
    /// * extended: Extended descriptor flag (true - use extended 128-element descriptors; false - use
    /// 64-element descriptors).
    /// * upright: Up-right or rotated features flag (true - do not compute orientation of features;
    /// false - compute orientation).
    ///
    /// ## C++ default parameters
    /// * hessian_threshold: 100
    /// * n_octaves: 4
    /// * n_octave_layers: 3
    /// * extended: false
    /// * upright: false
    pub fn create(hessian_threshold: f64, n_octaves: i32, n_octave_layers: i32, extended: bool, upright: bool) -> Result<types::PtrOfSURF> {
        unsafe { sys::cv_xfeatures2d_SURF_create_double_int_int_bool_bool(hessian_threshold, n_octaves, n_octave_layers, extended, upright) }.into_result().map(|ptr| types::PtrOfSURF { ptr })
    }
    
}

// boxed class cv::xfeatures2d::StarDetector
/// The class implements the keypoint detector introduced by [Agrawal08](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Agrawal08), synonym of StarDetector. :
pub struct StarDetector {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for StarDetector {
    fn drop(&mut self) {
        unsafe { sys::cv_StarDetector_delete(self.ptr) };
    }
}

impl StarDetector {
    #[inline(always)] pub fn as_raw_StarDetector(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for StarDetector {}

impl core::AlgorithmTrait for StarDetector {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl StarDetector {
    /// the full constructor
    ///
    /// ## C++ default parameters
    /// * max_size: 45
    /// * response_threshold: 30
    /// * line_threshold_projected: 10
    /// * line_threshold_binarized: 8
    /// * suppress_nonmax_size: 5
    pub fn create(max_size: i32, response_threshold: i32, line_threshold_projected: i32, line_threshold_binarized: i32, suppress_nonmax_size: i32) -> Result<types::PtrOfStarDetector> {
        unsafe { sys::cv_xfeatures2d_StarDetector_create_int_int_int_int_int(max_size, response_threshold, line_threshold_projected, line_threshold_binarized, suppress_nonmax_size) }.into_result().map(|ptr| types::PtrOfStarDetector { ptr })
    }
    
}

// Generating impl for trait crate::xfeatures2d::VGG
/// Class implementing VGG (Oxford Visual Geometry Group) descriptor trained end to end
/// using "Descriptor Learning Using Convex Optimisation" (DLCO) aparatus described in [Simonyan14](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Simonyan14).
///
/// ## Parameters
/// * desc: type of descriptor to use, VGG::VGG_120 is default (120 dimensions float)
/// Available types are VGG::VGG_120, VGG::VGG_80, VGG::VGG_64, VGG::VGG_48
/// * isigma: gaussian kernel value for image blur (default is 1.4f)
/// * img_normalize: use image sample intensity normalization (enabled by default)
/// * use_orientation: sample patterns using keypoints orientation, enabled by default
/// * scale_factor: adjust the sampling window of detected keypoints to 64.0f (VGG sampling window)
/// 6.25f is default and fits for KAZE, SURF detected keypoints window ratio
/// 6.75f should be the scale for SIFT detected keypoints window ratio
/// 5.00f should be the scale for AKAZE, MSD, AGAST, FAST, BRISK keypoints window ratio
/// 0.75f should be the scale for ORB keypoints ratio
///
/// * dsc_normalize: clamp descriptors to 255 and convert to uchar CV_8UC1 (disabled by default)
pub trait VGG: crate::features2d::Feature2DTrait {
    fn as_raw_VGG(&self) -> *mut c_void;
    fn set_sigma(&mut self, isigma: f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_VGG_setSigma_float(self.as_raw_VGG(), isigma) }.into_result()
    }
    
    fn get_sigma(&self) -> Result<f32> {
        unsafe { sys::cv_xfeatures2d_VGG_getSigma_const(self.as_raw_VGG()) }.into_result()
    }
    
    fn set_use_normalize_image(&mut self, img_normalize: bool) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_VGG_setUseNormalizeImage_bool(self.as_raw_VGG(), img_normalize) }.into_result()
    }
    
    fn get_use_normalize_image(&self) -> Result<bool> {
        unsafe { sys::cv_xfeatures2d_VGG_getUseNormalizeImage_const(self.as_raw_VGG()) }.into_result()
    }
    
    fn set_use_scale_orientation(&mut self, use_scale_orientation: bool) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_VGG_setUseScaleOrientation_bool(self.as_raw_VGG(), use_scale_orientation) }.into_result()
    }
    
    fn get_use_scale_orientation(&self) -> Result<bool> {
        unsafe { sys::cv_xfeatures2d_VGG_getUseScaleOrientation_const(self.as_raw_VGG()) }.into_result()
    }
    
    fn set_scale_factor(&mut self, scale_factor: f32) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_VGG_setScaleFactor_float(self.as_raw_VGG(), scale_factor) }.into_result()
    }
    
    fn get_scale_factor(&self) -> Result<f32> {
        unsafe { sys::cv_xfeatures2d_VGG_getScaleFactor_const(self.as_raw_VGG()) }.into_result()
    }
    
    fn set_use_normalize_descriptor(&mut self, dsc_normalize: bool) -> Result<()> {
        unsafe { sys::cv_xfeatures2d_VGG_setUseNormalizeDescriptor_bool(self.as_raw_VGG(), dsc_normalize) }.into_result()
    }
    
    fn get_use_normalize_descriptor(&self) -> Result<bool> {
        unsafe { sys::cv_xfeatures2d_VGG_getUseNormalizeDescriptor_const(self.as_raw_VGG()) }.into_result()
    }
    
}

impl dyn VGG + '_ {
    ///
    /// ## C++ default parameters
    /// * desc: VGG::VGG_120
    /// * isigma: 1.4f
    /// * img_normalize: true
    /// * use_scale_orientation: true
    /// * scale_factor: 6.25f
    /// * dsc_normalize: false
    pub fn create(desc: i32, isigma: f32, img_normalize: bool, use_scale_orientation: bool, scale_factor: f32, dsc_normalize: bool) -> Result<types::PtrOfVGG> {
        unsafe { sys::cv_xfeatures2d_VGG_create_int_float_bool_bool_float_bool(desc, isigma, img_normalize, use_scale_orientation, scale_factor, dsc_normalize) }.into_result().map(|ptr| types::PtrOfVGG { ptr })
    }
    
}

