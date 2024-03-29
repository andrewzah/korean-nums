extern crate korean_nums;
extern crate num;

use korean_nums::{
    hangeul_from_bigint, hangeul_from_expression, hangeul_from_int, hangeul_from_float,
    NumberSystem,
};
use num::{pow, BigInt, Float};

// -----------
// Pure Korean
// -----------

#[test]
fn it_handles_zero_through_nine_korean() {
    let testcases: Vec<(i8, &str)> = vec![
        (1, "하나"),
        (2, "둘"),
        (3, "셋"),
        (4, "넷"),
        (5, "다섯"),
        (6, "여섯"),
        (7, "일곱"),
        (8, "여덟"),
        (9, "아홉"),
    ];

    for (input, expected) in testcases {
        assert_eq!(expected, hangeul_from_int(input, NumberSystem::PureKorean).unwrap());
    }
}

#[test]
fn it_handles_ten_through_ninety_nine_pure() {
    let testcases: Vec<(i8, &str)> = vec![
        (10, "열"),
        (11, "열하나"),
        (12, "열둘"),
        (13, "열셋"),
        (14, "열넷"),
        (15, "열다섯"),
        (16, "열여섯"),
        (17, "열일곱"),
        (18, "열여덟"),
        (19, "열아홉"),
        (20, "스물"),
        (21, "스물하나"),
        (22, "스물둘"),
        (23, "스물셋"),
        (24, "스물넷"),
        (25, "스물다섯"),
        (26, "스물여섯"),
        (27, "스물일곱"),
        (28, "스물여덟"),
        (29, "스물아홉"),
        (30, "서른"),
        (31, "서른하나"),
        (32, "서른둘"),
        (33, "서른셋"),
        (34, "서른넷"),
        (35, "서른다섯"),
        (36, "서른여섯"),
        (37, "서른일곱"),
        (38, "서른여덟"),
        (39, "서른아홉"),
        (40, "마흔"),
        (41, "마흔하나"),
        (42, "마흔둘"),
        (43, "마흔셋"),
        (44, "마흔넷"),
        (45, "마흔다섯"),
        (46, "마흔여섯"),
        (47, "마흔일곱"),
        (48, "마흔여덟"),
        (49, "마흔아홉"),
        (50, "쉰"),
        (51, "쉰하나"),
        (52, "쉰둘"),
        (53, "쉰셋"),
        (54, "쉰넷"),
        (55, "쉰다섯"),
        (56, "쉰여섯"),
        (57, "쉰일곱"),
        (58, "쉰여덟"),
        (59, "쉰아홉"),
        (60, "예순"),
        (61, "예순하나"),
        (62, "예순둘"),
        (63, "예순셋"),
        (64, "예순넷"),
        (65, "예순다섯"),
        (66, "예순여섯"),
        (67, "예순일곱"),
        (68, "예순여덟"),
        (69, "예순아홉"),
        (70, "일흔"),
        (71, "일흔하나"),
        (72, "일흔둘"),
        (73, "일흔셋"),
        (74, "일흔넷"),
        (75, "일흔다섯"),
        (76, "일흔여섯"),
        (77, "일흔일곱"),
        (78, "일흔여덟"),
        (79, "일흔아홉"),
        (80, "여든"),
        (81, "여든하나"),
        (82, "여든둘"),
        (83, "여든셋"),
        (84, "여든넷"),
        (85, "여든다섯"),
        (86, "여든여섯"),
        (87, "여든일곱"),
        (88, "여든여덟"),
        (89, "여든아홉"),
        (90, "아흔"),
        (91, "아흔하나"),
        (92, "아흔둘"),
        (93, "아흔셋"),
        (94, "아흔넷"),
        (95, "아흔다섯"),
        (96, "아흔여섯"),
        (97, "아흔일곱"),
        (98, "아흔여덟"),
        (99, "아흔아홉"),
    ];

    for (input, expected) in testcases {
        assert_eq!(expected, hangeul_from_int(input, NumberSystem::PureKorean).unwrap());
    }
}

// ----
// Sino
// ----

