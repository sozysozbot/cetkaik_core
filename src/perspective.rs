use crate::{absolute, relative};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Perspective {
    IaIsDownAndPointsUpward,
    IaIsUpAndPointsDownward,
}

impl Perspective {
    #[must_use]
    pub fn ia_is_down(self) -> bool {
        self == Perspective::IaIsDownAndPointsUpward
    }
}

#[must_use]
pub fn to_absolute_board(board: &relative::Board, p: Perspective) -> absolute::Board {
    let mut ans = std::collections::HashMap::new();
    for i in 0..8 {
        for j in 0..8 {
            if let Some(piece) = board[i][j] {
                ans.insert(to_absolute_coord([i, j], p), to_absolute_piece(piece, p));
            }
        }
    }
    ans
}

#[must_use]
pub fn to_relative_board(board: &absolute::Board, p: Perspective) -> relative::Board {
    let mut ans = [
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
    ];

    for i in 0..8 {
        for j in 0..8 {
            if let Some(piece) = board.get(&to_absolute_coord([i, j], p)) {
                ans[i][j] = Some(to_relative_piece(*piece, p))
            }
        }
    }
    ans
}

#[must_use]
pub fn to_absolute_field(field: relative::Field, p: Perspective) -> absolute::Field {
    let relative::Field {
        hop1zuo1of_downward,
        hop1zuo1of_upward,
        current_board,
    } = field;
    absolute::Field {
        board: to_absolute_board(&current_board, p),
        ia_side_hop1zuo1: match p {
            Perspective::IaIsDownAndPointsUpward => hop1zuo1of_upward
                .iter()
                .copied()
                .map(
                    |relative::NonTam2PieceUpward { color, prof }| absolute::NonTam2Piece {
                        color,
                        prof,
                    },
                )
                .collect(),
            Perspective::IaIsUpAndPointsDownward => hop1zuo1of_downward
                .iter()
                .copied()
                .map(
                    |relative::NonTam2PieceDownward { color, prof }| absolute::NonTam2Piece {
                        color,
                        prof,
                    },
                )
                .collect(),
        },
        a_side_hop1zuo1: match p {
            Perspective::IaIsDownAndPointsUpward => hop1zuo1of_downward
                .iter()
                .copied()
                .map(
                    |relative::NonTam2PieceDownward { color, prof }| absolute::NonTam2Piece {
                        color,
                        prof,
                    },
                )
                .collect(),
            Perspective::IaIsUpAndPointsDownward => hop1zuo1of_upward
                .iter()
                .copied()
                .map(
                    |relative::NonTam2PieceUpward { color, prof }| absolute::NonTam2Piece {
                        color,
                        prof,
                    },
                )
                .collect(),
        },
    }
}

#[must_use]
pub fn to_relative_field(field: absolute::Field, p: Perspective) -> relative::Field {
    let absolute::Field {
        board,
        ia_side_hop1zuo1,
        a_side_hop1zuo1,
    } = field;

    relative::Field {
        hop1zuo1of_downward: match p {
            Perspective::IaIsUpAndPointsDownward => ia_side_hop1zuo1.iter().copied(),
            Perspective::IaIsDownAndPointsUpward => a_side_hop1zuo1.iter().copied(),
        }
        .map(
            |absolute::NonTam2Piece { color, prof }| relative::NonTam2PieceDownward { color, prof },
        )
        .collect(),
        hop1zuo1of_upward: match p {
            Perspective::IaIsUpAndPointsDownward => a_side_hop1zuo1.iter().copied(),
            Perspective::IaIsDownAndPointsUpward => ia_side_hop1zuo1.iter().copied(),
        }
        .map(|absolute::NonTam2Piece { color, prof }| relative::NonTam2PieceUpward { color, prof })
        .collect(),
        current_board: to_relative_board(&board, p),
    }
}

#[must_use]
pub const fn to_absolute_side(side: relative::Side, p: Perspective) -> absolute::Side {
    match (side, p) {
        (relative::Side::Upward, Perspective::IaIsDownAndPointsUpward)
        | (relative::Side::Downward, Perspective::IaIsUpAndPointsDownward) => {
            absolute::Side::IASide
        }
        (relative::Side::Downward, Perspective::IaIsDownAndPointsUpward)
        | (relative::Side::Upward, Perspective::IaIsUpAndPointsDownward) => absolute::Side::ASide,
    }
}

#[must_use]
pub const fn to_relative_side(side: absolute::Side, p: Perspective) -> relative::Side {
    match (side, p) {
        (absolute::Side::IASide, Perspective::IaIsDownAndPointsUpward)
        | (absolute::Side::ASide, Perspective::IaIsUpAndPointsDownward) => relative::Side::Upward,
        (absolute::Side::IASide, Perspective::IaIsUpAndPointsDownward)
        | (absolute::Side::ASide, Perspective::IaIsDownAndPointsUpward) => relative::Side::Downward,
    }
}

