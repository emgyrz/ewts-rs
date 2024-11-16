pub(crate) const CONS_LEN: usize = 54;
pub(crate) const VOWELS_LEN: usize = 12;
pub(crate) const SYM_LEN: usize = 28;

pub(crate) static CONSONANTS: [(Con, &str, &str, &str); CONS_LEN] = [
    (Con::K, "k", "\u{0f40}", "\u{0f90}"),
    (Con::Kh, "kh", "\u{0f41}", "\u{0f91}"),
    (Con::G, "g", "\u{0f42}", "\u{0f92}"),
    (Con::Gh, "gh", "\u{0f42}\u{0fb7}", "\u{0f92}\u{0fb7}"),
    (Con::GPlusH, "g+h", "\u{0f42}\u{0fb7}", "\u{0f92}\u{0fb7}"),
    (Con::Ng, "ng ", "\u{0f44}", "\u{0f94}"),
    (Con::C, "c", "\u{0f45}", "\u{0f95}"),
    (Con::Ch, "ch", "\u{0f46}", "\u{0f96}"),
    (Con::J, "j", "\u{0f47}", "\u{0f97}"),
    (Con::Ny, "ny", "\u{0f49}", "\u{0f99}"),
    (Con::TBig, "T", "\u{0f4a}", "\u{0f9a}"),
    (Con::MinusT, "-t", "\u{0f4a}", "\u{0f9a}"),
    (Con::TBigH, "Th", "\u{0f4b}", "\u{0f9b}"),
    (Con::MinusTh, "-th", "\u{0f4b}", "\u{0f9b}"),
    (Con::DBig, "D", "\u{0f4c}", "\u{0f9c}"),
    (Con::MinusD, "-d", "\u{0f4c}", "\u{0f9c}"),
    (Con::DBigH, "Dh", "\u{0f4c}\u{0fb7}", "\u{0f9c}\u{0fb7}"),
    (Con::DBigPlusH, "D+h", "\u{0f4c}\u{0fb7}", "\u{0f9c}\u{0fb7}"),
    (Con::MinusDh, "-dh", "\u{0f4c}\u{0fb7}", "\u{0f9c}\u{0fb7}"),
    (Con::MinusDPlusH, "-d+h", "\u{0f4c}\u{0fb7}", "\u{0f9c}\u{0fb7}"),
    (Con::NBig, "N", "\u{0f4e}", "\u{0f9e}"),
    (Con::MinusN, "-n", "\u{0f4e}", "\u{0f9e}"),
    (Con::T, "t", "\u{0f4f}", "\u{0f9f}"),
    (Con::Th, "th", "\u{0f50}", "\u{0fa0}"),
    (Con::D, "d", "\u{0f51}", "\u{0fa1}"),
    (Con::Dh, "dh", "\u{0f51}\u{0fb7}", "\u{0fa1}\u{0fb7}"),
    (Con::DPlusH, "d+h", "\u{0f51}\u{0fb7}", "\u{0fa1}\u{0fb7}"),
    (Con::N, "n", "\u{0f53}", "\u{0fa3}"),
    (Con::P, "p", "\u{0f54}", "\u{0fa4}"),
    (Con::Ph, "ph", "\u{0f55}", "\u{0fa5}"),
    (Con::B, "b", "\u{0f56}", "\u{0fa6}"),
    (Con::Bh, "bh", "\u{0f56}\u{0fb7}", "\u{0fa6}\u{0fb7}"),
    (Con::BPlusH, "b+h", "\u{0f56}\u{0fb7}", "\u{0fa6}\u{0fb7}"),
    (Con::M, "m", "\u{0f58}", "\u{0fa8}"),
    (Con::Ts, "ts", "\u{0f59}", "\u{0fa9}"),
    (Con::Tsh, "tsh", "\u{0f5a}", "\u{0faa}"),
    (Con::Dz, "dz", "\u{0f5b}", "\u{0fab}"),
    (Con::Dzh, "dzh", "\u{0f5b}\u{0fb7}", "\u{0fab}\u{0fb7}"),
    (Con::DzPlusH, "dz+h", "\u{0f5b}\u{0fb7}", "\u{0fab}\u{0fb7}"),
    (Con::W, "w", "\u{0f5d}", "\u{0fad}"),
    (Con::Zh, "zh", "\u{0f5e}", "\u{0fae}"),
    (Con::Z, "z", "\u{0f5f}", "\u{0faf}"),
    (Con::A, "'", "\u{0f60}", "\u{0fb0}"),
    (Con::Y, "y", "\u{0f61}", "\u{0fb1}"),
    (Con::R, "r", "\u{0f62}", "\u{0fb2}"),
    (Con::L, "l", "\u{0f63}", "\u{0fb3}"),
    (Con::Sh, "sh", "\u{0f64}", "\u{0fb4}"),
    (Con::SBigH, "Sh", "\u{0f65}", "\u{0fb5}"),
    (Con::MinusSh, "-sh", "\u{0f65}", "\u{0fb5}"),
    (Con::S, "s", "\u{0f66}", "\u{0fb6}"),
    (Con::H, "h", "\u{0f67}", "\u{0fb7}"),
    (Con::WBig, "W", "\u{0f5d}", "\u{0fba}"),
    (Con::YBig, "Y", "\u{0f61}", "\u{0fbb}"),
    (Con::RBig, "R", "\u{0f6a}", "\u{0fbc}"),
];

