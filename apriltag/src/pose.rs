//! Pose types storing estimated rotation and translation parameters.

use crate::{common::*, MatdRef};

/// Estimated pose along with error.
pub struct PoseEstimation {
    pub pose: Pose,
    pub error: f64,
}

/// Estimated pose rotation and translation parameters.
#[repr(transparent)]
pub struct Pose(pub(crate) sys::apriltag_pose_t);

impl Pose {
    /// Gets the rotation matrix.
    pub fn rotation(&self) -> MatdRef<'_> {
        unsafe { MatdRef::from_ptr(self.0.R) }
    }

    /// Gets the translation matrix.
    pub fn translation(&self) -> MatdRef<'_> {
        unsafe { MatdRef::from_ptr(self.0.t) }
    }
}

impl Debug for Pose {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Pose")
            .field("rotation", &self.rotation())
            .field("translation", &self.translation())
            .finish()
    }
}

impl Drop for Pose {
    fn drop(&mut self) {
        unsafe {
            sys::matd_destroy(self.0.R);
            sys::matd_destroy(self.0.t);
        }
    }
}

/// Stores tag size and camera parameters.
#[derive(Debug, Clone)]
pub struct TagParams {
    pub tagsize: f64,
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
}

#[cfg(feature = "nalgebra")]
mod nalgebra_conv {
    use super::*;
    use nalgebra::{Isometry3, MatrixSlice3, MatrixSlice3x1, Translation3, UnitQuaternion};

    impl Pose {
        pub fn to_isometry(self: &Pose) -> Isometry3<f64> {
            let rotation = self.rotation();
            let translation = self.translation();

            let rotation =
                UnitQuaternion::from_matrix(&MatrixSlice3::from_slice(rotation.data()).transpose());

            let translation: Translation3<f64> = MatrixSlice3x1::from_slice(translation.data())
                .into_owned()
                .into();

            Isometry3::from_parts(translation, rotation)
        }
    }
}
