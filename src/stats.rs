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

struct Snapshot {
    data: Vec<u16>,
}

impl Snapshot {
    fn of(world: &Renderer) -> Self {
        world.with_cpu_world_buffer(|world| {
            let mut data = Vec::new();
            for index in 0..world.len() / 6 {
                data.push(world[index * 6]);
            }
            Self { data }
        })
    }
}

pub struct Scorer {
    snapshots: Vec<Snapshot>,
}

impl Scorer {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
        }
    }

    pub fn add_snapshot(&mut self, world: &Renderer) {
        self.snapshots.push(Snapshot::of(world));
    }

    fn check_for_pattern(&self, period: usize, position: usize) -> bool {
        for index in 0..(self.snapshots.len() - period) {
            if self.snapshots[index].data[position] != self.snapshots[index + period].data[position]
            {
                return false;
            }
        }
        true
    }

    pub fn find_pattern_frequencies(&self) -> Vec<f32> {
        let max_period = self.snapshots.len() / 3;
        let mut counts = vec![0; max_period + 1];
        let positions = self.snapshots[0].data.len();
        for pos in 0..positions {
            let mut chaotic = true;
            for period in 1..max_period + 1 {
                if self.check_for_pattern(period, pos) {
                    chaotic = false;
                    if period != 1 || self.snapshots[0].data[pos] != 0 {
                        counts[period] += 1;
                    }
                    break;
                }
            }
            if chaotic {
                counts[0] += 1;
            }
        }
        let mut occupied_cells = 0;
        for pos in 0..positions {
            if self.snapshots[0].data[pos] != 0 {
                occupied_cells += 1;
            }
        }
        let density = occupied_cells as f32 / positions as f32;
        counts
            .into_iter()
            .map(|count| count as f32 / positions as f32 / density)
            .collect()
    }
}
