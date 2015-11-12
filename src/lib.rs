//! A binding for assimp: the Open Asset Import Library

#![crate_name = "assimp"]
#![crate_type = "rlib"]
#![doc(html_root_url = "http://www.rust-ci.org/jemcroft/rust-assimp/doc/assimp/")]

#![deny(missing_docs)]
#![deny(non_camel_case_types)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]
#![deny(unused_parens)]
#![deny(unused_results)]
#![warn(unused_imports)]

#![feature(core, collections)]
#![feature(libc)]

extern crate libc;
extern crate vecmath;
extern crate core;

pub use types::{Vector2D, Vector3D, Color3D, Color4D, Matrix3x3, Matrix4x4,
                Quaternion, Plane, Ray, AiString};
pub use scene::Scene;

pub use property::Property;
pub use property::Component;
pub use property::TransformUV;
pub use postprocess::Process;
pub use importer::Importer;

pub mod animation;
pub mod camera;
pub mod info;
pub mod light;
pub mod material;
pub mod mesh;
pub mod scene;
pub mod texture;
pub mod types;
pub mod importer;
pub mod log;

mod property;
mod postprocess;
mod util;
mod ffi;
mod fileio;

// vim: et tw=78 sw=4:
