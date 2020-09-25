#![warn(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
    /// Red, 赤
    Kok1,

    /// Black, 黒
    Huok2,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Profession {
    /// Vessel, 船, felkana
    Nuak1,

    /// Pawn, 兵, elmer
    Kauk2,

    /// Rook, 弓, gustuer
    Gua2,

    /// Bishop, 車, vadyrd
    Kaun1,

    /// Tiger, 虎, stistyst
    Dau2,

    /// Horse, 馬, dodor
    Maun1,

    /// Clerk, 筆, kua
    Kua2,

    /// Shaman, 巫, terlsk
    Tuk2,

    /// General, 将, varxle
    Uai1,

    /// King, 王, ales
    Io,
}

use std::str::FromStr;
impl FromStr for Profession{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match &*s {
            "vessel" | "船" | "felkana" | "nuak1" => Ok(Profession::Nuak1),
            "pawn" | "兵" | "elmer" | "kauk2" => Ok(Profession::Kauk2),
            "rook" | "弓" | "gustuer" | "gua2" => Ok(Profession::Gua2),
            "bishop" | "車" | "vadyrd" | "kaun1" => Ok(Profession::Kaun1),
            "tiger" | "虎" | "stistyst" | "dau2" => Ok(Profession::Dau2),
            "horse" | "馬" | "dodor" | "maun1" => Ok(Profession::Maun1),
            "clerk" | "筆" | "kua" | "kua2" => Ok(Profession::Kua2),
            "shaman" | "巫" | "terlsk" | "tuk2" => Ok(Profession::Tuk2),
            "general" | "将" | "varxle" | "uai1" => Ok(Profession::Uai1),
            "king" | "王" | "ales" | "io" => Ok(Profession::Io),
            _ => Err(()),
        }
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" | "赤" | "kok1" => Ok(Color::Kok1),
            "black" | "黒" | "Huok2" => Ok(Color::Huok2),
            _ => Err(()),
        }
    }
}

pub mod absolute {
    use std::str::FromStr;
    #[derive(Eq, PartialEq, Clone, Copy, Debug)]
    pub enum Side {
        ASide,
        IASide,
    }

    impl FromStr for Side {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" => Ok(Side::ASide),
                "IA" => Ok(Side::IASide),
                _ => Err(()),
            }
        }
    }

    use std::ops;
    impl ops::Not for Side {
        type Output = Side;

        fn not(self) -> Self::Output {
            match self {
                Side::ASide => Side::IASide,
                Side::IASide => Side::ASide,
            }
        }
    }

    #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
    pub enum Row {
        A,
        E,
        I,
        U,
        O,
        Y,
        AI,
        AU,
        IA,
    }

    #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
    pub enum Column {
        K,
        L,
        N,
        T,
        Z,
        X,
        C,
        M,
        P,
    }

    pub type Coord = (Row, Column);

    #[must_use]
    pub fn parse_coord(coord: &str) -> Option<(Row, Column)> {
        if coord.is_empty() || coord.len() > 3 {
            return None;
        }

        let column = match coord.chars().next() {
            Some('C') => Some(Column::C),
            Some('K') => Some(Column::K),
            Some('L') => Some(Column::L),
            Some('M') => Some(Column::M),
            Some('N') => Some(Column::N),
            Some('P') => Some(Column::P),
            Some('T') => Some(Column::T),
            Some('X') => Some(Column::X),
            Some('Z') => Some(Column::Z),
            None | Some(_) => None,
        }?;

        let row = match &coord[1..coord.len()] {
            "A" => Some(Row::A),
            "AI" => Some(Row::AI),
            "AU" => Some(Row::AU),
            "E" => Some(Row::E),
            "I" => Some(Row::I),
            "O" => Some(Row::O),
            "U" => Some(Row::U),
            "Y" => Some(Row::Y),
            "IA" => Some(Row::IA),
            _ => None,
        }?;

        Some((row, column))
    }

}


