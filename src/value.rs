#[derive(Debug)]
pub enum ScalarValue {
    Integer(i64),
    Float(f64),
    Character(char),
}

#[derive(Debug)]
pub enum Array {
    Scalar(ScalarValue),
    Enclosed(Box<Array>),
    Array(Vec<Array>),
}
