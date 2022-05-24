
use miette::Result;

//use wasm_bindgen::prelude::wasm_bindgen;

use crate::model_construction::smoother::Smoother;
use crate::api::SmoothingOperation;
use smoothing_operation_derive::SmoothingOperation;

#[derive(SmoothingOperation)]
pub struct SmoothingOperationApplySmoothToLayer {
    pub layer: usize,
    pub strength: f32,
    pub coverage: usize,
    pub svc_weight: usize,
    pub allow_svc_change: bool
}