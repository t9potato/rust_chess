macro_rules! b {
    ($a:expr) => {
        Some(Peice {
            color: Color::Black,
            style: $a,
        })
    }
}

macro_rules! w {
    ($a:expr) => {
        Some(Peice {
            color: Color::White,
            style: $a,
        })
    }
}
