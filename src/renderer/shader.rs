use std::{path::PathBuf, ffi::CString};
use gl::types::{GLint, GLchar};

use crate::prelude::ComCtx;

/// An enum to represent the sahder stage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderStage {
    Undefined,
    Vertex,
    Fragment,
}

/// A simple wrapper around an opengl shader
#[derive(Debug)]
pub struct Shader {
    // The shader id
    id: u32
}

impl Shader {
    /// Load a shader from string
    pub fn from_string(_ctx: &ComCtx, source: impl Into<String>) -> anyhow::Result<Self> {
        // Safety: Becomes safe since passing a ctx ensures the opengl functions are loaded 
        unsafe { Self::from_string_unchecked(source) }
    }

    /// Load a shader from a file
    pub fn from_file(_ctx: &ComCtx, path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        // Safety: Becomes safe since passing a ctx ensures the opengl functions are loaded 
        unsafe { Self::from_file_unchecked(path) }
    }

    /// Load a shader from a string without checking for an opengl contexxt
    /// # Safety
    /// Unsafe since there is no assurance the opengl functions are loaded and memory is being manipulated manually
    pub unsafe fn from_string_unchecked(source: impl Into<String>) -> anyhow::Result<Self> {
        // Get the shader source
        let (vertex_source, fragment_source) = parse(source)?;
        
        // Generate the shaders
        let vertex = gl::CreateShader(gl::VERTEX_SHADER);
        let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);

        // Create the cstrings
        let vertex_source = CString::new(vertex_source.as_bytes())?;
        let fragment_source = CString::new(fragment_source.as_bytes())?;

        // Assign the sources
        gl::ShaderSource(vertex, 1, &vertex_source.as_ptr(), std::ptr::null());
        gl::ShaderSource(fragment, 1, &fragment_source.as_ptr(), std::ptr::null());

        // Compile the shaders
        gl::CompileShader(vertex);
        gl::CompileShader(fragment);

        // Check for errors
        check_for_errors(vertex, fragment)?;

        // Create the shader program
        let id = gl::CreateProgram();
        gl::AttachShader(id, vertex);
        gl::AttachShader(id, fragment);
        gl::LinkProgram(id);

        // Delete the shaders
        gl::DeleteShader(vertex);
        gl::DeleteShader(fragment);

        Ok(Self { id })
    }


    /// Load a shader form a file without checking for opengl context
    /// # Safety
    /// Unsafe since there is no assurance the opengl functions are loaded and memory is being manipulated manually
    pub unsafe fn from_file_unchecked(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        // Cast the path
        let path: PathBuf = path.into();
        // Try to read the source
        let source = std::fs::read_to_string(path)?;
        // Create the shader form the source and return it
        Self::from_string_unchecked(source)        
    }

    /// Bind the shader without checking for opengl context
    pub unsafe fn bind_unchecked(&self) {
        gl::UseProgram(self.id);
    }

    /// Get the program id
    pub fn program(&self) -> u32  {
        self.id
    }
}

/// Parses a shader string to seperate the vertex and fragment sources
pub fn parse(source: impl Into<String>) -> anyhow::Result<(String, String)> {
    // Cast the source
    let source: String = source.into();
    // Parse the source
    let mut vertex_source = String::from(""); 
    let mut fragment_source = String::from("");

    // Loop over the lines and parse the source
    let mut stage = ShaderStage::Undefined;
    for line in source.split("\n") {
        // Skip if the line is empty
        if line.is_empty() {
            continue;
        }
        // Determine if the shader stage should be changed
        if line.contains("#stage vertex") {
            stage = ShaderStage::Vertex;
            continue;
        }
        if line.contains("#stage fragment") {
            stage = ShaderStage::Fragment;
            continue;
        }
        // Add the lines to the sources
        match stage {
            ShaderStage::Undefined => return Err(anyhow::anyhow!("No shader stage defined. Please add \"#stage {{stage}}\" to the top of your file.")),
            ShaderStage::Vertex => vertex_source = format!("{}\n{}", vertex_source, line),
            ShaderStage::Fragment => fragment_source = format!("{}\n{}", fragment_source, line),
        };
    }

    Ok((vertex_source.into(), fragment_source.into()))
}

/// Check for shader errors
pub unsafe fn check_for_errors(vertex_shader: u32, fragment_shader: u32) -> anyhow::Result<()> {
    // Check for errors with a specific stage
    unsafe fn check(shader: u32) -> anyhow::Result<()> {
        let mut success = gl::FALSE as GLint;
        let mut infolog = Vec::with_capacity(512);
        infolog.set_len(512 - 1); // Keep trailing null character

        // Check for success
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        // Handle failure
        if success != gl::TRUE as GLint {
            // Get the log
            gl::GetShaderInfoLog(shader, 512, std::ptr::null_mut(), infolog.as_mut_ptr() as *mut GLchar);
            // Return the error
            return Err(anyhow::anyhow!("{}", String::from_utf8_unchecked(infolog)))
        }
        Ok(())
    }

    // Check for each error
    check(vertex_shader)?;
    check(fragment_shader)?;
    Ok(())
}