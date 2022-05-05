use crate::Value;
use gobble::*;
use std::collections::HashMap;

pub fn wsn_<P: Parser>(p: P) -> impl Parser<Out = P::Out> {
    middle(WSL.istar(), p, WSL.istar())
}

parser! {(UniEscape ->char)
    ("\\u", HexDigit.exact(4).brk())
        .try_map(|(_,v)| {
            let n : u32 = u32::from_str_radix(&v,16).map_err(|_| Expected::Str("4 Hex digits"))?;
            std::char::from_u32(n).ok_or(Expected::Str("4 hex digits"))
        })
}

parser! {(BackEsc-> char)
    '\\'.ig_then( or!(
        'b'.asv('\u{08}'),
        'f'.asv('\u{0C}'),
        'n'.asv('\n'),
        'r'.asv('\r'),
        't'.asv('\t'),
        "\"\\".one(),
    )),
}

parser! { (JsonChar ->char)
    or!(
        UniEscape,
        BackEsc,
        not("\\\"").one()
    )
}

parser! {(JsonString -> String)
    "\"".ig_then(chars_until(JsonChar,"\"")).map(|(a,_)|a)
}

parser! {(MapItem->(String,Value))
    (JsonString, wsn_(":"),JsonValue).map(|(a,_,b)|(a,b))
}

parser! {(JsonValue->Value)
    wsn_(or!(
        "null".map(|_|Value::Null),
        common::Bool.map(|b| Value::Bool(b)),
        common::Float.map(|f| Value::Float(f)),
        common::Int.map(|i| Value::Int(i)),
        JsonString.map(|s| Value::Str(s)),
        "[".ig_then(sep_until_ig(JsonValue,",","]")).map(|a| Value::Array(a)),
        "{".ig_then(sep_until_ig(wsn_(MapItem),",","}")).map(|a| {
            let mut m = HashMap::new();
            for (k,v) in a {
                m.insert(k,v);
            }
            Value::Object(m)
        })
    ))
}
