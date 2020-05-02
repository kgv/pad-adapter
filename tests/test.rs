use core::fmt::{self, Display, Formatter, Write};
use pad_adapter::PadAdapter;

struct A {
    a: usize,
    b: usize,
}

impl Display for A {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("A {\n")?;
        let mut pad_adapter = PadAdapter::new(f);
        write!(pad_adapter, "a: {},\n", self.a)?;
        write!(pad_adapter, "b: {},\n", self.b)?;
        f.write_str("}")?;
        Ok(())
    }
}

#[test]
fn test() {
    assert_eq!(format!("{}", A { a: 9, b: 9 }), "A {\n    a: 9,\n    b: 9,\n}");
}
