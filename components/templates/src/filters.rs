use std::collections::HashMap;

use base64::{encode, decode};
use pulldown_cmark as cmark;
use tera::{Value, to_value, Result as TeraResult};


pub fn markdown(value: Value, args: HashMap<String, Value>) -> TeraResult<Value> {
    let s = try_get_value!("markdown", "value", String, value);
    let inline = match args.get("inline") {
        Some(val) => try_get_value!("markdown", "inline", bool, val),
        None => false,
    };

    let mut html = String::new();
    let parser = cmark::Parser::new(&s);
    cmark::html::push_html(&mut html, parser);

    if inline {
        html = html
            .trim_left_matches("<p>")
            // pulldown_cmark finishes a paragraph with `</p>\n`
            .trim_right_matches("</p>\n")
            .to_string();
    }

    Ok(to_value(&html).unwrap())
}


pub fn base64_encode(value: Value, _: HashMap<String, Value>) -> TeraResult<Value> {
    let s = try_get_value!("base64_encode", "value", String, value);
    Ok(
        to_value(&encode(s.as_bytes())).unwrap()
    )
}

pub fn base64_decode(value: Value, _: HashMap<String, Value>) -> TeraResult<Value> {
    let s = try_get_value!("base64_decode", "value", String, value);
    Ok(
        to_value(
            &String::from_utf8(
                decode(s.as_bytes()).unwrap()
            ).unwrap()
        ).unwrap()
    )
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use tera::{to_value};

    use super::{markdown, base64_decode, base64_encode};

    #[test]
    fn markdown_filter() {
        let result = markdown(to_value(&"# Hey").unwrap(), HashMap::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), to_value(&"<h1>Hey</h1>\n").unwrap());
    }

    #[test]
    fn markdown_filter_inline() {
        let mut args = HashMap::new();
        args.insert("inline".to_string(), to_value(true).unwrap());
        let result = markdown(to_value(&"Using `map`, `filter`, and `fold` instead of `for`").unwrap(), args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), to_value(&"Using <code>map</code>, <code>filter</code>, and <code>fold</code> instead of <code>for</code>").unwrap());
    }

    #[test]
    fn base64_encode_filter() {
        // from https://tools.ietf.org/html/rfc4648#section-10
        let tests = vec![
            ("", ""),
            ("f", "Zg=="),
            ("fo", "Zm8="),
            ("foo", "Zm9v"),
            ("foob", "Zm9vYg=="),
            ("fooba", "Zm9vYmE="),
            ("foobar", "Zm9vYmFy")
        ];
        for (input, expected) in tests {
            let args = HashMap::new();
            let result = base64_encode(to_value(input).unwrap(), args);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), to_value(expected).unwrap());
        }
    }


    #[test]
    fn base64_decode_filter() {
        let tests = vec![
            ("", ""),
            ("Zg==", "f"),
            ("Zm8=", "fo"),
            ("Zm9v", "foo"),
            ("Zm9vYg==", "foob"),
            ("Zm9vYmE=", "fooba"),
            ("Zm9vYmFy", "foobar")
        ];
        for (input, expected) in tests {
            let args = HashMap::new();
            let result = base64_decode(to_value(input).unwrap(), args);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), to_value(expected).unwrap());
        }
    }
}
