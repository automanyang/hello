// -- utilities.rs --

pub fn snake_to_camel(ident_str: &str) -> String {
    let mut camel_ty = String::new();
    let chars = ident_str.chars();

    let mut last_char_was_underscore = true;
    for c in chars {
        match c {
            '_' => last_char_was_underscore = true,
            c if last_char_was_underscore => {
                camel_ty.extend(c.to_uppercase());
                last_char_was_underscore = false;
            }
            c => camel_ty.extend(c.to_lowercase()),
        }
    }

    camel_ty
}

#[allow(unused)]
fn snake_to_camel2(ident_str: &str) -> String {
    let mut camel_ty = String::new();
    let chars = ident_str.chars();

    let mut last_char_was_underscore = true;
    for c in chars {
        match c {
            '_' => last_char_was_underscore = true,
            c => camel_ty.push_str(&if last_char_was_underscore {
                last_char_was_underscore = false;
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }),
        }
    }

    camel_ty
}

// --

#[cfg(test)]
mod tests {
    extern crate test_case;
    use super::*;
    use test_case::test_case;

    // --

    #[test_case("abc_def" => "AbcDef".to_string(); "basic")]
    #[test_case("abc_def_" => "AbcDef".to_string(); "suffix")]
    #[test_case("_abc_def"=> "AbcDef".to_string(); "prefix")]
    #[test_case("abc__def"=> "AbcDef".to_string(); "consecutive")]
    #[test_case("aBc_dEf"=> "AbcDef".to_string(); "middle")]
    #[test_case("__abc__def__" => "AbcDef".to_string(); "double middle")]
    fn test_snake_to_camel(ident_str: &str) -> String {
        // snake_to_camel(ident_str)
        snake_to_camel2(ident_str)
    }
}
