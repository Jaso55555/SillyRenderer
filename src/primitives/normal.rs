#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: [f32; 3]
}

implement_vertex!(Normal, normal);