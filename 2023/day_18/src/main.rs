#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

type Position = (i64, i64);

#[derive(Debug, Clone, Copy)]
struct Corner {
    position: Position,
    direction: Direction,
    up: bool,
}

#[derive(Debug, Clone)]
struct Puzzle {
    corners: Vec<Corner>,
}

#[derive(Debug, Clone)]
struct Block {
    height: i64,
    segments: Vec<Segment>,
}

impl Block {
    fn area(&self, inner_is_up: bool) -> i64 {
        self.height * length(&self.segments, inner_is_up)
    }
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    first: i64,
    last: i64,
    last_up: bool,
}

fn length(segments: &[Segment], inner_is_up: bool) -> i64 {
    let mut result = 0;
    let mut inner = false;
    for index in 0..segments.len() {
        let segment = segments[index];
        if inner {
            result += segment.first - segments[index - 1].last - 1;
        }
        result += segment.last - segment.first + 1;
        inner = segment.last_up == inner_is_up;
    }
    assert!(!inner);
    result
}

#[derive(Debug, Clone, Copy)]
struct Wall {
    above_corner_index: usize,
    is_corner: bool,
}

impl Puzzle {
    fn parse(input: &str) -> Puzzle {
        let mut position = (0, 0);
        let mut corners: Vec<Corner> = vec![];
        for line in input.lines() {
            let mut parts = line.split('#');
            parts.next().unwrap();
            let code = parts.next().unwrap();
            let distance = i64::from_str_radix(&code[..5], 16).unwrap();
            let direction = match code.as_bytes()[5] {
                b'0' => Direction::Right,
                b'1' => Direction::Down,
                b'2' => Direction::Left,
                b'3' => Direction::Up,
                _ => unreachable!(),
            };
            if let Some(corner) = corners.last() {
                assert!(corner.direction != direction && corner.direction.opposite() != direction);
            }
            let up = match direction {
                Direction::Up => true,
                Direction::Down => false,
                _ => corners.last().map(|corner| corner.up).unwrap_or(true), // First corner `up` may be decided by last corner.
            };
            corners.push(Corner {
                position,
                direction,
                up,
            });
            match direction {
                Direction::Up => position.0 -= distance,
                Direction::Down => position.0 += distance,
                Direction::Left => position.1 -= distance,
                Direction::Right => position.1 += distance,
            }
        }

        match corners.last().unwrap().direction {
            Direction::Up => corners.first_mut().unwrap().up = true,
            Direction::Down => corners.first_mut().unwrap().up = false,
            _ => {}
        }

        Puzzle { corners }
    }

    fn to_blocks(&self) -> (Vec<Block>, bool) {
        // Sort the corner indices by position.
        let mut corners = self.corners.iter().enumerate().collect::<Vec<_>>();
        corners.sort_by_key(|corner| corner.1.position);

        // Group the corners that have same vertical position.
        let mut corner_indices = vec![];
        let mut index = 0;
        while index < corners.len() {
            let mut group = vec![];
            group.push(corners[index].0);
            index += 1;
            while index < corners.len()
                && corners[index].1.position.0 == corners[index - 1].1.position.0
            {
                group.push(corners[index].0);
                index += 1;
            }
            corner_indices.push(group);
        }
        for corner_indices in &corner_indices {
            assert!(corner_indices.iter().all(|&index| {
                let corner = self.corners[index];
                corner.position.0 == self.corners[corner_indices[0]].position.0
            }));
        }

        let mut walls = corner_indices[0]
            .iter()
            .map(|&index| Wall {
                above_corner_index: index,
                is_corner: true,
            })
            .collect::<Vec<_>>();
        let mut blocks = vec![Block {
            height: 1,
            segments: self.create_segments(&walls),
        }];
        let mut corner_indices_index = 1;
        while corner_indices_index < corner_indices.len() {
            // From now on, we only consider the walls extending down, including top left and top right corners, and edges.
            // The extending walls are all not corners.
            let walls_extending_down = walls
                .iter()
                .copied()
                .filter_map(|wall| if self.is_wall_extending_down(wall) {
                    Some(Wall { above_corner_index: wall.above_corner_index, is_corner: false})
                } else {
                    None
                })
                .collect::<Vec<_>>();

            // These walls form a block.
            let last_height = self.corners[corner_indices[corner_indices_index - 1][0]]
                .position
                .0;
            let current_height = self.corners[corner_indices[corner_indices_index][0]]
                .position
                .0;
            let height = current_height - last_height - 1;
            if height > 0 {
                blocks.push(Block {
                    height,
                    segments: self.create_segments(&walls_extending_down),
                });
            }

            // New walls include all the new corners.
            let mut new_walls = corner_indices[corner_indices_index]
                .iter()
                .map(|&index| Wall {
                    above_corner_index: index,
                    is_corner: true,
                })
                .collect::<Vec<_>>();
            // And the old walls that are not connected to any of the new corners.
            for wall in walls_extending_down {
                if !corner_indices[corner_indices_index]
                    .iter()
                    .any(|&index| self.is_wall_connected_to_corner(wall, index))
                {
                    new_walls.push(Wall {
                        above_corner_index: wall.above_corner_index,
                        is_corner: false,
                    })
                }
            }
            // Sort walls by horizontal position.
            new_walls.sort_by_key(|wall| self.corners[wall.above_corner_index].position.1);

            blocks.push(Block {
                height: 1,
                segments: self.create_segments(&new_walls),
            });
            walls = new_walls;
            corner_indices_index += 1;
        }

        let inner_is_up = self.corners[corner_indices[0][0]].up;

        (blocks, inner_is_up)
    }

