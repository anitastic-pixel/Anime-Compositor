//! G1-core colour, alpha and compositing (B-02, requirement R-10).
//!
//! Document 06 line 45: "An untagged buffer never crosses a module boundary." That is
//! enforced structurally here rather than by convention. [`ImageBuffer`] always carries its
//! [`ColorSpace`] and [`AlphaMode`]; the compositor in [`composite`] does not accept an
//! `ImageBuffer` at all. It accepts only a [`WorkingBuffer`], which cannot be constructed
//! except by converting a tagged buffer into the document 21 working space. Compositing two
//! images in the wrong space is therefore not a checked error, it is unrepresentable.

pub mod color;
pub mod composite;
pub mod diagnostics;
pub mod media;
pub mod time;

/// How the RGB channels of a buffer are encoded.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorSpace {
    /// Linear light. The working space of document 21.
    LinearLight,
    /// sRGB encoded, IEC 61966-2-1 transfer function. See [`color`].
    Srgb,
}

/// How the RGB channels relate to alpha.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AlphaMode {
    /// RGB has already been multiplied by alpha.
    Premultiplied,
    /// RGB is independent of alpha. Document 21: G1 PNG input is straight.
    Straight,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BufferError {
    /// `data.len()` was not `width * height * 4`.
    LengthMismatch { expected: usize, actual: usize },
}

impl std::fmt::Display for BufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BufferError::LengthMismatch { expected, actual } => write!(
                f,
                "buffer length {actual} does not match {expected} for the declared extent"
            ),
        }
    }
}

/// An RGBA float32 image that knows how it is encoded.
///
/// There is deliberately no way to build one without stating both tags.
#[derive(Clone, Debug)]
pub struct ImageBuffer {
    width: usize,
    height: usize,
    color_space: ColorSpace,
    alpha_mode: AlphaMode,
    /// RGBA, row-major, top-left origin (document 21 coordinate system).
    data: Vec<f32>,
}

impl ImageBuffer {
    pub fn new(
        width: usize,
        height: usize,
        color_space: ColorSpace,
        alpha_mode: AlphaMode,
        data: Vec<f32>,
    ) -> Result<Self, BufferError> {
        let expected = width * height * 4;
        if data.len() != expected {
            return Err(BufferError::LengthMismatch {
                expected,
                actual: data.len(),
            });
        }
        Ok(ImageBuffer {
            width,
            height,
            color_space,
            alpha_mode,
            data,
        })
    }

    /// Transparent black (document 21: `(0,0,0,0)`), in the given encoding.
    pub fn transparent(
        width: usize,
        height: usize,
        color_space: ColorSpace,
        alpha_mode: AlphaMode,
    ) -> Self {
        ImageBuffer {
            width,
            height,
            color_space,
            alpha_mode,
            data: vec![0.0; width * height * 4],
        }
    }

    /// Decode 8-bit RGBA as document 21 specifies for G1 PNG input: sRGB encoded, straight alpha.
    pub fn from_srgb8_straight(
        width: usize,
        height: usize,
        bytes: &[u8],
    ) -> Result<Self, BufferError> {
        let expected = width * height * 4;
        if bytes.len() != expected {
            return Err(BufferError::LengthMismatch {
                expected,
                actual: bytes.len(),
            });
        }
        let data = bytes.iter().map(|&v| color::dequantise_u8(v)).collect();
        ImageBuffer::new(width, height, ColorSpace::Srgb, AlphaMode::Straight, data)
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn color_space(&self) -> ColorSpace {
        self.color_space
    }
    pub fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
    pub fn data(&self) -> &[f32] {
        &self.data
    }

    pub fn pixel(&self, x: usize, y: usize) -> [f32; 4] {
        let i = (y * self.width + x) * 4;
        [
            self.data[i],
            self.data[i + 1],
            self.data[i + 2],
            self.data[i + 3],
        ]
    }

    /// Convert into the document 21 working space: linear light, premultiplied.
    ///
    /// Each conversion runs at most once, and only if the tag says it is needed. A buffer
    /// already tagged `LinearLight` is not transformed again; this is what makes a duplicate
    /// display transform (T-09) impossible rather than merely discouraged.
    pub fn into_working(self) -> WorkingBuffer {
        let ImageBuffer {
            width,
            height,
            color_space,
            alpha_mode,
            mut data,
        } = self;
        for px in data.chunks_exact_mut(4) {
            if color_space == ColorSpace::Srgb {
                for c in &mut px[..3] {
                    *c = color::srgb_to_linear(*c);
                }
            }
            if alpha_mode == AlphaMode::Straight {
                let a = px[3];
                for c in &mut px[..3] {
                    *c *= a;
                }
            }
        }
        WorkingBuffer(ImageBuffer {
            width,
            height,
            color_space: ColorSpace::LinearLight,
            alpha_mode: AlphaMode::Premultiplied,
            data,
        })
    }
}

/// A buffer proven to be in the document 21 working space: linear light, premultiplied RGB.
///
/// The only constructor is [`ImageBuffer::into_working`].
#[derive(Clone, Debug)]
pub struct WorkingBuffer(ImageBuffer);

impl WorkingBuffer {
    pub fn transparent(width: usize, height: usize) -> Self {
        WorkingBuffer(ImageBuffer::transparent(
            width,
            height,
            ColorSpace::LinearLight,
            AlphaMode::Premultiplied,
        ))
    }

    pub fn as_image(&self) -> &ImageBuffer {
        &self.0
    }
    pub fn into_image(self) -> ImageBuffer {
        self.0
    }
    pub fn width(&self) -> usize {
        self.0.width
    }
    pub fn height(&self) -> usize {
        self.0.height
    }
    pub fn data(&self) -> &[f32] {
        &self.0.data
    }
    pub fn data_mut(&mut self) -> &mut [f32] {
        &mut self.0.data
    }
    pub fn pixel(&self, x: usize, y: usize) -> [f32; 4] {
        self.0.pixel(x, y)
    }

    /// Encode for output: unpremultiply in linear light, apply the output transfer function,
    /// then quantise. Document 21 line 31: output "converts the linear working RGB to the
    /// declared output encoding, then writes straight alpha".
    pub fn to_srgb8_straight(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.0.data.len());
        for px in self.0.data.chunks_exact(4) {
            let straight = composite::unpremultiply([px[0], px[1], px[2], px[3]]);
            for &c in &straight[..3] {
                out.push(color::quantise_u8(color::linear_to_srgb(c)));
            }
            out.push(color::quantise_u8(straight[3]));
        }
        out
    }
}
