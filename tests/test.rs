extern crate korean_nums;

use korean_nums::*;

// specific numbers
#[test]
fn it_handles_sino_one() {
    let strings = parse_string("1");
    assert_eq!("일", &calculate(strings));
}


#[test]
fn it_handles_one_hundred_twenty() {
    let strings = parse_string("120");
    assert_eq!("백이십", &calculate(strings));
}

#[test]
fn it_handles_one_hundred_twenty_thousand() {
    let strings = parse_string("120,000");
    assert_eq!("십이만", &calculate(strings));
}

// <num> digits
#[test]
fn it_handles_one_digit() {
    let strings = parse_string("4");
    assert_eq!("사", &calculate(strings));
}

#[test]
fn it_handles_two_digits() {
    let strings = parse_string("12");
    assert_eq!("십이", &calculate(strings));
}

#[test]
fn it_handles_three_digits() {
    let strings = parse_string("123");
    assert_eq!("백이십삼", &calculate(strings));
}

#[test]
fn it_handles_four_digits() {
    let strings = parse_string("1,234");
    assert_eq!("천이백삼십사", &calculate(strings));
}

#[test]
fn it_handles_five_digits() {
    let strings = parse_string("12345");
    assert_eq!("만 이천삼백사십오", &calculate(strings));
}

#[test]
fn it_handles_six_digits() {
    let strings = parse_string("123456");
    assert_eq!("십이만 삼천사백오십육", &calculate(strings));
}

#[test]
fn it_handles_seven_digits() {
    let strings = parse_string("1234567");
    assert_eq!("백이십삼만 사천오백육십칠", &calculate(strings));
}

#[test]
fn it_handles_eight_digits() {
    let strings = parse_string("12345678");
    assert_eq!("천이백삼십사만 오천육백칠십팔", &calculate(strings));

    let strings = parse_string("53565453");
    assert_eq!("오천삼백오십육만 오천사백오십삼", &calculate(strings));

    let strings = parse_string("99999999");
    assert_eq!("구천구백구십구만 구천구백구십구", &calculate(strings));
}


#[test]
fn it_handles_ten_digits() {
    let strings = parse_string("1234567891");
    assert_eq!("십이억 삼천사백오십육만 칠천팔백구십일", &calculate(strings));
}

#[test]
fn it_handles_eleven_digits() {
    let strings = parse_string("12345678912");
    assert_eq!("백이십삼억 사천오백육십칠만 팔천구백십이", &calculate(strings));
}

#[test]
fn it_handles_twelve_digits() {
    let strings = parse_string("123456789123");
    assert_eq!("천이백삼십사억 오천육백칠십팔만 구천백이십삼", &calculate(strings));

    let strings = parse_string("999999999999");
    assert_eq!("구천구백구십구억 구천구백구십구만 구천구백구십구", &calculate(strings));
}

// 1 trillion
#[test]
fn it_handles_thirteen_digits() {
    let strings = parse_string("1234567891234");
    assert_eq!("조 이천삼백사십오억 육천칠백팔십구만 천이백삼십사", &calculate(strings));

    let strings = parse_string("3543454632455");
    assert_eq!("삼조 오천사백삼십사억 오천사백육십삼만 이천사백오십오", &calculate(strings));
}

#[test]
fn it_handles_fourteen_digits() {
    let strings = parse_string("12345678912345");
    assert_eq!("십이조 삼천사백오십육억 칠천팔백구십일만 이천삼백사십오", &calculate(strings));
}

#[test]
fn it_handles_fifteen_digits() {
    let strings = parse_string("123456789123456");
    assert_eq!("백이십삼조 사천오백육십칠억 팔천구백십이만 삼천사백오십육", &calculate(strings));
}

#[test]
fn it_handles_sixteen_digits() {
    let strings = parse_string("1234567891234567");
    assert_eq!("천이백삼십사조 오천육백칠십팔억 구천백이십삼만 사천오백육십칠", &calculate(strings));

    let strings = parse_string("9999999999999999");
    assert_eq!("구천구백구십구조 구천구백구십구억 구천구백구십구만 구천구백구십구", &calculate(strings));
}

// zeroes
#[test]
fn it_handles_one_zero() {
    let strings = parse_string("10");
    assert_eq!("십", &calculate(strings));
}

#[test]
fn it_handles_two_zeroes() {
    let strings = parse_string("100");
    assert_eq!("백", &calculate(strings));
}

#[test]
fn it_handles_three_zeroes() {
    let strings = parse_string("1,000");
    assert_eq!("천", &calculate(strings));
}

#[test]
fn it_handles_four_zeroes() {
    let strings = parse_string("10,000");
    assert_eq!("만", &calculate(strings));
}

#[test]
fn it_handles_five_zeroes() {
    let strings = parse_string("100,000");
    assert_eq!("십만", &calculate(strings));
}

#[test]
fn it_handles_six_zeroes() {
    let strings = parse_string("1,000,000");
    assert_eq!("백만", &calculate(strings));
}

#[test]
fn it_handles_seven_zeroes() {
    let strings = parse_string("10,000,000");
    assert_eq!("천만", &calculate(strings));
}

#[test]
fn it_handles_eight_zeroes() {
    let strings = parse_string("100,000,000");
    assert_eq!("억", &calculate(strings));
}

#[test]
fn it_handles_nine_zeroes() {
    let strings = parse_string("1,000,000,000");
    assert_eq!("십억", &calculate(strings));
}

#[test]
fn it_handles_ten_zeroes() {
    let strings = parse_string("10,000,000,000");
    assert_eq!("백억", &calculate(strings));
}

#[test]
fn it_handles_eleven_zeroes() {
    let strings = parse_string("100,000,000,000");
    assert_eq!("천억", &calculate(strings));
}

#[test]
fn it_handles_twelve_zeroes() {
    let strings = parse_string("1,000,000,000,000");
    assert_eq!("조", &calculate(strings));
}

#[test]
fn it_handles_thirteen_zeroes() {
    let strings = parse_string("10,000,000,000,000");
    assert_eq!("십조", &calculate(strings));
}

#[test]
fn it_handles_fourteen_zeroes() {
    let strings = parse_string("100,000,000,000,000");
    assert_eq!("백조", &calculate(strings));
}

#[test]
fn it_handles_fifteen_zeroes() {
    let strings = parse_string("1,000,000,000,000,000");
    assert_eq!("천조", &calculate(strings));
}

#[test]
fn it_handles_sixteen_zeroes() {
    let strings = parse_string("10,000,000,000,000,000");
    assert_eq!("경", &calculate(strings));
}
