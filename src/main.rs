mod solutions;

use solutions::len_of_last_word;

use crate::solutions::roman_to_int;
use crate::solutions::two_sum;

fn main() {
    two_sum::solve();
    roman_to_int::solve();
    len_of_last_word::solve();
}