    fn is_wall_extending_down(&self, wall: Wall) -> bool {
        if !wall.is_corner {
            return true;
        }

        let from_index = (wall.above_corner_index + self.corners.len() - 1) % self.corners.len();
        let from_direction = self.corners[from_index].direction;
        let to_direction = self.corners[wall.above_corner_index].direction;
        match (from_direction, to_direction) {
            (Direction::Up, Direction::Left) => true,
            (Direction::Up, Direction::Right) => true,
            (Direction::Down, Direction::Left) => false,
            (Direction::Down, Direction::Right) => false,
            (Direction::Left, Direction::Up) => false,
            (Direction::Left, Direction::Down) => true,
            (Direction::Right, Direction::Up) => false,
            (Direction::Right, Direction::Down) => true,
            _ => unreachable!(),
        }
    }

    fn is_wall_connected_to_corner(&self, wall: Wall, corner_index: usize) -> bool {
        let from_index = (wall.above_corner_index + self.corners.len() - 1) % self.corners.len();
        let to_index = (wall.above_corner_index + 1) % self.corners.len();
        from_index == corner_index || to_index == corner_index
    }

    fn create_segments(&self, walls: &[Wall]) -> Vec<Segment> {
        let mut result = vec![];
        let mut index = 0;
        while index < walls.len() {
            if index + 1 < walls.len() {
                if let Some(segment) = self.create_segment_opt(walls[index], walls[index + 1]) {
                    result.push(segment);
                    index += 2;
                    continue;
                }
            }

            let corner = self.corners[walls[index].above_corner_index];
            result.push(Segment {
                first: corner.position.1,
                last: corner.position.1,
                last_up: corner.up,
            });
            index += 1;
        }
        result
    }

    fn create_segment_opt(&self, wall1: Wall, wall2: Wall) -> Option<Segment> {
        if !(wall1.is_corner && wall2.is_corner) {
            return None;
        }
        if (wall1.above_corner_index + 1) % self.corners.len() == wall2.above_corner_index
            || (wall2.above_corner_index + 1) % self.corners.len() == wall1.above_corner_index
        {
            let corner1 = self.corners[wall1.above_corner_index];
            let corner2 = self.corners[wall2.above_corner_index];
            let (first, last, last_up) = if corner1.position.1 < corner2.position.1 {
                (corner1.position.1, corner2.position.1, corner2.up)
            } else {
                (corner2.position.1, corner1.position.1, corner1.up)
            };
            Some(Segment {
                first,
                last,
                last_up,
            })
        } else {
            None
        }
    }
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let puzzle = Puzzle::parse(INPUT);
    let (blocks, inner_is_up) = puzzle.to_blocks();
    println!(
        "{}",
        blocks
            .iter()
            .map(|block| block.area(inner_is_up))
            .sum::<i64>()
    );
}
