extern crate korean_nums;

use korean_nums::{calculate, str_to_vec_string};

#[test]
fn it_calculates_sino_one_correctly() {
    let strings = str_to_vec_string("1");
    assert_eq!("일", &calculate(strings));
}

#[test]
fn it_calculates_one_hundred_twenty_thousand_correctly() {
    let strings = str_to_vec_string("120000");
    assert_eq!("십이만", &calculate(strings));
}

#[test]
fn it_calculates_one_million_correctly() {
    let strings = str_to_vec_string("1000000");
    assert_eq!("백만", &calculate(strings));
}

#[test]
fn it_calculates_one_digit_correctly() {
    let strings = str_to_vec_string("4");
    assert_eq!("사", &calculate(strings));
}

#[test]
fn it_calculates_two_digits_correctly() {
    let strings = str_to_vec_string("12");
    assert_eq!("십이", &calculate(strings));
}

#[test]
fn it_calculates_three_digits_correctly() {
    let strings = str_to_vec_string("123");
    assert_eq!("백이십삼", &calculate(strings));
}

#[test]
fn it_calculates_four_digits_correctly() {
    let strings = str_to_vec_string("1234");
    assert_eq!("천이백삼십사", &calculate(strings));
}

#[test]
fn it_calculates_five_digits_correctly() {
    let strings = str_to_vec_string("12345");
    assert_eq!("만 이천삼백사십오", &calculate(strings));
}

#[test]
fn it_calculates_six_digits_correctly() {
    let strings = str_to_vec_string("123456");
    assert_eq!("십이만 삼천사백오십육", &calculate(strings));
}

#[test]
fn it_calculates_seven_digits_correctly() {
    let strings = str_to_vec_string("1234567");
    assert_eq!("백이십삼만 사천오백육십칠", &calculate(strings));
}

#[test]
fn it_calculates_eight_digits_correctly() {
    let strings = str_to_vec_string("12345678");
    assert_eq!(
        "천이백삼십사만 오천육백칠십팔",
        &calculate(strings)
    );

    let strings = str_to_vec_string("53565453");
    assert_eq!(
        "오천삼백오십육만 오천사백오십삼",
        &calculate(strings)
    );

    let strings = str_to_vec_string("99999999");
    assert_eq!(
        "구천구백구십구만 구천구백구십구",
        &calculate(strings)
    );
}

#[test]
fn it_calculates_nine_digits_correctly() {
    let strings = str_to_vec_string("100000000");
    assert_eq!("일억", &calculate(strings));
}

#[test]
fn it_calculates_ten_digits_correctly() {
    let strings = str_to_vec_string("1234567891");
    assert_eq!(
        "십이억 삼천사백오십육만 칠천팔백구십일",
        &calculate(strings)
    );
}

#[test]
fn it_calculates_eleven_digits_correctly() {
    let strings = str_to_vec_string("12345678912");
    assert_eq!(
        "백이십삼억 사천오백육십칠만 팔천구백십이",
        &calculate(strings)
    );
}

#[test]
fn it_calculates_twelve_digits_correctly() {
    let strings = str_to_vec_string("123456789123");
    assert_eq!(
        "천이백삼십사억 오천육백칠십팔만 구천백이십삼",
        &calculate(strings)
    );

    let strings = str_to_vec_string("999999999999");
    assert_eq!(
        "구천구백구십구억 구천구백구십구만 구천구백구십구",
        &calculate(strings)
    );
}

// 1 trillion
#[test]
fn it_calculates_thirteen_digits_correctly() {
    let strings = str_to_vec_string("1000000000000");
    assert_eq!("일조", &calculate(strings));

    let strings = str_to_vec_string("1234567891234");
    assert_eq!(
        "일조 이천삼백사십오억 육천칠백팔십구만 천이백삼십사",
        &calculate(strings)
    );

    let strings = str_to_vec_string("3543454632455");
    assert_eq!(
        "삼조 오천사백삼십사억 오천사백육십삼만 이천사백오십오",
        &calculate(strings)
    );
}

#[test]
fn it_calculates_fourteen_digits_correctly() {
    let strings = str_to_vec_string("12345678912345");
    assert_eq!(
        "십이조 삼천사백오십육억 칠천팔백구십일만 이천삼백사십오",
        &calculate(strings)
    );
}

#[test]
fn it_calculates_fifteen_digits_correctly() {
    let strings = str_to_vec_string("123456789123456");
    assert_eq!(
        "백이십삼조 사천오백육십칠억 팔천구백십이만 삼천사백오십육",
        &calculate(strings)
    );
}

#[test]
fn it_calculates_sixteen_digits_correctly() {
    let strings = str_to_vec_string("1234567891234567");
    assert_eq!("천이백삼십사조 오천육백칠십팔억 구천백이십삼만 사천오백육십칠", &calculate(strings));

    let strings = str_to_vec_string("9999999999999999");
    assert_eq!("구천구백구십구조 구천구백구십구억 구천구백구십구만 구천구백구십구", &calculate(strings));
}

// ten-thousand trillions, ten quadrillions
#[test]
fn it_calculates_seventeen_digits_correctly() {
    let strings = str_to_vec_string("999900000000000000");
    assert_eq!("천구백구십구경", &calculate(strings));
}
