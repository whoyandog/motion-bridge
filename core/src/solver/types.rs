#[derive(Debug, Clone)]
pub enum SolverEvent {
    Handshake {
        model_name: String,
    },
    BodyFrame {
        timestamp: u64,
        actor_id: u32,
        landmarks_data: Vec<f32>,
    },
}
