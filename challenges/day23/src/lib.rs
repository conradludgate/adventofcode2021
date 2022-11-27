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
pub struct Position<const N: usize> {
    rooms: [[State; N]; 4],
    corridor: [State; 11],
}

impl<const N: usize> fmt::Display for Position<N> {
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
        for i in 1..N {
            for room in self.rooms {
                write!(f, "{}#", room[i])?;
            }
            write!(f, "\n  #")?;
        }
        writeln!(f, "########")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Day23(Position<2>);

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
            Self(Position {
                rooms,
                corridor: [State::Empty; 11],
            }),
        ))
    }
}

impl<const N: usize> Position<N> {
    const SUCCESS: Self = Self {
        rooms: [[State::A; N], [State::B; N], [State::C; N], [State::D; N]],
        corridor: [State::Empty; 11],
    };

    fn successors(&self) -> Vec<(Self, usize)> {
        let mut output = vec![];

        // first, try move pieces in the cooridor
        'outer: for (i, state) in self.corridor.into_iter().enumerate() {
            if state == State::Empty {
                continue;
            }

            // try from corridor to room
            let x = state.room();
            let offset = x * 2 + 2;
            let room = self.rooms[x];

            // if our room is empty/contains only our friends
            let mut room_state = [State::Empty; N];
            let mut room_pos = N - 1;
            loop {
                if room == room_state {
                    break;
                }
                if room_pos == 0 {
                    continue 'outer;
                }
                room_state[room_pos] = state;
                room_pos -= 1;
            }

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
        'outer: for (i, room) in self.rooms.into_iter().enumerate() {
            let Some(room_pos) = room.iter().position(|&s| s != State::Empty) else {
                continue 'outer
            };
            let state = room[room_pos];
            // if our room is empty/contains only our friends, we should not try to move
            if state.room() == i && room[room_pos..].iter().all(|&s| s == state) {
                continue 'outer;
            }

            // corridor positions
            let offset = i * 2 + 2;
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
        }

        output
    }

    fn heuristic(&self) -> usize {
        let mut distance = 0;
        for (i, room) in self.rooms.into_iter().enumerate() {
            for (j, state) in room.into_iter().enumerate() {
                if state.room() != i {
                    distance += state.cost() * (4 + j);
                }
            }
        }
        for (i, state) in self.corridor.into_iter().enumerate() {
            let x = state.room();
            let offset = x * 2 + 2;
            distance += state.cost() * (offset.abs_diff(i) + 1)
        }
        distance
    }

    fn solve(&self) -> usize {
        let (_, cost) = pathfinding::directed::astar::astar(self, Position::successors, Position::heuristic, |x| {
            x == &Position::SUCCESS
        })
        .unwrap();
        cost
    }
}

impl Challenge for Day23 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> usize {
        self.0.solve()
    }

    fn part_two(self) -> usize {
        // unfold
        // #D#C#B#A#
        // #D#B#A#C#
        let Position { rooms, corridor } = self.0;
        let rooms = [
            [rooms[0][0], State::D, State::D, rooms[0][1]],
            [rooms[1][0], State::C, State::B, rooms[1][1]],
            [rooms[2][0], State::B, State::A, rooms[2][1]],
            [rooms[3][0], State::A, State::C, rooms[3][1]],
        ];
        let pos = Position { rooms, corridor };
        pos.solve()
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
    fn part_one() {
        let output = Day23::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 12521);
    }

    #[test]
    fn part_two() {
        let output = Day23::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 44169);
    }
}
