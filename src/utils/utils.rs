use human_format::{Formatter, Scales};

#[macro_export]
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Deserialize, Serialize)]
        #[allow(dead_code)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub fn fmt_num(num: i32) -> String {
    let decimals = if num > 999 { 1 } else { 0 };
    let mut scales = Scales::new();
    scales.with_base(1000).with_suffixes(vec!["", "k", "M"]);

    Formatter::new()
        .with_scales(scales)
        .with_decimals(decimals)
        .with_separator("")
        .format(num as f64)
}
