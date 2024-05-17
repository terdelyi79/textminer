use super::parser_context::ParserContext;
use crate::error::Error;
use tokio::io::{AsyncRead, AsyncWrite};

// Parse HTML entities like &nbsp; or &amp;
pub struct EntityParser {}

impl EntityParser {
    // Method is called when an & character indicated that an entity is starting
    pub async fn parse<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send>(
        context: &mut ParserContext<'_, R, W>,
    ) -> Result<(), Error> {
        loop {
            match context.input.read_char().await? {
                // ';' character indicates the nd of the entity
                ';' => {
                    // Extract the entity and write to the output
                    EntityParser::extract_entity(context).await?;
                    context.buffer.clear();

                    return Ok(());
                }
                // All other caharcters are simply stored in the buffer
                ch => {
                    context.buffer.push(ch);
                }
            }
        }
    }

    // Function to extract the entity from its name or code
    async fn extract_entity<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send>(
        context: &mut ParserContext<'_, R, W>,
    ) -> Result<(), Error> {
        let char_code: u32;

        // The '#' character indicated that entity is defined by its code in unicode
        if context.buffer.starts_with('#') {
            let mut buf = context.buffer.strip_prefix('#').unwrap();

            // If the 'x' prefix is used, then code is in hexadecimal format
            if buf.starts_with('x') {
                buf = buf.strip_prefix('x').expect("Already checked.");
                char_code = u32::from_str_radix(buf, 16).unwrap();
            // Code is in decimal format otherwise
            } else {
                char_code = buf.parse::<u32>().unwrap();
            }
        // Entity is defined by a name
        } else {
            char_code = match context.buffer.as_str() {
                "amp" => 38,
                "lt" => 60,
                "gt" => 62,
                "nbsp" => 32,
                "iexcl" => 161,
                "cent" => 162,
                "pound" => 163,
                "curren" => 164,
                "yen" => 165,
                "brvbar" => 166,
                "sect" => 167,
                "uml" => 168,
                "copy" => 169,
                "ordf" => 170,
                "laquo" => 171,
                "not" => 172,
                "shy" => 173,
                "reg" => 174,
                "macr" => 175,
                "deg" => 176,
                "plusmn" => 177,
                "sup2" => 178,
                "sup3" => 179,
                "acute" => 180,
                "micro" => 181,
                "para" => 182,
                "cedil" => 184,
                "sup1" => 185,
                "ordm" => 186,
                "raquo" => 187,
                "frac14" => 188,
                "frac12" => 189,
                "frac34" => 190,
                "iquest" => 191,
                "times" => 215,
                "divide" => 247,
                "forall" => 8704,
                "part" => 8706,
                "exist" => 8707,
                "empty" => 8709,
                "nabla" => 8711,
                "isin" => 8712,
                "notin" => 8713,
                "ni" => 8715,
                "prod" => 8719,
                "sum" => 8721,
                "minus" => 8722,
                "lowast" => 8727,
                "radic" => 8730,
                "prop" => 8733,
                "infin" => 8734,
                "ang" => 8736,
                "and" => 8743,
                "or" => 8744,
                "cap" => 8745,
                "cup" => 8746,
                "int" => 8747,
                "there4" => 8756,
                "sim" => 8764,
                "cong" => 8773,
                "asymp" => 8776,
                "ne" => 8800,
                "equiv" => 8801,
                "le" => 8804,
                "ge" => 8805,
                "sub" => 8834,
                "sup" => 8835,
                "nsub" => 8836,
                "sube" => 8838,
                "supe" => 8839,
                "oplus" => 8853,
                "otimes" => 8855,
                "perp" => 8869,
                "sdot" => 8901,
                "Alpha" => 913,
                "Beta" => 914,
                "Gamma" => 915,
                "Delta" => 916,
                "Epsilon" => 917,
                "Zeta" => 918,
                "Eta" => 919,
                "Theta" => 920,
                "Iota" => 921,
                "Kappa" => 922,
                "Lambda" => 923,
                "Mu" => 924,
                "Nu" => 925,
                "Xi" => 926,
                "Omicron" => 927,
                "Pi" => 928,
                "Rho" => 929,
                "Sigma" => 931,
                "Tau" => 932,
                "Upsilon" => 933,
                "Phi" => 934,
                "Chi" => 935,
                "Psi" => 936,
                "Omega" => 937,
                "alpha" => 945,
                "beta" => 946,
                "gamma" => 947,
                "delta" => 948,
                "epsilon" => 949,
                "zeta" => 950,
                "eta" => 951,
                "theta" => 952,
                "iota" => 953,
                "kappa" => 954,
                "lambda" => 955,
                "mu" => 956,
                "nu" => 957,
                "xi" => 958,
                "omicron" => 959,
                "pi" => 960,
                "rho" => 961,
                "sigmaf" => 962,
                "sigma" => 963,
                "tau" => 964,
                "upsilon" => 965,
                "phi" => 966,
                "chi" => 967,
                "psi" => 968,
                "omega" => 969,
                "thetasym" => 977,
                "upsih" => 978,
                "piv" => 982,
                "OElig" => 338,
                "oelig" => 339,
                "Scaron" => 352,
                "scaron" => 353,
                "Yuml" => 376,
                "fnof" => 402,
                "circ" => 710,
                "tilde" => 732,
                "ensp" => 8194,
                "emsp" => 8195,
                "thinsp" => 8201,
                "zwnj" => 8204,
                "zwj" => 8205,
                "lrm" => 8206,
                "rlm" => 8207,
                "ndash" => 8211,
                "mdash" => 8212,
                "lsquo" => 8216,
                "rsquo" => 8217,
                "sbquo" => 8218,
                "ldquo" => 8220,
                "rdquo" => 8221,
                "bdquo" => 8222,
                "dagger" => 8224,
                "Dagger" => 8225,
                "bull" => 8226,
                "hellip" => 8230,
                "permil" => 8240,
                "prime" => 8242,
                "Prime" => 8243,
                "lsaquo" => 8249,
                "rsaquo" => 8250,
                "oline" => 8254,
                "euro" => 8364,
                "trade" => 8482,
                "larr" => 8592,
                "uarr" => 8593,
                "rarr" => 8594,
                "darr" => 8595,
                "harr" => 8596,
                "crarr" => 8629,
                "lceil" => 8968,
                "rceil" => 8969,
                "lfloor" => 8970,
                "rfloor" => 8971,
                "loz" => 9674,
                "spades" => 9824,
                "clubs" => 9827,
                "hearts" => 9829,
                "diams" => 9830,
                _ => 0,
            }
        }
        // Write the character defined by the entity to the output
        context
            .write(char::from_u32(char_code).expect("HTML should contain valid entity codes."))
            .await?;

        Ok(())
    }
}
