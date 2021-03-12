#[macro_export]
macro_rules! impl_consensus_encoding {
    ($thing:ident, $($field:ident),+) => (
        impl consensus_encode::Encodable for $thing {
            #[inline]
            fn consensus_encode<S: ::std::io::Write>(
                &self,
                mut s: S,
            ) -> Result<usize, ::std::io::Error> {
                let mut len = 0;
                $(len += self.$field.consensus_encode(&mut s)?;)+
                Ok(len)
            }
        }

        impl consensus_encode::Decodable for $thing {
            #[inline]
            fn consensus_decode<D: ::std::io::Read>(
                mut d: D,
            ) -> Result<$thing, consensus_encode::Error> {
                Ok($thing {
                    $($field: consensus_encode::Decodable::consensus_decode(&mut d)?),+
                })
            }
        }
    );
}
