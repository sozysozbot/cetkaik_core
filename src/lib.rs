#![warn(clippy::pedantic)]

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

pub mod absolute {
    #[derive(Eq, PartialEq, Clone, Copy, Debug)]
    pub enum Side {
        ASide,
        IASide,
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
}


