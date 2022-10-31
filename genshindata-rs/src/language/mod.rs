use strum::{Display, EnumCount, EnumIter, EnumString};

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, Display, EnumCount, EnumIter, EnumString)]
pub enum Language {
    CHS,
    CHT,
    DE,
    EN,
    ES,
    FR,
    ID,
    JP,
    KR,
    PT,
    RU,
    TH,
    VI,
}
