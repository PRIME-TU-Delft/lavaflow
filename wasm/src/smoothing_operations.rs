use crate::model_construction::smoother::Smoother;
use miette::{Result};

trait SmoothingOperation {
    fn apply(&self, smoother: &mut Smoother) -> Result<()>;
}

struct SmoothingOperationApplySmoothToLayer {
    layer: usize,
    strength: f32,
    coverage: usize,
    svc_weight: usize,
    allow_svc_change: bool
}

impl SmoothingOperation for SmoothingOperationApplySmoothToLayer {
    fn apply(&self, smoother: &mut Smoother) -> Result<()> {
        smoother.apply_smooth_to_layer(self.layer, self.strength, self.coverage, self.svc_weight, self.allow_svc_change)
    }
}