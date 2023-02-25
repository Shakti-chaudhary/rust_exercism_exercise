pub fn expected_minutes_in_oven() -> i32 {
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    expected_minutes_in_oven() - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * 2
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
fn main() {
    let expect_minutes = expected_minutes_in_oven();
    let remain_minutes = remaining_minutes_in_oven(30);
    let preparation_time = preparation_time_in_minutes(2);
    let elapsed_time = elapsed_time_in_minutes(3, 20);

    println!( "Expected minutes: {:?}, remained minutes: {:?}, preparation minutes: {:?}, elapsed minutes: {:?}",expect_minutes,remain_minutes,preparation_time,elapsed_time );
}
