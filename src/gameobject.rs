use super::transform::Transform;
use rand::Rng;

pub struct GameObject<'a> {
    m_id: i128,
    m_type: &'a str,
    m_transform: Transform,
}

impl GameObject<'_> {
    fn new(m_type: &str, m_transform: Transform) -> GameObject {
        let mut rng = rand::thread_rng();
        let mut go = GameObject {
            m_id: rng.gen(),
            m_type,
            m_transform,
        };
        go.create();
        go
    }
    pub fn create(&mut self) {}
    pub fn update(&mut self) {}
}
