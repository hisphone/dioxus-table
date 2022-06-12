use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rust_decimal::Decimal;
use thiserror::Error;

pub enum Value {
    Bool(Option<bool>),
    I8(Option<i8>),
    I16(Option<i16>),
    I32(Option<i32>),
    I64(Option<i64>),
    I128(Option<i128>),
    U8(Option<u8>),
    U16(Option<u16>),
    U32(Option<u32>),
    U64(Option<u64>),
    U128(Option<u128>),
    F32(Option<f32>),
    F64(Option<f64>),
    String(Option<Box<String>>),
    Decimal(Option<Box<Decimal>>),
    BigDecimal(Option<Box<BigDecimal>>),
    ChronoDate(Option<Box<NaiveDate>>),
    ChronoTime(Option<Box<NaiveTime>>),
    ChronoDateTime(Option<Box<NaiveDateTime>>),
}

#[derive(Debug, Error)]
pub enum ValueTypeErr {
    #[error("fail to convert value to bool")]
    BoolErr,
    #[error("fail to convert value to i8")]
    I8Err,
    #[error("fail to convert value to i16")]
    I16Err,
    #[error("fail to convert value to i32")]
    I32Err,
    #[error("fail to convert value to i64")]
    I64Err,
    #[error("fail to convert value to i128")]
    I128Err,
    #[error("fail to convert value to u8")]
    U8Err,
    #[error("fail to convert value to u16")]
    U16Err,
    #[error("fail to convert value to u32")]
    U32Err,
    #[error("fail to convert value to u64")]
    U64Err,
    #[error("fail to convert value to u128")]
    U128Err,
    #[error("fail to convert value to u128")]
    F32Err,
    #[error("fail to convert value to f32")]
    F64Err,
    // #[error("fail to convert value to f64")]
    // StringErr,
    // #[error("fail to convert value to bool")]
    // DecimalErr,
    // #[error("fail to convert value to bool")]
    // BigDecimalErr,
    // #[error("fail to convert value to bool")]
    // ChronoDateErr,
    // #[error("fail to convert value to bool")]
    // ChronoTimeErr,
    // #[error("fail to convert value to bool")]
    // ChronoDateTimeErr,
}

pub trait Nullable {
    fn null() -> Value;
}

pub trait ValueType {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr>
    where
        Self: Sized;

    fn type_name() -> String;
}

macro_rules! type_to_value {
    ( $type: ty, $name: ident, $errname: ident ) => {
        impl From<$type> for Value {
            fn from(x: $type) -> Value {
                Value::$name(Some(x))
            }
        }

        impl Nullable for $type {
            fn null() -> Value {
                Value::$name(None)
            }
        }

        impl ValueType for $type {
            fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
                match v {
                    Value::$name(Some(x)) => Ok(x),
                    _ => Err(ValueTypeErr::$errname),
                }
            }

            fn type_name() -> String {
                stringify!($type).to_owned()
            }
        }
    };
}

type_to_value!(bool, Bool, BoolErr);
type_to_value!(i8, I8, I8Err);
type_to_value!(i16, I16, I16Err);
type_to_value!(i32, I32, I32Err);
type_to_value!(i64, I64, I64Err);
type_to_value!(i128, I128, I128Err);
type_to_value!(u8, U8, U8Err);
type_to_value!(u16, U16, U16Err);
type_to_value!(u32, U32, U32Err);
type_to_value!(u64, U64, U64Err);
type_to_value!(u128, U128, U128Err);
type_to_value!(f32, F32, F32Err);
type_to_value!(f64, F64, F64Err);
