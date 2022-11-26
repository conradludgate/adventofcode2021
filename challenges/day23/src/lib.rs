use core::fmt;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum State {
    A,
    B,
    C,
    D,
    Empty,
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::A => write!(f, "A"),
            State::B => write!(f, "B"),
            State::C => write!(f, "C"),
            State::D => write!(f, "D"),
            State::Empty => write!(f, "."),
        }
    }
}

impl State {
    fn cost(&self) -> usize {
        match self {
            State::A => 1,
            State::B => 10,
            State::C => 100,
            State::D => 1000,
            State::Empty => 0,
        }
    }
    fn room(&self) -> usize {
        match self {
            State::A => 0,
            State::B => 1,
            State::C => 2,
            State::D => 3,
            State::Empty => 4,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
// first 8 slots are divided into the 4 rooms
// the first of each pair being the closest slot to the corridor
// the last 7 slots are the corridor
pub struct Day23 {
    rooms: [[State; 2]; 4],
    corridor: [State; 11],
}

impl fmt::Display for Day23 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for state in self.corridor {
            write!(f, "{state}")?;
        }
        write!(f, "#\n###")?;
        for room in self.rooms {
            write!(f, "{}#", room[0])?;
        }
        write!(f, "##\n  #")?;
        for room in self.rooms {
            write!(f, "{}#", room[1])?;
        }
        writeln!(f, "  \n  #########")
    }
}

impl<'i> ChallengeParser<'i> for Day23 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        let mut rooms = [[State::Empty; 2]; 4];

        let line_length = 14;
        for (i, room) in rooms.iter_mut().enumerate() {
            for (j, pos) in room.iter_mut().enumerate() {
                let x = line_length * (2 + j) + 3 + i * 2;
                *pos = match input.as_bytes()[x] {
                    b'A' => State::A,
                    b'B' => State::B,
                    b'C' => State::C,
                    b'D' => State::D,
                    _ => {
                        return Err(nom::Err::Error(nom::error::Error::<&str>::new(
                            &input[x..],
                            nom::error::ErrorKind::OneOf,
                        )))
                    }
                };
            }
        }
        Ok((
            "",
            Self {
                rooms,
                corridor: [State::Empty; 11],
            },
        ))
    }
}

impl Day23 {
    const SUCCESS: Self = Self {
        rooms: [[State::A; 2], [State::B; 2], [State::C; 2], [State::D; 2]],
        corridor: [State::Empty; 11],
    };

