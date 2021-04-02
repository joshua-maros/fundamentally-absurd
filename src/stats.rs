use crate::renderer::Renderer;

struct StatCruncher<'a> {
    world: &'a [u16],
}

impl<'a> StatCruncher<'a> {
    fn crunch(self) -> Stats {
        let world = self.world;
        let total_cells = world.len();
        let occupied_cells = world.iter().filter(|&&cell| cell != 0).count();
        let population_density = occupied_cells as f32 / total_cells as f32;
        Stats { population_density }
    }
}

#[derive(Debug)]
pub struct Stats {
    pub population_density: f32,
}

impl Stats {
    pub fn of(world: &Renderer) -> Self {
        world.with_cpu_world_buffer(|world| StatCruncher { world }.crunch())
    }
}

pub struct Judge {
    snapshots: Vec<Stats>,
}

impl Judge {
    pub fn new(initial: Stats) -> Self {
        Self {
            snapshots: vec![initial],
        }
    }

    pub fn push_snapshot(&mut self, snapshot: Stats) {
        self.snapshots.push(snapshot);
    }

    pub fn judgement(&self) -> AutomaticJudgement {
        use AutomaticJudgement::*;
        let ns = self.snapshots.len();
        if ns == 1 {
            return Unknown;
        }
        let d0 = self.snapshots[0].population_density;
        if self
            .snapshots
            .iter()
            .skip(1)
            .all(|s| s.population_density > d0)
        {
            return Chaotic;
        }
        let last = &self.snapshots[self.snapshots.len() - 1];
        if last.population_density < 0.00001 {
            return Dead;
        }
        Unknown
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AutomaticJudgement {
    Dead,
    Chaotic,
    Unknown,
}

impl AutomaticJudgement {
    pub fn is_interesting(self) -> bool {
        match self {
            Self::Dead | Self::Chaotic => false,
            Self::Unknown => true,
        }
    }

    pub fn is_unknown(self) -> bool {
        if let Self::Unknown = self {
            true
        } else {
            false
        }
    }
}
