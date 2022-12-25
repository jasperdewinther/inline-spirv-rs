#![allow(dead_code)]
use std::path::PathBuf;
mod backends;

enum InputSourceLanguage {
    Unknown,
    Glsl,
    Hlsl,
}
#[derive(Clone, Copy)]
pub enum TargetSpirvVersion {
    Spirv1_0,
    #[allow(dead_code)]
    Spirv1_1,
    #[allow(dead_code)]
    Spirv1_2,
    Spirv1_3,
    #[allow(dead_code)]
    Spirv1_4,
    Spirv1_5,
}
#[derive(Clone, Copy)]
enum TargetEnvironmentType {
    Vulkan,
}
#[derive(Clone, Copy)]
pub enum OptimizationLevel {
    MinSize,
    MaxPerformance,
    None,
}
#[derive(Clone, Copy)]
pub enum ShaderKind {
    Unknown,

    Vertex,
    TesselationControl,
    TesselationEvaluation,
    Geometry,
    Fragment,
    Compute,
    // Mesh Pipeline
    Mesh,
    Task,
    // Ray-tracing Pipeline
    RayGeneration,
    Intersection,
    AnyHit,
    ClosestHit,
    Miss,
    Callable,
}

pub struct ShaderCompilationConfig {
    lang: InputSourceLanguage,
    incl_dirs: Vec<PathBuf>,
    pub defs: Vec<(String, Option<String>)>,
    spv_ver: TargetSpirvVersion,
    env_ty: TargetEnvironmentType,
    entry: String,
    optim_lv: OptimizationLevel,
    pub debug: bool,
    pub kind: ShaderKind,
    auto_bind: bool,
}
impl Default for ShaderCompilationConfig {
    fn default() -> Self {
        ShaderCompilationConfig {
            lang: InputSourceLanguage::Glsl,
            incl_dirs: Vec::new(),
            defs: Vec::new(),
            spv_ver: TargetSpirvVersion::Spirv1_5,
            env_ty: TargetEnvironmentType::Vulkan,
            entry: "main".to_owned(),
            optim_lv: OptimizationLevel::None,
            debug: false,
            kind: ShaderKind::Unknown,
            auto_bind: false,
        }
    }
}

struct CompilationFeedback {
    spv: Vec<u32>,
}

pub fn runtime_compile(src: &str, path: Option<&str>, cfg: &ShaderCompilationConfig) -> Result<Vec<u32>, String> {
    match backends::shaderc::compile(src, path, cfg) {
        Ok(feedback) => Ok(feedback.spv),
        Err(error) => Err(error),
    }
}