    fn successors(&self) -> Vec<(Self, usize)> {
        // println!("{self:?}");
        let mut output = vec![];

        // first, try move pieces in the cooridor
        for (i, state) in self.corridor.into_iter().enumerate() {
            if state == State::Empty {
                continue;
            }

            // // tackle the extreme ends of the corridor
            // if (i == 0 && self.corridor[1] == State::Empty) || (i == 1 && self.corridor[0] == State::Empty) {
            //     let mut pos = *self;
            //     pos.corridor.swap(0, 1);
            //     output.push((pos, state.cost()))
            // }
            // if (i == 10 && self.corridor[9] == State::Empty) || (i == 9 && self.corridor[10] == State::Empty) {
            //     let mut pos = *self;
            //     pos.corridor.swap(9, 10);
            //     output.push((pos, state.cost()))
            // }

            // // handle other corridor to corridor moves
            // if i > 1 && self.corridor[i - 2] == State::Empty {
            //     let mut pos = *self;
            //     pos.corridor.swap(i, i - 2);
            //     output.push((pos, state.cost() * 2))
            // }
            // if i < 9 && self.corridor[i + 2] == State::Empty {
            //     let mut pos = *self;
            //     pos.corridor.swap(i, i + 2);
            //     output.push((pos, state.cost() * 2))
            // }

            // try from corridor to room
            let x = state.room();
            let offset = x * 2 + 2;
            let rooms = self.rooms[x];

            // if our room is empty/contains our friend
            let room_pos = if rooms == [State::Empty; 2] {
                1
            } else if rooms == [State::Empty, state] {
                0
            } else {
                continue;
            };

            // if there's nothing in the way between the room and our position
            let range = if offset < i { offset..i } else { i + 1..offset };
            if self.corridor[range].iter().all(|&x| x == State::Empty) {
                let mut pos = *self;
                pos.rooms[x][room_pos] = state;
                pos.corridor[i] = State::Empty;
                output.push((pos, state.cost() * (room_pos + 1 + i.abs_diff(offset))));
            }
        }

        // try move pieces in the rooms into the corridor
        for (i, room) in self.rooms.into_iter().enumerate() {
            if room[1] == State::Empty {
                continue;
            }
            let offset = i * 2 + 2;

            let (room_pos, state) = match room {
                [State::Empty, State::Empty] => continue,
                [State::Empty, state] if state.room() == i => continue,
                [state1, state2] if state1.room() == i && state2.room() == i => continue,
                [State::Empty, state] => (1, state),
                [state, _] => (0, state),
            };

            // corridor positions
            for x in 0..=10 {
                // ignore directly above a room
                if [2, 4, 6, 8].contains(&x) {
                    continue;
                }
                // ignore occupied spaces
                if self.corridor[x] != State::Empty {
                    continue;
                }
                // if there's nothing in the way between the room and our position
                let range = if offset < x { offset..x } else { x + 1..offset };
                if self.corridor[range].iter().all(|&x| x == State::Empty) {
                    let mut pos = *self;
                    pos.rooms[i][room_pos] = State::Empty;
                    pos.corridor[x] = state;
                    output.push((pos, state.cost() * (room_pos + 1 + x.abs_diff(offset))));
                }
            }

            // if room[0] == State::Empty {
            //     if self.corridor[offset - 1] == State::Empty {
            //         let mut pos = *self;
            //         pos.rooms[i][1] = State::Empty;
            //         pos.corridor[offset - 1] = room[1];
            //         output.push((pos, room[1].cost() * 3));
            //     }
            //     if self.corridor[offset + 1] == State::Empty {
            //         let mut pos = *self;
            //         pos.rooms[i][1] = State::Empty;
            //         pos.corridor[offset + 1] = room[1];
            //         output.push((pos, room[1].cost() * 3));
            //     }
            // } else {
            //     if self.corridor[offset - 1] == State::Empty {
            //         let mut pos = *self;
            //         pos.rooms[i][0] = State::Empty;
            //         pos.corridor[offset - 1] = room[0];
            //         output.push((pos, room[1].cost() * 2));
            //     }
            //     if self.corridor[offset + 1] == State::Empty {
            //         let mut pos = *self;
            //         pos.rooms[i][0] = State::Empty;
            //         pos.corridor[offset + 1] = room[0];
            //         output.push((pos, room[1].cost() * 2));
            //     }
            // }
        }

        output
    }

    fn heuristic(&self) -> usize {
        let mut distance = 0;
        for (i, room) in self.rooms.into_iter().enumerate() {
            // both correct
            if room[0] == room[1] && room[0].room() == i {
                continue;
            }

            // if outer is occupied
            if room[0].room() < 4 {
                let dist = room[0].room();
                distance += (dist * 2 + 2) * room[0].cost();
            }

            // if inner is correct
            if room[1].room() == i {
                continue;
            }

            // if inner is occupied
            if room[1].room() < 4 {
                let dist = room[1].room();
                distance += (dist * 2 + 3) * room[1].cost();
            }
        }
        for (i, state) in self.corridor.into_iter().enumerate() {
            let x = state.room();
            let offset = x * 2 + 2;
            distance += state.cost() * (offset.abs_diff(i) + 1)
        }
        distance
    }
}

impl Challenge for Day23 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        let (path, cost) =
            pathfinding::directed::astar::astar(&self, Self::successors, Self::heuristic, |x| x == &Self::SUCCESS)
                .unwrap();
        for i in path {
            println!("{i}");
        }
        cost
    }

    fn part_two(self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day23;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

    #[test]
    fn parse() {
        let output = Day23::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn heuristic() {
        let output = Day23::parse(INPUT).unwrap().1;
        assert_eq!(Day23::SUCCESS.heuristic(), 0);
        assert_eq!(output.heuristic(), 17683);
    }

    #[test]
    fn part_one() {
        let output = Day23::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 12521);
    }

    #[test]
    fn part_two() {
        let output = Day23::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
