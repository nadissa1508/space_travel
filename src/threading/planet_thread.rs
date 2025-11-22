use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use crate::math::Vec3;

/// Mensaje para comunicar estado de un planeta
#[derive(Debug, Clone)]
pub struct PlanetState {
    pub index: usize,
    pub position: Vec3,
    pub rotation_angle: f32,
    pub orbit_angle: f32,
}

/// Comando que puede recibir un hilo de planeta
#[derive(Debug, Clone)]
pub enum PlanetCommand {
    Update(f32),  // delta_time
    Stop,
}

/// Worker que actualiza un planeta en su propio hilo
pub struct PlanetWorker {
    pub index: usize,
    pub handle: Option<JoinHandle<()>>,
    pub command_sender: mpsc::Sender<PlanetCommand>,
}

impl PlanetWorker {
    /// Crea un nuevo worker para un planeta
    pub fn new(
        index: usize,
        orbit_radius: f32,
        orbit_speed: f32,
        rotation_speed: f32,
        initial_orbit_angle: f32,
        state_sender: mpsc::Sender<PlanetState>,
    ) -> Self {
        let (command_sender, command_receiver) = mpsc::channel::<PlanetCommand>();

        let handle = thread::spawn(move || {
            let mut orbit_angle = initial_orbit_angle;
            let mut rotation_angle = 0.0f32;

            loop {
                match command_receiver.recv() {
                    Ok(PlanetCommand::Update(delta_time)) => {
                        // Actualizar ángulos
                        orbit_angle += orbit_speed * delta_time;
                        rotation_angle += rotation_speed * delta_time;

                        // Normalizar ángulos
                        if orbit_angle > std::f32::consts::TAU {
                            orbit_angle -= std::f32::consts::TAU;
                        }
                        if rotation_angle > std::f32::consts::TAU {
                            rotation_angle -= std::f32::consts::TAU;
                        }

                        // Calcular posición
                        let position = Vec3::new(
                            orbit_radius * orbit_angle.cos(),
                            0.0,
                            orbit_radius * orbit_angle.sin(),
                        );

                        // Enviar estado actualizado
                        let state = PlanetState {
                            index,
                            position,
                            rotation_angle,
                            orbit_angle,
                        };

                        if state_sender.send(state).is_err() {
                            break; // Canal cerrado, terminar
                        }
                    }
                    Ok(PlanetCommand::Stop) | Err(_) => {
                        break;
                    }
                }
            }
        });

        Self {
            index,
            handle: Some(handle),
            command_sender,
        }
    }

    /// Envía comando de actualización al hilo
    pub fn send_update(&self, delta_time: f32) {
        let _ = self.command_sender.send(PlanetCommand::Update(delta_time));
    }

    /// Detiene el worker
    pub fn stop(&mut self) {
        let _ = self.command_sender.send(PlanetCommand::Stop);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for PlanetWorker {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Administrador de todos los workers de planetas
pub struct PlanetThreadPool {
    workers: Vec<PlanetWorker>,
    state_receiver: mpsc::Receiver<PlanetState>,
    pub latest_states: Vec<Option<PlanetState>>,
}

impl PlanetThreadPool {
    /// Crea un pool vacío
    pub fn new(planet_count: usize) -> (Self, mpsc::Sender<PlanetState>) {
        let (state_sender, state_receiver) = mpsc::channel();
        
        let pool = Self {
            workers: Vec::new(),
            state_receiver,
            latest_states: vec![None; planet_count],
        };
        
        (pool, state_sender)
    }

    /// Agrega un worker al pool
    pub fn add_worker(&mut self, worker: PlanetWorker) {
        self.workers.push(worker);
    }

    /// Envía comando de actualización a todos los workers
    pub fn update_all(&mut self, delta_time: f32) {
        for worker in &self.workers {
            worker.send_update(delta_time);
        }

        // Recolectar estados actualizados (non-blocking)
        while let Ok(state) = self.state_receiver.try_recv() {
            let index = state.index;
            if index < self.latest_states.len() {
                self.latest_states[index] = Some(state);
            }
        }
    }

    /// Obtiene el estado más reciente de un planeta
    pub fn get_state(&self, index: usize) -> Option<&PlanetState> {
        self.latest_states.get(index).and_then(|s| s.as_ref())
    }

    /// Detiene todos los workers
    pub fn stop_all(&mut self) {
        for worker in &mut self.workers {
            worker.stop();
        }
    }
}

impl Drop for PlanetThreadPool {
    fn drop(&mut self) {
        self.stop_all();
    }
}