//                                                   ~-.
//                                                   ,,,;            ~-.~-.~-
//                                               (.../           ~-.~-.~-.~-.~-.
//                                           < } O~`, ,        ~-.~-.~-.~-.~-.~-.
//                                               (/    T ,     ~-.~-.~-.~-.~-.~-.~-.
//                                                   ;    T     ~-.~-.~-.~-.~-.~-.~-.
//                                                 ;   {_.~-.~-.~-.~-.~-.~-.~
//                                               ;:  .-~`    ~-.~-.~-.~-.~-.
//                                               ;.: :'    ._   ~-.~-.~-.~-.~-
//                                               ;::`-.    '-._  ~-.~-.~-.~-
//                                               ;::. `-.    '-,~-.~-.~-.
//                                                   ';::::.`''-.-'
//                                                   ';::;;:,:'
//                                                       '||T
//                                                     __   _
//                                                       / |

use common::*;
use hex_literal::hex;
use sp_runtime::traits::Convert;
use xcm::prelude::*;

const AVA: [u8; 32] = hex!("0100000000000000000000000000000000000000000000000000000000000000");
const XOR: [u8; 32] = hex!("0200000000000000000000000000000000000000000000000000000000000000");
const VAL: [u8; 32] = hex!("0200040000000000000000000000000000000000000000000000000000000000");
const XSTUSD: [u8; 32] = hex!("0200080000000000000000000000000000000000000000000000000000000000");

pub struct AvtanTokenConverter;
impl Convert<TokenId, Option<MultiLocation>> for AvtanTokenConverter {
    fn convert(id: TokenId) -> Option<MultiLocation> {
        match id {
            TokenId::KSM => Some(Parent.into()),
            TokenId::AVA => {
                Some((Parent, Parachain(2666), GeneralKey { length: 32, data: AVA }).into())
            },
            TokenId::Sora(SoraToken::XOR) => {
                Some((Parent, Parachain(2011), GeneralKey { length: 32, data: XOR }).into())
            },
            TokenId::Sora(SoraToken::VAL) => {
                Some((Parent, Parachain(2011), GeneralKey { length: 32, data: VAL }).into())
            },
            TokenId::Sora(SoraToken::XSTUSD) => {
                Some((Parent, Parachain(2011), GeneralKey { length: 32, data: XSTUSD }).into())
            },
        }
    }
}

impl Convert<MultiLocation, Option<TokenId>> for AvtanTokenConverter {
    fn convert(l: MultiLocation) -> Option<TokenId> {
        if l == MultiLocation::parent() {
            return Some(TokenId::KSM);
        }
        match l {
            MultiLocation { parents, interior } if parents == 1 => match interior {
                X2(Parachain(2666), GeneralKey { length: 32, data: k }) if k == AVA => {
                    Some(TokenId::AVA)
                },
                _ => None,
            },
            MultiLocation { parents, interior } if parents == 1 => match interior {
                X2(Parachain(2011), GeneralKey { length: 32, data: k }) if k == XOR => {
                    Some(TokenId::Sora(SoraToken::XOR))
                },
                _ => None,
            },
            MultiLocation { parents, interior } if parents == 1 => match interior {
                X2(Parachain(2011), GeneralKey { length: 32, data: k }) if k == VAL => {
                    Some(TokenId::Sora(SoraToken::VAL))
                },
                _ => None,
            },
            MultiLocation { parents, interior } if parents == 1 => match interior {
                X2(Parachain(2011), GeneralKey { length: 32, data: k }) if k == XSTUSD => {
                    Some(TokenId::Sora(SoraToken::XSTUSD))
                },
                _ => None,
            },
            _ => None,
        }
    }
}

impl Convert<MultiAsset, Option<TokenId>> for AvtanTokenConverter {
    fn convert(a: MultiAsset) -> Option<TokenId> {
        if let MultiAsset { fun: Fungible(_), id: Concrete(id) } = a {
            Self::convert(id)
        } else {
            Option::None
        }
    }
}
