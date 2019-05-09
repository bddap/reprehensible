use crate::network_byte_order::Ne;
use crate::pow_header::PowHeader;
use crate::Datagram;

/// A datagram with a timestamped Pow tag over it's source and destination public keys.
#[derive(Clone, Debug, PartialEq)]
pub struct PowDatagram {
    pub pow_header: PowHeader,
    pub datagram: Datagram,
}

impl PowDatagram {
    pub fn score(&self) -> u32 {
        self.pow_header.pow_score(
            &self.datagram.head.destination_pk,
            &self.datagram.head.source_pk,
        )
    }

    pub fn parse(raw: &[u8]) -> Option<PowDatagram> {
        let (pow_header, rest) = PowHeader::pick(raw)?;
        let datagram = Datagram::parse(&rest)?;
        Some(PowDatagram {
            pow_header,
            datagram,
        })
    }
}
