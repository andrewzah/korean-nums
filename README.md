## note!

For 0.6, this crate is going under a substantial re-write. The API is likely to change.

## Korean Numbers

Converting a few different types to hangeul is supported:

* int
* money (f64)
* bigint
* str
* some math expressions

You can choose between Sino-Korean numbers (based on Korean's adopted Chinese characters, 한자), or Pure Korean numbers.

### Example Usage

```rust
// Get Pure Korean Hangeul
assert_eq!("서른둘", hangeul_from_int(30, false));
assert_eq!("둘", hangeul_from_string(String::from("2"), false));

// Get Sino-Korean Hangeul
assert_eq!("백이십", hangeul_from_int(120, true));
assert_eq!("만 이천삼백사십오", hangeul_from_string(String::from("12345"), true));

// Get Sino-Korean Hangeul from a BigInt
assert_eq!("천극", hangeul_from_bigint(pow(BigInt::from(10), 51))),

// Get Hangeul from a math expression (Sino-Korean only)
assert_eq!("일 더하기 일", "hangeul_from_expression("1 + 1");

// Get Hangeul from money (an f64 which gets truncated to 2 places) (Sino-Korean only)
assert_eq!("일 점 일", "hangeul_from_money(1.1);
```

### About Korean Numbers
Korean utilizes two number systems. One uses numbers that are purely Korean, the other uses numbers that are derived from borrowed Chinese characters ([한자](https://en.wikipedia.org/wiki/Hanja)).


---

A confusing aspect is that number groupings are done in terms of 4 zeroes, not 3. 

In English, 100,000 is one hundred thousand: `100 * 1,000`. 
In Korean, it's 십만, or `10 * 10,000`.

10 = 십 (in Sino-Korean)
10,000 = 만.

In English, 1,000,000 is one million. In Korean, it's 백만, or `100 * 10,000`.

100 = 백.
10,000 = 만.

---

After 만, the next unique word for a grouping is 억, at 10^8. You can see all of the groupings this program supports below.

---

Lastly, Korean number words are not spaced except for grouping words like 만, 억, 조, and upwards.

### Pure Korean Numbers
Pure Korean numbers are only used for numbers 1 through 99. They mirror our `twenty`, `thirty`, etc, in that each tens increase has a unique word to memorize.

In real life, it's uncommon to use pure korean numbers over 50 or so.

---

To show the tens place, the place word comes before the number.

1 is 하나, 10 is 열, 20 is 스물 (in Pure Korean).

| Number | Hangeul | Literal |
| :---: | :---: | :---: |
| 1 | 하나 | `one` |
| 10 | 열 | `ten` |
| 11 | 열하나 | `ten one` |
| 20 | 스물 | `twenty ` |
| 21 | 스물하나 | `twenty one` |

1, 2, 3, 4, and 20 can be conjugated as well. todo: add conjugation rules here

### Sino-Korean Numbers
Adding a number before a place word like 십, 백, 천, 만, etc, multiplies it.
Adding a number after adds it.

3 is 삼, 10 is 십 (in Sino-Korean).

| Number | Hangeul | Literal |
| :---: | :---: | :---: |
| 13 | 십삼 | `ten three` |
| 30 | 삼십 | `3 tens` |

---

Sino-Korean number support in this program goes up to 10^51:

| Power | Hangeul Grouping | Hanja |
| :---: | :---: | :---: |
| 10^4  | 만 | 萬 |
| 10^8  | 억 | 億 |
| 10^12 | 조 | 兆 |
| 10^16 | 경 | 京 |
| 10^20 | 해 | 垓 |
| 10^24 | 자 | 秭 |
| 10^28 | 양 | 穰 |
| 10^32 | 구 | 溝 |
| 10^36 | 간 | 澗 |
| 10^40 | 정 | 正 |
| 10^44 | 재 | 載 |
| 10^48 | 극 | 極 |

Wikipedia lists even higher groupings, but at different exponents...

| Power | Hangeul Grouping | Hanja |
| :---: | :---: | :---: |
| 10^52 or 10^56 | 항하사 | 恒河沙 |
| 10^56 or 10^64 | 아승기| 阿僧祇|
| 10^60 or 10^72 | 나유타| 那由他|
| 10^64 or 10^80 | 불가사의| 不可思議|
| 10^68 or 10^88 | 무량대수| 無量大數|

# Math Expressions

The following math expressions are supported. When doing math, Sino-Korean numbers are used.

```
pub enum KoreanMathOp {
    Add,
    Divide,
    Multiply,
    Subtract,
    Pow,
    Fraction,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    Log
}
```

### Further Reading

  0. [Korean Numbers](https://www.howtostudykorean.com/unit1/unit-1-lessons-9-16/unit-1-lesson-10/)
  1. [Big Korean Numbers](https://www.howtostudykorean.com/unit-6/lessons-126-133/lesson130/)
  2. [Wikipedia - Korean Numerals](https://en.wikipedia.org/wiki/Korean_numerals)
  3. [Korean Numbers Reference](https://www.omniglot.com/language/numbers/korean.htm)

### Todo
* add conjugations for pure korean numbers
* Add counter enum and mappings
* Add API fn to get a counter for a kind of term
* Add counter information in README
