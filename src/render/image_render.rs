use glium::{
    backend::glutin::Display,
    texture::{texture2d::Texture2d, RawImage2d},
    uniforms::{Sampler, MagnifySamplerFilter},
};
use std::{
    path::Path,
    collections::HashMap,
    error::Error,
};

#[derive(Copy, Clone, Debug)]
struct InvalidPathName;

impl std::fmt::Display for InvalidPathName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unable to convert path into string")
    }
}

impl Error for InvalidPathName {}

pub enum SamplerType {
    Linear,
    Nearest
}

pub struct Textures {
    images: HashMap<String, Texture2d>,
}

impl Textures {
    pub fn new() -> Textures {
        Textures {
            images: HashMap::new()
        }
    }

    pub fn get_sampler<'a>(&'a self, name: &str, sample: SamplerType) -> Option<Sampler<'a, Texture2d>> {
        self.images.get(name).map(|texture| {
            let mut sampler = Sampler::new(texture);
            match sample {
                SamplerType::Nearest => sampler = sampler.magnify_filter(MagnifySamplerFilter::Nearest),
                SamplerType::Linear => sampler = sampler.magnify_filter(MagnifySamplerFilter::Linear),
            }

            sampler
        })
    }

    pub fn load_new(&mut self, display: &Display, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let to_load = image::open(&path)?;
        let name = path.as_ref().to_str().ok_or(InvalidPathName)?;

        let rbga = to_load.to_rgba();
        let dims = rbga.dimensions();
        let raw_image = RawImage2d::from_raw_rgba(rbga.into_raw(), dims);
        let texture = Texture2d::new(display, raw_image)?;

        self.images.insert(name.to_string(), texture);

        Ok(())
    }
}