#[test]
fn it_handles_mixed_digits_zeroes_sino() {
    let testcases: Vec<(u64, &str)> = vec![
        (1, "일"),
        (100, "백"),
        (101, "백일"),
        (108, "백팔"),
        (120, "백이십"),
        (1001, "천일"),
        (1_0001, "만 일"),
        (2_0037, "이만 삼십칠"),
        (7_0007, "칠만 칠"),
        (5_0808, "오만 팔백팔"),
        (9_0000, "구만"),
        (9_9000, "구만 구천"),
        (12_0000, "십이만"),
        (12_0001, "십이만 일"),
        (81_0652, "팔십일만 육백오십이"),
        (2044_355, "이백사만 사천삼백오십오"),
        (8300_0000, "팔천삼백만"),
        (8375_2000, "팔천삼백칠십오만 이천"),
        (8001_9459, "팔천일만 구천사백오십구"),
        (5_0337_9569, "오억 삼백삼십칠만 구천오백육십구"),
        (5_0337_9570, "오억 삼백삼십칠만 구천오백칠십"),
        (10_0110_2120, "십억 백십만 이천백이십"),
        (11_3960_3741, "십일억 삼천구백육십만 삼천칠백사십일"),
        (14_2108_6610, "십사억 이천백팔만 육천육백십"),
        (132_5964_2032, "백삼십이억 오천구백육십사만 이천삼십이"),
    ];

    for &(input, expected) in testcases.iter() {
        assert_eq!(expected, hangeul_from_int(input, NumberSystem::SinoKorean).unwrap());
    }
}

#[test]
fn it_handles_digits_sino() {
    let testcases: Vec<(u64, &str)> = vec![
        (4, "사"),
        (12, "십이"),
        (123, "백이십삼"),
        (1234, "천이백삼십사"),
        (6365, "육천삼백육십오"),
        (1_2345, "만 이천삼백사십오"),
        (12_3456, "십이만 삼천사백오십육"),
        (123_4567, "백이십삼만 사천오백육십칠"),
        (1234_5678, "천이백삼십사만 오천육백칠십팔"),
        (5356_5453, "오천삼백오십육만 오천사백오십삼"),
        (9999_9999, "구천구백구십구만 구천구백구십구"),
        (1_2345_6789, "일억 이천삼백사십오만 육천칠백팔십구"),
        (12_3456_7891, "십이억 삼천사백오십육만 칠천팔백구십일"),
        (123_4567_8912, "백이십삼억 사천오백육십칠만 팔천구백십이"),
        (
            1234_5678_9123,
            "천이백삼십사억 오천육백칠십팔만 구천백이십삼",
        ),
        (
            9999_9999_9999,
            "구천구백구십구억 구천구백구십구만 구천구백구십구",
        ),
        (
            1_2345_6789_1234,
            "일조 이천삼백사십오억 육천칠백팔십구만 천이백삼십사",
        ),
        (
            3_5434_5463_2455,
            "삼조 오천사백삼십사억 오천사백육십삼만 이천사백오십오",
        ),
        (
            12_3456_7891_2345,
            "십이조 삼천사백오십육억 칠천팔백구십일만 이천삼백사십오",
        ),
        (
            123_4567_8912_3456,
            "백이십삼조 사천오백육십칠억 팔천구백십이만 삼천사백오십육",
        ),
        (
            1234_5678_9123_4567,
            "천이백삼십사조 오천육백칠십팔억 구천백이십삼만 사천오백육십칠",
        ),
        (
            9999_9999_9999_9999,
            "구천구백구십구조 구천구백구십구억 구천구백구십구만 구천구백구십구",
        ),
    ];

    for (input, expected) in testcases {
        assert_eq!(expected, hangeul_from_int(input, NumberSystem::SinoKorean).unwrap());
    }
}

#[test]
fn it_handles_zeroes_sino() {
    let testcases: Vec<(&str, u128)> = vec![
        ("십", 10),
        ("백", 100),
        ("천", 1000),
        ("만", 1_0000),
        ("십만", 10_0000),
        ("백만", 100_0000),
        ("천만", 1000_0000),
        ("일억", 1_0000_0000),
        ("십억", pow(10u128, 9)),
        ("백억", pow(10u128, 10)),
        ("천억", pow(10u128, 11)),
        ("일조", pow(10u128, 12)),
        ("십조", pow(10u128, 13)),
        ("백조", pow(10u128, 14)),
        ("천조", pow(10u128, 15)),
        ("일경", pow(10u128, 16)),
        ("십경", pow(10u128, 17)),
        ("백경", pow(10u128, 18)),
        ("천경", pow(10u128, 19)),
        ("일해", pow(10u128, 20)),
        ("십해", pow(10u128, 21)),
        ("백해", pow(10u128, 22)),
        ("천해", pow(10u128, 23)),
        ("일자", pow(10u128, 24)),
        ("십자", pow(10u128, 25)),
        ("백자", pow(10u128, 26)),
        ("천자", pow(10u128, 27)),
        ("일양", pow(10u128, 28)),
        ("십양", pow(10u128, 29)),
        ("백양", pow(10u128, 30)),
        ("천양", pow(10u128, 31)),
        ("일구", pow(10u128, 32)),
        ("십구", pow(10u128, 33)),
        ("백구", pow(10u128, 34)),
        ("천구", pow(10u128, 35)),
        ("일간", pow(10u128, 36)),
        ("십간", pow(10u128, 37)),
        ("백간", pow(10u128, 38)),
    ];

    for (expected, input) in testcases {
        assert_eq!(expected, hangeul_from_int(input, NumberSystem::SinoKorean).unwrap());
    }
}

