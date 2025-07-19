pub struct Cube;

impl Cube {
    pub fn new(width: f32, height: f32, length: f32) -> (Vec<f32>, Vec<u32>) {
        let w = width / 2.0;
        let h = height / 2.0;
        let l = length / 2.0;

        let vertices: Vec<f32> = vec![
            // Front face
            -w, -h, l, 0.0, 0.0, 1.0, 1.0, 1.0, w, -h, l, 1.0, 0.0, 1.0, 1.0, 1.0, w, h, l, 1.0,
            1.0, 1.0, 1.0, 1.0, -w, h, l, 0.0, 1.0, 1.0, 1.0, 1.0, // Back face
            -w, -h, -l, 1.0, 0.0, 1.0, 1.0, 1.0, w, -h, -l, 0.0, 0.0, 1.0, 1.0, 1.0, w, h, -l, 0.0,
            1.0, 1.0, 1.0, 1.0, -w, h, -l, 1.0, 1.0, 1.0, 1.0, 1.0,
        ];

        let indices: Vec<u32> = vec![
            // Front
            0, 1, 2, 2, 3, 0, // Back
            4, 5, 6, 6, 7, 4, // Left
            4, 0, 3, 3, 7, 4, // Right
            1, 5, 6, 6, 2, 1, // Top
            3, 2, 6, 6, 7, 3, // Bottom
            4, 5, 1, 1, 0, 4,
        ];

        (vertices, indices)
    }
}
