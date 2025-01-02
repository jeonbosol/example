mod test_encode_decode;
mod test_first;
mod test_iterations;
mod test_number;
mod test_string;
mod test_vector;

fn main() {
    test_first::add_two_integers::test();

    // test - Vector
    test_vector::get_concatenation::test();
    test_vector::build_array::test();
    test_vector::running_sum::test();
    test_vector::maximum_wealth::test();
    test_vector::shuffle::test();

    // test - Numbers
    test_number::minimum_sum::test();
    test_number::num_identical_pairs::test();
    test_number::kids_with_candies::test();
    test_number::subtract_product_and_sum::test();
    test_number::smaller_numbers_than_current::test();

    // test - String
    test_string::defang_i_paddr::test();
    test_string::num_jewels_in_stones::test();
    test_string::most_words_found::test();
    test_string::sort_sentence::test();
    test_string::count_matches::test();

    // test - Encoding/Decoding
    test_encode_decode::decode::test();
    test_encode_decode::decompress_rl_elist::test();
    test_encode_decode::restore_string::test();
    test_encode_decode::decode_message::test();
    test_encode_decode::get_decimal_value::test();

    // test - Iterations
    test_iterations::final_value_after_operations::test();
    test_iterations::number_of_steps::test();
    test_iterations::create_target_array::test();
    test_iterations::number_of_matches::test();
    test_iterations::min_moves_to_seat::test();
}
