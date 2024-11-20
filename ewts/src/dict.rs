#![cfg_attr(rustfmt, rustfmt_skip)]

pub(crate) type EwtsAnUnicodeStr = (&'static str, &'static str);

type Combinations<'a> = &'a [(
    Con,              // sup or sub
    &'a [Con],        // combination with one
    &'a [(Con, Con)], // with two
)];

pub(crate) static SUPERSCRIPTS: Combinations<'static> = &[
    // (["k","g","ng","j","ny","t","d","n","b","m","ts","dz" ||| "k+y","g+y","m+y","b+w","ts+w","g+w"])
    (
        Con::R,
        &[
            Con::K, Con::G, Con::Ng, Con::J, Con::Ny, Con::T, Con::D, Con::N, Con::B, Con::M, Con::Ts, Con::Dz,
        ],
        &[
            (Con::K, Con::Y),
            (Con::G, Con::Y),
            (Con::M, Con::Y),
            (Con::B, Con::W),
            (Con::Ts, Con::W),
            (Con::G, Con::W),
        ],
    ),
    //
    // (["k","g","ng","c","j","t","d","p","b","h"])
    (
        Con::L,
        &[
            Con::K, Con::G, Con::Ng, Con::C, Con::J, Con::T, Con::D, Con::P, Con::B, Con::H,
        ],
        &[],
    ),
    //
    //  ([
    //      "k","g","ng","ny","t","d","n","p","b","m","ts" 
    //      ||| 
    //      "k+y","g+y","p+y","b+y","m+y","k+r","g+r","p+r","b+r","m+r","n+r"
    //  ])
    (
        Con::S,
        &[
            Con::K, Con::G, Con::Ng, Con::Ny, Con::T, Con::D, Con::N, Con::P, Con::B, Con::M, Con::Ts,
        ],
        &[
            (Con::K, Con::Y),
            (Con::G, Con::Y),
            (Con::P, Con::Y),
            (Con::B, Con::Y),
            (Con::M, Con::Y),
            (Con::K, Con::R),
            (Con::G, Con::R),
            (Con::P, Con::R),
            (Con::B, Con::R),
            (Con::M, Con::R),
            (Con::N, Con::R),
        ],
    ),
];

pub(crate) static SUBSCRIPTS: Combinations<'static> = &[
    // (["k","kh","g","p","ph","b","m" ||| "r+k","r+g","r+m","s+k","s+g","s+p","s+b","s+m"])
    (
        Con::Y,
        &[
            Con::K, Con::Kh, Con::G, Con::P, Con::Ph, Con::B, Con::M
        ],
        &[
            (Con::R, Con::K),
            (Con::R, Con::G),
            (Con::R, Con::M),
            (Con::S, Con::K),
            (Con::S, Con::G),
            (Con::S, Con::P),
            (Con::S, Con::B),
            (Con::S, Con::M),
        ],
    ),
    // (["k","kh","g","t","th","d","n","p","ph","b","m","sh","s","h","dz" ||| "s+k","s+g","s+p","s+b","s+m","s+n"])
    (
        Con::R,
        &[
            Con::K, Con::Kh, Con::G, Con::T, Con::Th, Con::D, Con::N, Con::P, Con::Ph, Con::B, Con::M, Con::Sh, Con::S, Con::H, Con::Dz,
        ],
        &[
            (Con::S, Con::K),
            (Con::S, Con::G),
            (Con::S, Con::P),
            (Con::S, Con::B),
            (Con::S, Con::M),
            (Con::S, Con::N),
        ]
    ),
    // (["k","g","b","r","s","z"])
    (
        Con::L, 
        &[
            Con::K, Con::G, Con::B, Con::R, Con::S, Con::Z
        ],
        &[]
    ),
    // (["k","kh","g","c","ny","t","d","ts","tsh","zh","z","r","l","sh","s","h" ||| "g+r","d+r","ph+y","r+g","r+ts"])
    (
        Con::W,
        &[
            Con::K, Con::Kh, Con::G, Con::C, Con::Ny, Con::T, Con::D, Con::Ts, Con::Tsh, Con::Zh, Con::Z, Con::R, Con::L, Con::Sh, Con::S, Con::H,
        ],
        &[
            (Con::G, Con::R),
            (Con::D, Con::R),
            (Con::Ph, Con::Y),
            (Con::R, Con::G),
            (Con::R, Con::Ts),
        ]
    ),
];

