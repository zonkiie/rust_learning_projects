/// https://stackoverflow.com/questions/29986057/is-there-is-a-way-to-get-the-field-names-of-a-struct-in-a-macro/29986760#29986760
/// https://stackoverflow.com/questions/37140768/how-to-get-struct-field-names-in-rust

macro_rules! my_macro {
    (struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }) => {
        struct $name {
            $($field_name: $field_type,)*
        }

        impl $name {
            // This is purely an example—not a good one.
            fn get_field_names() -> Vec<&'static str> {
                vec![$(stringify!($field_name)),*]
            }
        }
    }
}

my_macro! {
    struct S {
        a: String,
        b: String,
        c: String,
        d: i64,
    }
}

fn main() {
    //S::get_field_names() == vec!["a", "b"]
    println!("{:?}", S::get_field_names());
}
