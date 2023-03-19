
use egui::text::LayoutJob;

/// Memoized Code highlighting
pub fn highlight(ctx: &egui::Context, theme: &CodeTheme, code: &str, language: &str) -> LayoutJob {
    impl egui::util::cache::ComputerMut<(&CodeTheme, &str, &str), LayoutJob> for Highlighter {
        fn compute(&mut self, (theme, code, lang): (&CodeTheme, &str, &str)) -> LayoutJob {
            self.highlight(theme, code, lang)
        }
    }

    type HighlightCache = egui::util::cache::FrameCache<LayoutJob, Highlighter>;

    ctx.memory_mut(|mem| {
        mem.caches
            .cache::<HighlightCache>()
            .get((theme, code, language))
    })
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(enum_map::Enum)]
enum TokenType {
    Comment,
    Keyword,
    Literal,
    StringLiteral,
    Punctuation,
    Whitespace,
}

#[derive(Clone, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CodeTheme {
    dark_mode: bool,

    formats: enum_map::EnumMap<TokenType, egui::TextFormat>,
}

impl Default for CodeTheme {
    fn default() -> Self {
        Self::dark()
    }
}

impl CodeTheme {
    pub fn dark() -> Self {
        let font_id = egui::FontId::monospace(10.0);
        use egui::{Color32, TextFormat};
        Self {
            dark_mode: true,
            formats: enum_map::enum_map![
                TokenType::Comment => TextFormat::simple(font_id.clone(), Color32::from_gray(120)),
                TokenType::Keyword => TextFormat::simple(font_id.clone(), Color32::from_rgb(255, 100, 100)),
                TokenType::Literal => TextFormat::simple(font_id.clone(), Color32::from_rgb(87, 165, 171)),
                TokenType::StringLiteral => TextFormat::simple(font_id.clone(), Color32::from_rgb(109, 147, 226)),
                TokenType::Punctuation => TextFormat::simple(font_id.clone(), Color32::LIGHT_GRAY),
                TokenType::Whitespace => TextFormat::simple(font_id.clone(), Color32::TRANSPARENT),
            ],
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Default)]
struct Highlighter {}

impl Highlighter {
    fn highlight(&self, theme: &CodeTheme, mut text: &str, _language: &str) -> LayoutJob {
        // Extremely simple syntax highlighter for when we compile without syntect

        let mut job = LayoutJob::default();

        while !text.is_empty() {
            if text.starts_with("//") {
                let end = text.find('\n').unwrap_or(text.len());
                job.append(&text[..end], 0.0, theme.formats[TokenType::Comment].clone());
                text = &text[end..];
            } else if text.starts_with('"') {
                let end = text[1..]
                    .find('"')
                    .map(|i| i + 2)
                    .or_else(|| text.find('\n'))
                    .unwrap_or(text.len());
                job.append(
                    &text[..end],
                    0.0,
                    theme.formats[TokenType::StringLiteral].clone(),
                );
                text = &text[end..];
            } else if text.starts_with(|c: char| c.is_ascii_alphanumeric()) {
                let end = text[1..]
                    .find(|c: char| !c.is_ascii_alphanumeric())
                    .map_or_else(|| text.len(), |i| i + 1);
                let word = &text[..end];
                let tt = if is_keyword(word) {
                    TokenType::Keyword
                } else {
                    TokenType::Literal
                };
                job.append(word, 0.0, theme.formats[tt].clone());
                text = &text[end..];
            } else if text.starts_with(|c: char| c.is_ascii_whitespace()) {
                let end = text[1..]
                    .find(|c: char| !c.is_ascii_whitespace())
                    .map_or_else(|| text.len(), |i| i + 1);
                job.append(
                    &text[..end],
                    0.0,
                    theme.formats[TokenType::Whitespace].clone(),
                );
                text = &text[end..];
            } else {
                let mut it = text.char_indices();
                it.next();
                let end = it.next().map_or(text.len(), |(idx, _chr)| idx);
                job.append(
                    &text[..end],
                    0.0,
                    theme.formats[TokenType::Punctuation].clone(),
                );
                text = &text[end..];
            }
        }

        job
    }
}

fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "break" |
        "case" |
        "continue" |
        "default" |
        "discard" |
        "do" |
        "else" |
        "for" |
        "if" |
        "return" |
        "switch" |
        "while" |
        "abs" |
        "acos" |
        "all" |
        "any" |
        "asin" |
        "atan" |
        "ceil" |
        "clamp" |
        "cos" |
        "cross" |
        "degrees" |
        "dFdx" |
        "dFdy" |
        "distance" |
        "dot" |
        "equal" |
        "exp" |
        "exp2" |
        "faceforward" |
        "floor" |
        "fract" |
        "ftransform" |
        "fwidth" |
        "greaterThan" |
        "greaterThanEqual" |
        "inversesqrt" |
        "length" |
        "lessThan" |
        "lessThanEqual" |
        "log" |
        "log2" |
        "matrixCompMult" |
        "max" |
        "min" |
        "mix" |
        "mod" |
        "noise1" |
        "noise2" |
        "noise3" |
        "noise4" |
        "normalize" |
        "not" |
        "notEqual" |
        "outerProduct" |
        "pow" |
        "radians" |
        "reflect" |
        "refract" |
        "shadow1D" |
        "shadow1DLod" |
        "shadow1DProj" |
        "shadow1DProjLod" |
        "shadow2D" |
        "shadow2DLod" |
        "shadow2DProj" |
        "shadow2DProjLod" |
        "sign" |
        "sin" |
        "smoothstep" |
        "sqrt" |
        "step" |
        "tan" |
        "texture" |
        "texture1D" |
        "texture1DLod" |
        "texture1DProj" |
        "texture1DProjLod" |
        "texture2D" |
        "texture2DLod" |
        "texture2DProj" |
        "texture2DProjLod" |
        "texture3D" |
        "texture3DLod" |
        "texture3DProj" |
        "texture3DProjLod" |
        "textureCube" |
        "textureCubeLod" |
        "transpose" |
        "void" |
        "bool" |
        "int" |
        "uint" |
        "float" |
        "double" |
        "vec2" |
        "vec3" |
        "vec4" |
        "dvec2" |
        "dvec3" |
        "dvec4" |
        "bvec2" |
        "bvec3" |
        "bvec4" |
        "ivec2" |
        "ivec3" |
        "ivec4" |
        "uvec2" |
        "uvec3" |
        "uvec4" |
        "mat2" |
        "mat3" |
        "mat4" |
        "mat2x2" |
        "mat2x3" |
        "mat2x4" |
        "mat3x2" |
        "mat3x3" |
        "mat3x4" |
        "mat4x2" |
        "mat4x3" |
        "mat4x4" |
        "dmat2" |
        "dmat3" |
        "dmat4" |
        "dmat2x2" |
        "dmat2x3" |
        "dmat2x4" |
        "dmat3x2" |
        "dmat3x3" |
        "dmat3x4" |
        "dmat4x2" |
        "dmat4x3" |
        "dmat4x4" |
        "sampler1D" |
        "sampler2D" |
        "sampler3D" |
        "image1D" |
        "image2D" |
        "image3D" |
        "samplerCube" |
        "imageCube" |
        "sampler2DRect" |
        "image2DRect" |
        "sampler1DArray" |
        "sampler2DArray" |
        "image1DArray" |
        "image2DArray" |
        "samplerBuffer" |
        "imageBuffer" |
        "sampler2DMS" |
        "image2DMS" |
        "sampler2DMSArray" |
        "image2DMSArray" |
        "samplerCubeArray" |
        "imageCubeArray" |
        "sampler1DShadow" |
        "sampler2DShadow" |
        "sampler2DRectShadow" |
        "sampler1DArrayShadow" |
        "sampler2DArrayShadow" |
        "samplerCubeShadow" |
        "samplerCubeArrayShadow" |
        "isampler1D" |
        "isampler2D" |
        "isampler3D" |
        "iimage1D" |
        "iimage2D" |
        "iimage3D" |
        "isamplerCube" |
        "iimageCube" |
        "isampler2DRect" |
        "iimage2DRect" |
        "isampler1DArray" |
        "isampler2DArray" |
        "iimage1DArray" |
        "iimage2DArray" |
        "isamplerBuffer" |
        "iimageBuffer" |
        "isampler2DMS" |
        "iimage2DMS" |
        "isampler2DMSArray" |
        "iimage2DMSArray" |
        "isamplerCubeArray" |
        "iimageCubeArray" |
        "atomic_uint" |
        "usampler1D" |
        "usampler2D" |
        "usampler3D" |
        "uimage1D" |
        "uimage2D" |
        "uimage3D" |
        "usamplerCube" |
        "uimageCube" |
        "usampler2DRect" |
        "uimage2DRect" |
        "usampler1DArray" |
        "usampler2DArray" |
        "uimage1DArray" |
        "uimage2DArray" |
        "usamplerBuffer" |
        "uimageBuffer" |
        "usampler2DMS" |
        "uimage2DMS" |
        "usampler2DMSArray" |
        "uimage2DMSArray" |
        "usamplerCubeArray" |
        "uimageCubeArray"
    )
}