// TODO: del?
//static VOW_R_MINUS_I: (&str, &str) = ("r-i", "\u{0fb2}\u{0f80}");
//static VOW_R_MINUS_I_BIG: (&str, &str) = ("r-I", "\u{0fb2}\u{0f71}\u{0f80}");
//static VOW_L_MINUS_I: (&str, &str) = ("l-i", "\u{0fb3}\u{0f80}");
//static VOW_L_MINUS_I_BIG: (&str, &str) = ("l-I", "\u{0fb3}\u{0f71}\u{0f80}");
//

//TODO: remove Debug,
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Con {
    K, Kh, G, Gh, GPlusH, Ng, C, Ch, J, Ny, TBig, MinusT, TBigH, MinusTh, DBig, MinusD, DBigH, DBigPlusH, MinusDh, MinusDPlusH, NBig, MinusN, T, Th, D, Dh, DPlusH, N, P, Ph, B, Bh, BPlusH, M, Ts, Tsh, Dz, Dzh, DzPlusH, W, Zh, Z, A, Y, R, L, Sh, SBigH, MinusSh, S, H, AChen, WBig, YBig, RBig,
}

impl Con {
    pub(crate) fn list() -> &'static [Con] {
        &[
            Con::K, Con::Kh, Con::G, Con::Gh, Con::GPlusH, Con::Ng, Con::C, Con::Ch, Con::J, Con::Ny, Con::TBig, Con::MinusT, Con::TBigH, Con::MinusTh, Con::DBig, Con::MinusD, Con::DBigH, Con::DBigPlusH, Con::MinusDh, Con::MinusDPlusH, Con::NBig, Con::MinusN, Con::T, Con::Th, Con::D, Con::Dh, Con::DPlusH, Con::N, Con::P, Con::Ph, Con::B, Con::Bh, Con::BPlusH, Con::M, Con::Ts, Con::Tsh, Con::Dz, Con::Dzh, Con::DzPlusH, Con::W, Con::Zh, Con::Z, Con::A, Con::Y, Con::R, Con::L, Con::Sh, Con::SBigH, Con::MinusSh, Con::S, Con::H, Con::AChen, Con::WBig, Con::YBig, Con::RBig,
        ]
    }

    pub(crate) fn get(&self) -> (&'static str, &'static str, &'static str) {
        match &self {
            Con::K => ("k", "\u{0f40}", "\u{0f90}"),
            Con::Kh => ("kh", "\u{0f41}", "\u{0f91}"),
            Con::G => ("g", "\u{0f42}", "\u{0f92}"),
            Con::Gh => ("gh", "\u{0f42}\u{0fb7}", "\u{0f92}\u{0fb7}"),
            Con::GPlusH => ("g+h", "\u{0f42}\u{0fb7}", "\u{0f92}\u{0fb7}"),
            Con::Ng => ("ng", "\u{0f44}", "\u{0f94}"),
            Con::C => ("c", "\u{0f45}", "\u{0f95}"),
            Con::Ch => ("ch", "\u{0f46}", "\u{0f96}"),
            Con::J => ("j", "\u{0f47}", "\u{0f97}"),
            Con::Ny => ("ny", "\u{0f49}", "\u{0f99}"),
            Con::TBig => ("T", "\u{0f4a}", "\u{0f9a}"),
            Con::MinusT => ("-t", "\u{0f4a}", "\u{0f9a}"),
            Con::TBigH => ("Th", "\u{0f4b}", "\u{0f9b}"),
            Con::MinusTh => ("-th", "\u{0f4b}", "\u{0f9b}"),
            Con::DBig => ("D", "\u{0f4c}", "\u{0f9c}"),
            Con::MinusD => ("-d", "\u{0f4c}", "\u{0f9c}"),
            Con::DBigH => ("Dh", "\u{0f4c}\u{0fb7}", "\u{0f9c}\u{0fb7}"),
            Con::DBigPlusH => ("D+h", "\u{0f4c}\u{0fb7}", "\u{0f9c}\u{0fb7}"),
            Con::MinusDh => ("-dh", "\u{0f4c}\u{0fb7}", "\u{0f9c}\u{0fb7}"),
            Con::MinusDPlusH => ("-d+h", "\u{0f4c}\u{0fb7}", "\u{0f9c}\u{0fb7}"),
            Con::NBig => ("N", "\u{0f4e}", "\u{0f9e}"),
            Con::MinusN => ("-n", "\u{0f4e}", "\u{0f9e}"),
            Con::T => ("t", "\u{0f4f}", "\u{0f9f}"),
            Con::Th => ("th", "\u{0f50}", "\u{0fa0}"),
            Con::D => ("d", "\u{0f51}", "\u{0fa1}"),
            Con::Dh => ("dh", "\u{0f51}\u{0fb7}", "\u{0fa1}\u{0fb7}"),
            Con::DPlusH => ("d+h", "\u{0f51}\u{0fb7}", "\u{0fa1}\u{0fb7}"),
            Con::N => ("n", "\u{0f53}", "\u{0fa3}"),
            Con::P => ("p", "\u{0f54}", "\u{0fa4}"),
            Con::Ph => ("ph", "\u{0f55}", "\u{0fa5}"),
            Con::B => ("b", "\u{0f56}", "\u{0fa6}"),
            Con::Bh => ("bh", "\u{0f56}\u{0fb7}", "\u{0fa6}\u{0fb7}"),
            Con::BPlusH => ("b+h", "\u{0f56}\u{0fb7}", "\u{0fa6}\u{0fb7}"),
            Con::M => ("m", "\u{0f58}", "\u{0fa8}"),
            Con::Ts => ("ts", "\u{0f59}", "\u{0fa9}"),
            Con::Tsh => ("tsh", "\u{0f5a}", "\u{0faa}"),
            Con::Dz => ("dz", "\u{0f5b}", "\u{0fab}"),
            Con::Dzh => ("dzh", "\u{0f5b}\u{0fb7}", "\u{0fab}\u{0fb7}"),
            Con::DzPlusH => ("dz+h", "\u{0f5b}\u{0fb7}", "\u{0fab}\u{0fb7}"),
            Con::W => ("w", "\u{0f5d}", "\u{0fad}"),
            Con::Zh => ("zh", "\u{0f5e}", "\u{0fae}"),
            Con::Z => ("z", "\u{0f5f}", "\u{0faf}"),
            Con::A => ("'", "\u{0f60}", "\u{0fb0}"),
            Con::Y => ("y", "\u{0f61}", "\u{0fb1}"),
            Con::R => ("r", "\u{0f62}", "\u{0fb2}"),
            Con::L => ("l", "\u{0f63}", "\u{0fb3}"),
            Con::Sh => ("sh", "\u{0f64}", "\u{0fb4}"),
            Con::SBigH => ("Sh", "\u{0f65}", "\u{0fb5}"),
            Con::MinusSh => ("-sh", "\u{0f65}", "\u{0fb5}"),
            Con::S => ("s", "\u{0f66}", "\u{0fb6}"),
            Con::H => ("h", "\u{0f67}", "\u{0fb7}"),
            Con::AChen => ("a", "\u{0f68}", "\u{0fb8}"),
            Con::WBig => ("W", "\u{0f5d}", "\u{0fba}"),
            Con::YBig => ("Y", "\u{0f61}", "\u{0fbb}"),
            Con::RBig => ("R", "\u{0f6a}", "\u{0fbc}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Vowel {
    ABig, I, IBig, U, UBig, E, AI, O, AU, MinusI, MinusIBig,
}

impl Vowel {
    pub(crate) fn list() -> &'static [Vowel] {
        &[
            Vowel::ABig, Vowel::I, Vowel::IBig, Vowel::U, Vowel::UBig, Vowel::E, Vowel::AI, Vowel::O, Vowel::AU, Vowel::MinusI, Vowel::MinusIBig,
        ]
    }

    pub(crate) fn get(&self) -> EwtsAnUnicodeStr {
        match &self {
            Vowel::ABig => ("A", "\u{0f71}"),
            Vowel::I => ("i", "\u{0f72}"),
            Vowel::IBig => ("I", "\u{0f71}\u{0f72}"),
            Vowel::U => ("u", "\u{0f74}"),
            Vowel::UBig => ("U", "\u{0f71}\u{0f74}"),
            Vowel::E => ("e", "\u{0f7a}"),
            Vowel::AI => ("ai", "\u{0f7b}"),
            Vowel::O => ("o", "\u{0f7c}"),
            Vowel::AU => ("au", "\u{0f7d}"),
            Vowel::MinusI => ("-i", "\u{0f80}"),
            Vowel::MinusIBig => ("-I", "\u{0f71}\u{0f80}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sym {
    Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Space, Asterisk, Slash, TwoSlashes, Semicolon, Bar, ExclamMark, Colon, Underscore, Equal, Less, Greater, LParen, RParen, At, Hash, S, Percent,
}

impl Sym {
    pub(crate) fn list() -> &'static [Sym] {
        &[
            Sym::Zero, Sym::One, Sym::Two, Sym::Three, Sym::Four, Sym::Five, Sym::Six, Sym::Seven, Sym::Eight, Sym::Nine, Sym::Space, Sym::Asterisk, Sym::Slash, Sym::TwoSlashes, Sym::Semicolon, Sym::Bar, Sym::ExclamMark, Sym::Colon, Sym::Underscore, Sym::Equal, Sym::Less, Sym::Greater, Sym::LParen, Sym::RParen, Sym::At, Sym::Hash, Sym::S, Sym::Percent,
        ]
    }

    pub(crate) fn get(&self) -> EwtsAnUnicodeStr {
        match &self {
            Sym::Zero => ("0", "\u{0f20}"),
            Sym::One => ("1", "\u{0f21}"),
            Sym::Two => ("2", "\u{0f22}"),
            Sym::Three => ("3", "\u{0f23}"),
            Sym::Four => ("4", "\u{0f24}"),
            Sym::Five => ("5", "\u{0f25}"),
            Sym::Six => ("6", "\u{0f26}"),
            Sym::Seven => ("7", "\u{0f27}"),
            Sym::Eight => ("8", "\u{0f28}"),
            Sym::Nine => ("9", "\u{0f29}"),
            Sym::Space => (" ", "\u{0f0b}"),
            Sym::Asterisk => ("*", "\u{0f0c}"),
            Sym::Slash => ("/", "\u{0f0d}"),
            Sym::TwoSlashes => ("//", "\u{0f0e}"),
            Sym::Semicolon => (";", "\u{0f0f}"),
            Sym::Bar => ("|", "\u{0f11}"),
            Sym::ExclamMark => ("!", "\u{0f08}"),
            Sym::Colon => (":", "\u{0f14}"),
            Sym::Underscore => ("_", " "),
            Sym::Equal => ("=", "\u{0f34}"),
            Sym::Less => ("<", "\u{0f3a}"),
            Sym::Greater => (">", "\u{0f3b}"),
            Sym::LParen => ("(", "\u{0f3c}"),
            Sym::RParen => (")", "\u{0f3d}"),
            Sym::At => ("@", "\u{0f04}"),
            Sym::Hash => ("#", "\u{0f05}"),
            Sym::S => ("$", "\u{0f06}"),
            Sym::Percent => ("%", "\u{0f07}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Final {
    Anusvara, NadaBindu, ChandraBindu, Nuqta, ChandraNuqta, Visarga, Halanta, TsaPhru,
}

impl Final {
    pub(crate) fn list() -> &'static [Final] {
        &[
            Final::Anusvara, Final::NadaBindu, Final::ChandraBindu, Final::Nuqta, Final::ChandraNuqta, Final::Visarga, Final::Halanta, Final::TsaPhru,
        ]
    }
    pub(crate) fn get(&self) -> EwtsAnUnicodeStr {
        match &self {
            Final::Anusvara => ("M", "\u{0f7e}"),
            Final::NadaBindu => ("~M`", "\u{0f82}"),
            Final::ChandraBindu => ("~M", "\u{0f83}"),
            Final::Nuqta => ("X", "\u{0f37}"),
            Final::ChandraNuqta => ("~X", "\u{0f35}"),
            Final::Visarga => ("H", "\u{0f7f}"),
            Final::Halanta => ("?", "\u{0f84}"),
            Final::TsaPhru => ("^", "\u{0f39}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConSpec {
    F, V, Paluta, Plus, Period,
}

impl ConSpec {
    pub(crate) fn list() -> &'static [ConSpec] {
        &[
            ConSpec::F, ConSpec::V, ConSpec::Paluta, ConSpec::Plus, ConSpec::Period,
        ]
    }

    pub(crate) fn get(&self) -> EwtsAnUnicodeStr {
        match &self {
            ConSpec::F => ("f", "\u{0f55}\u{0f39}"),
            ConSpec::V => ("v", "\u{0f56}\u{0f39}"),
            ConSpec::Paluta => ("&", "\u{0f85}"),
            ConSpec::Plus => ("+", ""),
            ConSpec::Period => (".", ""),
        }
    }
}