#[test]
fn it_handles_big_zeroes_sino() {
    let testcases: Vec<(&str, BigInt)> = vec![
        ("천간", pow(BigInt::from(10), 39)),
        ("일정", pow(BigInt::from(10), 40)),
        ("십정", pow(BigInt::from(10), 41)),
        ("백정", pow(BigInt::from(10), 42)),
        ("천정", pow(BigInt::from(10), 43)),
        ("일재", pow(BigInt::from(10), 44)),
        ("십재", pow(BigInt::from(10), 45)),
        ("백재", pow(BigInt::from(10), 46)),
        ("천재", pow(BigInt::from(10), 47)),
        ("일극", pow(BigInt::from(10), 48)),
        ("십극", pow(BigInt::from(10), 49)),
        ("백극", pow(BigInt::from(10), 50)),
        ("천극", pow(BigInt::from(10), 51)),
    ];

    for (expected, input) in testcases {
        assert_eq!(expected, hangeul_from_bigint(input).unwrap());
    }
}

#[test]
fn it_handles_mixed_digits_floats() {
    let testcases: Vec<(f64, &str)> = vec![
        (1.1, "일 점 일"),
        (2.3, "이 점 삼"),
        (1001.03, "천일 점 영삼"),
        (1_0001.03, "만 일 점 영삼"),
        (1001.030000, "천일 점 영삼"),
    ];

    for &(input, expected) in testcases.iter() {
        assert_eq!(expected, hangeul_from_float(input).unwrap());
    }
}

#[test]
fn it_handles_mixed_digits_zeroes_negative() {
    let testcases: Vec<(i32, &str)> = vec![
        (-1, "마이너스 일"),
        (-100, "마이너스 백"),
        (-101, "마이너스 백일"),
        (-108, "마이너스 백팔"),
        (-120, "마이너스 백이십"),
        (-1001, "마이너스 천일"),
        (-1_0001, "마이너스 만 일"),
    ];

    for &(input, expected) in testcases.iter() {
        assert_eq!(expected, hangeul_from_int(input, NumberSystem::SinoKorean).unwrap());
    }
}

#[test]
fn it_handles_math_expressions() {
    let testcases: Vec<(&str, &str)> = vec![
        ("1 + 1", "일 더하기 일"),
        ("1 - 1", "일 빼기 일"),
        ("2 > 1", "이는 일보다 크다"),
        ("3 > 2", "삼은 이보다 크다"),
        ("1 = 1", "일은 일이다"),
        ("1 != 3", "일은 삼이 아니다"),
        ("1 <> 3", "일은 삼이 아니다"),
        ("1 =/= 3", "일은 삼이 아니다"),
        ("1 log 3", "일 로그 삼"),
    ];

    for &(input, expected) in testcases.iter() {
        assert_eq!(expected, hangeul_from_expression(input).unwrap());
    }
}

// --------------------
// Text and Conjugation
// --------------------

//#[test]
fn it_determines_vowels() {
    let testcases = vec![
        ("이", "은", '1'),
        ("가", "는", '2'),
        ("이", "은", '3'),
        ("가", "는", '4'),
        ("가", "는", '5'),
        ("이", "은", '6'),
        ("이", "은", '7'),
        ("이", "은", '8'),
        ("가", "는", '9'),
    ];

    for (subject, topic, c) in testcases {
        //let num = KoreanNumberSino::from_char(&c).unwrap();

        //assert_eq!(topic, num.ending_type().topic_particle());
        //assert_eq!(subject, num.ending_type().subject_particle());
    }
}

//#[test]
fn math_operators() {
    let testcases = vec![
        ("1+3", ""),
        ("1-3", ""),
        ("1/3", ""),
        ("1*3", ""),
        ("1^3", ""),
        ("1^3", ""),
        ("1/3", ""),
        ("1<3", ""),
        ("1>3", ""),
        ("1<=3", ""),
        ("1>=3", ""),
        ("1=3", ""),
        ("1=3", ""),
        ("1!=3", ""),
        ("1=3", ""),
        ("1<>3", ""),
        ("1=/=3", ""),
        ("1log3", ""),
    ];
}
