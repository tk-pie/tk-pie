use embassy_executor::{SendSpawner, Spawner};

#[derive(Copy, Clone)]
pub struct Spawners {
    /// High priority executor. Tasks spawned on this will preempt medium and low priority tasks.
    pub high: SendSpawner,

    /// Medium priority executor. Tasks spawned on this will preempt low priority tasks.
    pub med: SendSpawner,

    /// Lom priority executor. Tasks spawned on this will be preempted by higher priority tasks.
    pub low: Spawner,
}
