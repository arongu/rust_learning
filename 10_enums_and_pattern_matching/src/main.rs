mod a_enum_with_values;
mod b_enum_complex_with_associated_function;
mod c_optional;
mod d_enum_with_match;
mod f_more_match;

fn main() {
    a_enum_with_values::example1();
    b_enum_complex_with_associated_function::example1();
    c_optional::example1();
    c_optional::example2();
    d_enum_with_match::example1();
    f_more_match::example1();
}
