use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Voxel {
    x: i64,
    y: i64,
    z: i64,
}

impl Voxel {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Voxel { x, y, z }
    }

    fn get_neighbors(&self) -> Vec<Voxel> {
        vec![
            Self::new(self.x + 1, self.y, self.z),
            Self::new(self.x - 1, self.y, self.z),
            Self::new(self.x, self.y + 1, self.z),
            Self::new(self.x, self.y - 1, self.z),
            Self::new(self.x, self.y, self.z + 1),
            Self::new(self.x, self.y, self.z - 1),
        ]
    }
}

fn main() {
    println!("Day 18:");
    let input = include_str!("../inputs/input.txt");

    let mut lava = HashSet::new();
    let (mut min_x, mut max_x) = (i64::MAX, i64::MIN);
    let (mut min_y, mut max_y) = (i64::MAX, i64::MIN);
    let (mut min_z, mut max_z) = (i64::MAX, i64::MIN);

    for line in input.lines() {
        let vec: Vec<i64> = line.split(',').map(|n| n.parse().unwrap()).collect();
        let voxel = Voxel::new(vec[0], vec[1], vec[2]);

        min_x = min_x.min(voxel.x);
        max_x = max_x.max(voxel.x);
        min_y = min_y.min(voxel.y);
        max_y = max_y.max(voxel.y);
        min_z = min_z.min(voxel.z);
        max_z = max_z.max(voxel.z);

        lava.insert(voxel);
    }

    let mut answer_1 = 0;
    let mut answer_2 = 0;

    let min = Voxel::new(min_x, min_y, min_z);
    let max = Voxel::new(max_x, max_y, max_z);

    let mut exterior = HashSet::new();

    get_exterior(
        Voxel::new(min_x - 1, min_y - 1, min_z - 1),
        &mut exterior,
        &lava,
        &min,
        &max,
    );

    for voxel in lava.clone() {
        for neighbor in voxel.get_neighbors() {
            if !lava.contains(&neighbor) {
                answer_1 += 1
            };
            if exterior.contains(&neighbor) {
                answer_2 += 1
            };
        }
    }

    println!("\t1) {answer_1}");
    println!("\t2) {answer_2}");
}

fn get_exterior(
    voxel: Voxel,
    exterior: &mut HashSet<Voxel>,
    lava: &HashSet<Voxel>,
    min: &Voxel,
    max: &Voxel,
) {
    if voxel.x >= min.x - 1
        && voxel.x <= max.x + 1
        && voxel.y >= min.y - 1
        && voxel.y <= max.y + 1
        && voxel.z >= min.z - 1
        && voxel.z <= max.z + 1
        && !lava.contains(&voxel) & !exterior.contains(&voxel)
    {
        exterior.insert(voxel.clone());
        for neighbor in voxel.get_neighbors() {
            get_exterior(neighbor, exterior, lava, min, max)
        }
    }
}