pub(crate) static VOWELS: [(Vowel, &str, &str); VOWELS_LEN] = [
    (Vowel::A, "a", "\u{0f68}"),
    (Vowel::ABig, "A", "\u{0f71}"),
    (Vowel::I, "i", "\u{0f72}"),
    (Vowel::IBig, "I", "\u{0f71}\u{0f72}"),
    (Vowel::U, "u", "\u{0f74}"),
    (Vowel::UBig, "U", "\u{0f71}\u{0f74}"),
    (Vowel::E, "e", "\u{0f7a}"),
    (Vowel::AI, "ai", "\u{0f7b}"),
    (Vowel::O, "o", "\u{0f7c}"),
    (Vowel::AU, "au", "\u{0f7d}"),
    (Vowel::MinusI, "-i", "\u{0f80}"),
    (Vowel::MinusIBig, "-I", "\u{0f71}\u{0f80}"),
];

pub(crate) static SYM: [(Sym, &str, &str); SYM_LEN] = [
    (Sym::Zero, "0", "\u{0f20}"),
    (Sym::One, "1", "\u{0f21}"),
    (Sym::Two, "2", "\u{0f22}"),
    (Sym::Three, "3", "\u{0f23}"),
    (Sym::Four, "4", "\u{0f24}"),
    (Sym::Five, "5", "\u{0f25}"),
    (Sym::Six, "6", "\u{0f26}"),
    (Sym::Seven, "7", "\u{0f27}"),
    (Sym::Eight, "8", "\u{0f28}"),
    (Sym::Nine, "9", "\u{0f29}"),
    (Sym::Space, " ", "\u{0f0b}"),
    (Sym::Asterisk, "*", "\u{0f0c}"),
    (Sym::Slash, "/", "\u{0f0d}"),
    (Sym::TwoSlashes, "//", "\u{0f0e}"),
    (Sym::Semicolon, ";", "\u{0f0f}"),
    (Sym::Bar, "|", "\u{0f11}"),
    (Sym::ExclamMark, "!", "\u{0f08}"),
    (Sym::Colon, ",", "\u{0f14}"),
    (Sym::Underscore, "_", " "),
    (Sym::Equal, "=", "\u{0f34}"),
    (Sym::Less, "<", "\u{0f3a}"),
    (Sym::Greater, ">", "\u{0f3b}"),
    (Sym::LParen, "(", "\u{0f3c}"),
    (Sym::RParen, ")", "\u{0f3d}"),
    (Sym::At, "@", "\u{0f04}"),
    (Sym::Hash, "#", "\u{0f05}"),
    (Sym::S, "$", "\u{0f06}"),
    (Sym::Percent, "%", "\u{0f07}"),
];

// special sanskrit vowels
// TODO: del?
//static VOW_R_MINUS_I: (&str, &str) = ("r-i", "\u{0fb2}\u{0f80}");
//static VOW_R_MINUS_I_BIG: (&str, &str) = ("r-I", "\u{0fb2}\u{0f71}\u{0f80}");
//static VOW_L_MINUS_I: (&str, &str) = ("l-i", "\u{0fb3}\u{0f80}");
//static VOW_L_MINUS_I_BIG: (&str, &str) = ("l-I", "\u{0fb3}\u{0f71}\u{0f80}");
//
//
//

//#[derive(PartialEq, Eq, Hash, Clone)]
//TODO: remove Debug,
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Con {
    K,
    Kh,
    G,
    Gh,
    GPlusH,
    Ng,
    C,
    Ch,
    J,
    Ny,
    TBig,
    MinusT,
    TBigH,
    MinusTh,
    DBig,
    MinusD,
    DBigH,
    DBigPlusH,
    MinusDh,
    MinusDPlusH,
    NBig,
    MinusN,
    T,
    Th,
    D,
    Dh,
    DPlusH,
    N,
    P,
    Ph,
    B,
    Bh,
    BPlusH,
    M,
    Ts,
    Tsh,
    Dz,
    Dzh,
    DzPlusH,
    W,
    Zh,
    Z,
    A,
    Y,
    R,
    L,
    Sh,
    SBigH,
    MinusSh,
    S,
    H,
    WBig,
    YBig,
    RBig,
}

impl Con {}

//todo: remove Debug,
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Vowel {
    A,
    ABig,
    I,
    IBig,
    U,
    UBig,
    E,
    AI,
    O,
    AU,
    MinusI,
    MinusIBig,
}

impl Vowel {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sym {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Space,
    Asterisk,
    Slash,
    TwoSlashes,
    Semicolon,
    Bar,
    ExclamMark,
    Colon,
    Underscore,
    Equal,
    Less,
    Greater,
    LParen,
    RParen,
    At,
    Hash,
    S,
    Percent,
}

// TODO: special
//static CON_F: (&str, &str) = ("f", "\u{0f55}\u{0f39}");
//static CON_V: (&str, &str) = ("v", "\u{0f56}\u{0f39}");
//static SUB_A: (&str, &str) = ("a", "\u{0fb8}");

//
//// stuff that can come after the vowel
//// symbol => [ unicode, class ]  (cannot have more than 1 of the same class in the same stack)
//static finals = {
//  "M": [ "\u0f7e", "M" ],   // anusvara / bindu / circle above / nga ro
//  "~M`": [ "\u0f82", "M" ], // crescent, bindu & nada
//  "~M": [ "\u0f83", "M" ],  // crescent & bindu
//  "X": [ "\u0f37", "X" ],   // small circle under
//  "~X": [ "\u0f35", "X" ],  // small circle w/ crescent under
//  "H": [ "\u0f7f", "H" ],   // visarga / rnam bcad
//  "?": [ "\u0f84", "?" ],   // halanta / srog med
//  "^": [ "\u0f39", "^" ],   // tsa-phru
//  "&": [ "\u0f85", "&" ],   // paluta / avagraha
//};
