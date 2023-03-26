mod a_panic;
mod b_result_enum;
mod c_error_handling_simplification;

fn main() {
    //a_panic::panic();
    //b_result_enum::open_file_or_panic();
    b_result_enum::open_file();
}
