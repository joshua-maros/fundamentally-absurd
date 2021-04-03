use std::fs::File;

use gif::{Encoder, Frame, Repeat};

use crate::{options::WORLD_SIZE, renderer::Renderer};

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

const CLIP_SIZE: i32 = 20;

impl Snapshot {
    fn of(world: &Renderer) -> Self {
        world.with_cpu_world_buffer(|world| Self {
            data: Vec::from(world),
        })
    }

    fn pixel(&self, x: i32, y: i32) -> u16 {
        let x = (x + WORLD_SIZE as i32) as u32 % WORLD_SIZE;
        let y = (y + WORLD_SIZE as i32) as u32 % WORLD_SIZE;
        let index = y * WORLD_SIZE + x;
        self.data[index as usize]
    }

    fn clip(&self, cx: i32, cy: i32) -> Vec<u16> {
        let mut result = Vec::new();
        for y in (cy - CLIP_SIZE)..(cy + CLIP_SIZE + 1) {
            for x in (cx - CLIP_SIZE)..(cx + CLIP_SIZE + 1) {
                result.push(self.pixel(x, y));
            }
        }
        result
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

    pub fn find_pattern_densities(&self) -> Vec<f32> {
        let max_period = self.snapshots.len() / 3;
        let mut counts = vec![0; max_period + 1];
        let positions = self.snapshots[0].data.len() / 6;
        for pos in 0..positions {
            let pos = pos * 6;
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
            let pos = pos * 6;
            if self.snapshots[0].data[pos] != 0 {
                occupied_cells += 1;
            }
        }
        let density = occupied_cells as f32 / positions as f32;
        let densities = counts
            .into_iter()
            .map(|count| count as f32 / positions as f32 / density)
            .collect();
        densities
    }

    pub fn compute_score(&self, densities: &[f32]) -> f32 {
        let mut score = 0.0;
        for period in 2..densities.len() {
            score += (period * period) as f32 * densities[period];
        }
        let oscillator_density: f32 = densities.iter().skip(2).sum();
        if oscillator_density < 1.0 {
            score -= densities[0] * 20.0;
        }
        score += 1e3;
        if densities[1] > oscillator_density * 5.0 {
            score *= 0.001;
        }
        score
    }

    fn hue_part(hue: f32) -> f32 {
        let hue = hue * 3.0;
        let hue = (hue + 3.0) % 3.0;
        if hue < 1.0 {
            hue
        } else if hue > 2.0 {
            3.0 - hue
        } else {
            1.0
        }
    }

    pub fn create_gif(&self, densities: &[f32], filename: &str) {
        let mut period_pool: Vec<_> = (0..densities.len())
            .filter(|period| densities[*period] > 0.01)
            .collect();
        period_pool.sort_unstable_by(|a, b| densities[*a].partial_cmp(&densities[*b]).unwrap());
        let ppl = period_pool.len();
        let other_copies = 16 / ppl;
        let highlighted_copies = 16 % ppl;
        let mut periods = Vec::new();
        for idx in 0..ppl {
            let num = if idx < highlighted_copies {
                other_copies + 1
            } else {
                other_copies
            };
            for _ in 0..num {
                periods.push(period_pool[idx]);
            }
        }
        println!("{:?}", periods);
        let mut positions = Vec::new();
        while positions.len() < periods.len() {
            let period = periods[positions.len()];
            let x: usize = rand::random::<usize>() % WORLD_SIZE as usize;
            let y: usize = rand::random::<usize>() % WORLD_SIZE as usize;
            let position = (y * WORLD_SIZE as usize) + x;
            if period == 1 && self.snapshots[0].data[position] == 0 {
                continue;
            } else if period > 1 {
                let mut ok = true;
                for previous in 1..period {
                    if self.check_for_pattern(previous, position) {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    continue;
                }
            }
            if self.check_for_pattern(period, position) {
                positions.push((x as i32, y as i32));
            }
        }
        let mut frames = Vec::new();
        for snapshot in &self.snapshots {
            let mut frame = DisplayFrame::new();
            for (index, &(x, y)) in positions.iter().enumerate() {
                let clip = snapshot.clip(x, y);
                let fx = (index % 4) * (CLIP_SIZE as usize * 2 + 2) + 1;
                let fy = (index / 4) * (CLIP_SIZE as usize * 2 + 2) + 1;
                frame.put_clip(fx, fy, &clip[..]);
            }
            frames.push(frame.into_gif_frame());
        }
        let mut palette = Vec::new();
        palette.append(&mut vec![0x0, 0x0, 0x0]);
        palette.append(&mut vec![0xFF, 0xFF, 0xFF]);
        palette.append(&mut vec![0xFF, 0xFF, 0x0]);
        let mut hue = 0.5;
        let mut step = 1.0;
        while palette.len() < 256 * 3 {
            let r = Self::hue_part(hue + 0.66);
            let g = Self::hue_part(hue + 0.33);
            let b = Self::hue_part(hue);
            palette.push((r * 255.9) as u8);
            palette.push((g * 255.9) as u8);
            palette.push((b * 255.9) as u8);
            hue += step;
            if hue >= 0.9999 {
                step /= 2.0;
                hue = step / 2.0;
            }
        }
        let mut file = File::create(filename).unwrap();
        let mut encoder =
            Encoder::new(&mut file, frames[0].width, frames[0].width, &palette[..]).unwrap();
        encoder.set_repeat(Repeat::Infinite).unwrap();
        for frame in &frames {
            encoder.write_frame(frame).unwrap();
        }
    }
}

struct DisplayFrame {
    width: usize,
    data: Vec<u8>,
}

impl DisplayFrame {
    fn new() -> Self {
        let width = (CLIP_SIZE as usize * 2 + 1) * 4 + 5;
        let mut res = Self {
            width,
            data: vec![0; width * width],
        };
        for x in 0..width {
            for y in 0..5 {
                res.set_pixel(x, y * (CLIP_SIZE as usize * 2 + 2), 1);
            }
        }
        for x in 0..5 {
            for y in 0..width {
                res.set_pixel(x * (CLIP_SIZE as usize * 2 + 2), y, 1);
            }
        }
        res
    }

    fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        self.data[y * self.width + x] = value;
    }

    fn put_clip(&mut self, x: usize, y: usize, snapshot: &[u16]) {
        let clip_dim = CLIP_SIZE as usize * 2 + 1;
        for dx in 0..clip_dim {
            for dy in 0..clip_dim {
                self.set_pixel(x + dx, y + dy, snapshot[dy * clip_dim + dx] as _);
            }
        }
    }

    fn into_gif_frame(self) -> Frame<'static> {
        let mut res =
            Frame::from_indexed_pixels(self.width as _, self.width as _, &self.data[..], None);
        res.delay = 33;
        res
    }
}
