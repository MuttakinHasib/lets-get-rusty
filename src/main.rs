mod data_types;
pub mod functions;

fn main() {
    data_types::data_types();
    let x = functions::get_sum_result(1, 2);
    println!("The sum of 1 and 2 is {}", x);
}
