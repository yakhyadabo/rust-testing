pub struct Calculator {
    op1: i32,
    op2: i32,
}

impl Calculator {
    pub(crate) fn new (op1: i32, op2: i32) -> Self {
        Self { op1, op2 }
    }

    pub(crate) fn sum(&self) -> i32 {
        self.op1 + self.op2
    }

    // pub fn double_value(&self) -> i32 {
    //     self.value * 2
    // }

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