/// Converts `absolute::Piece` into `relative::Piece`
/// # Examples
/// ```
/// use cetkaik_core::*;
/// use cetkaik_core::perspective::*;
/// assert_eq!(
///     to_relative_piece(absolute::Piece::Tam2, Perspective::IaIsDownAndPointsUpward),
///     relative::Piece::Tam2
/// );
/// assert_eq!(
///     to_relative_piece(absolute::Piece::NonTam2Piece {
///         prof: Profession::Uai1,
///         color: Color::Kok1,
///         side: absolute::Side::IASide
///     }, Perspective::IaIsDownAndPointsUpward),
///     relative::Piece::NonTam2Piece {
///         prof: Profession::Uai1,
///         color: Color::Kok1,
///         side: relative::Side::Upward
///     }
/// );
/// ```
#[must_use]
pub const fn to_relative_piece(piece: absolute::Piece, p: Perspective) -> relative::Piece {
    match piece {
        absolute::Piece::Tam2 => relative::Piece::Tam2,
        absolute::Piece::NonTam2Piece { prof, color, side } => relative::Piece::NonTam2Piece {
            prof,
            color,
            side: to_relative_side(side, p),
        },
    }
}

/// Converts `relative::Piece` into `absolute::Piece`
/// # Examples
/// ```
/// use cetkaik_core::*;
/// use cetkaik_core::perspective::*;
/// assert_eq!(
///     to_absolute_piece(relative::Piece::Tam2, Perspective::IaIsDownAndPointsUpward),
///     absolute::Piece::Tam2
/// );
/// assert_eq!(
///     to_absolute_piece(relative::Piece::NonTam2Piece {
///         prof: Profession::Uai1,
///         color: Color::Kok1,
///         side: relative::Side::Upward
///     }, Perspective::IaIsDownAndPointsUpward),
///     absolute::Piece::NonTam2Piece {
///         prof: Profession::Uai1,
///         color: Color::Kok1,
///         side: absolute::Side::IASide
///     }
/// );
/// ```
#[must_use]
pub const fn to_absolute_piece(piece: relative::Piece, p: Perspective) -> absolute::Piece {
    match piece {
        relative::Piece::Tam2 => absolute::Piece::Tam2,
        relative::Piece::NonTam2Piece { prof, color, side } => absolute::Piece::NonTam2Piece {
            prof,
            color,
            side: to_absolute_side(side, p),
        },
    }
}

/// Converts `relative::Coord` into `absolute::Coord`
/// # Examples
/// ```
/// use cetkaik_core::*;
/// use cetkaik_core::perspective::*;
/// assert_eq!(
///     to_absolute_coord([2, 4], Perspective::IaIsDownAndPointsUpward),
///     (absolute::Row::I, absolute::Column::Z)
/// )
/// ```
#[must_use]
pub fn to_absolute_coord(coord: relative::Coord, p: Perspective) -> absolute::Coord {
    let [row, col] = coord;

    let columns = vec![
        absolute::Column::K,
        absolute::Column::L,
        absolute::Column::N,
        absolute::Column::T,
        absolute::Column::Z,
        absolute::Column::X,
        absolute::Column::C,
        absolute::Column::M,
        absolute::Column::P,
    ];

    let rows = vec![
        absolute::Row::A,
        absolute::Row::E,
        absolute::Row::I,
        absolute::Row::U,
        absolute::Row::O,
        absolute::Row::Y,
        absolute::Row::AI,
        absolute::Row::AU,
        absolute::Row::IA,
    ];

    (
        rows[if p.ia_is_down() { row } else { 8 - row }],
        columns[if p.ia_is_down() { col } else { 8 - col }],
    )
}

/// Converts `absolute::Coord` into `relative::Coord`
/// # Examples
/// ```
/// use cetkaik_core::*;
/// use cetkaik_core::perspective::*;
/// assert_eq!(
///     to_relative_coord((absolute::Row::I, absolute::Column::Z), Perspective::IaIsDownAndPointsUpward),
///     [2, 4]
/// )
/// ```
#[must_use]
pub fn to_relative_coord(coord: absolute::Coord, p: Perspective) -> relative::Coord {
    let (row, col) = coord;

    let columns_col = match col {
        absolute::Column::K => 0,
        absolute::Column::L => 1,
        absolute::Column::N => 2,
        absolute::Column::T => 3,
        absolute::Column::Z => 4,
        absolute::Column::X => 5,
        absolute::Column::C => 6,
        absolute::Column::M => 7,
        absolute::Column::P => 8,
    };

    let rows_row = match row {
        absolute::Row::A => 0,
        absolute::Row::E => 1,
        absolute::Row::I => 2,
        absolute::Row::U => 3,
        absolute::Row::O => 4,
        absolute::Row::Y => 5,
        absolute::Row::AI => 6,
        absolute::Row::AU => 7,
        absolute::Row::IA => 8,
    };

    if p.ia_is_down() {
        [rows_row, columns_col]
    } else {
        [8 - rows_row, 8 - columns_col]
    }
}
