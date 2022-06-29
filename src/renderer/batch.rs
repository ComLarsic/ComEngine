use std::os::raw::c_void;

use gl::types::{GLfloat, GLint, GLuint, GLsizeiptr, GLsizei};

/// Represents a vertex
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

/// Reperesents a batch of vertices
#[derive(Debug, Clone)]
pub struct Batch {
    // The vertices in a batch
    pub vertices: Vec<Vertex>,
    // The indices in a batch
    pub indices: Vec<u32>,
}

/// Manages the batches for the renderer
#[derive(Debug)]
pub struct BatchManager {
    // The max amount of vertices in a batch
    vertex_per_batch: u32,
    // The batches to manage
    batches: Vec<Batch>,
}

impl BatchManager {
    /// Construct a new [`BatchManager`]
    pub fn new(vertex_per_batch: u32) -> Self {
        Self {
            vertex_per_batch,
            batches: Vec::new(),
        }
    }

    /// Add a polygon to the batch
    pub fn add_polygon(&mut self, vertices: [Vertex; 3]) {
        // Add a batch if none exists
        if self.batches.len() < 1 {
            self.batches.push(Batch {
                vertices: vec![vertices[0], vertices[1], vertices[2]],
                indices: vec![0, 1, 2],
            });
            return;
        }
        // Add a new batch if the batch exceeds the max amount
        let batch = self.batches.last().unwrap();
        if batch.vertices.len() >= self.vertex_per_batch as usize {
            self.batches.push(Batch {
                vertices: vec![vertices[0], vertices[1], vertices[2]],
                indices: vec![0, 1, 2],
            });
            return;
        }
        // Add vertices to the batch
        let batch = self.batches.last_mut().unwrap();
        batch.vertices.append(&mut vertices.to_vec());
        // Add the indices to the batch
        let start = batch.indices.len() as u32;
        batch
            .indices
            .append(&mut vec![start + 0, start + 1, start + 2])
    }

    /// Adds a quad to the batch
    pub fn add_quad(&mut self, vertices: [Vertex; 4]) {
        // Add a batch if none exists
        if self.batches.len() < 1 {
            self.batches.push(Batch {
                vertices: vec![vertices[0], vertices[1], vertices[2], vertices[3]],
                indices: vec![0, 1, 3, 1, 2, 3],
            });
            return;
        }
        // Add a new batch if the batch exceeds the max amount
        let batch = self.batches.last().unwrap();
        if batch.vertices.len() >= self.vertex_per_batch as usize {
            self.batches.push(Batch {
                vertices: vec![vertices[0], vertices[1], vertices[2], vertices[3]],
                indices: vec![0, 1, 3, 1, 2, 3],
            });
            return;
        }
        // Add vertices to the batch
        let batch = self.batches.last_mut().unwrap();
        batch.vertices.append(&mut vertices.to_vec());
        // Add the indices to the batch
        let start = batch.indices.len() as u32;
        batch.indices.append(&mut vec![
            start + 0,
            start + 1,
            start + 3,
            start + 1,
            start + 2,
            start + 3,
        ])
    }

    /// Draws the batches using opengl
    /// # Safety
    /// Unsafe since there is no garuntee the opengl functions are loaded and memory is being manipulated directly
    pub unsafe fn draw(&mut self) {
        // Draw each batch
        for batch in self.batches.iter() {
            // Generate the buffers
            let mut vao: u32 = 0;
            let mut vbo: u32 = 0;
            let mut ebo: u32 = 0;

            // Generate the vao
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Generate vbo
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (batch.vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
                &batch.vertices[0].position[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );
            // Set the vertex attributes
            // -- Position
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            // -- Color
            gl::VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                (3 * std::mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            // Generate the ebo
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (batch.indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                &batch.indices[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW
            );

            // Draw the indexed vertices
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::DrawElements(gl::TRIANGLES, batch.indices.len() as GLint, gl::UNSIGNED_INT, std::ptr::null());
        }

        // Clear the batches
        self.batches.clear();
    }
}