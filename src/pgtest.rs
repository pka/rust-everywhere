use postgres::{Connection, SslMode};

pub fn pgquery() -> i32 {
    let conn = Connection::connect("postgres://postgres:Password12!@localhost", SslMode::None).unwrap();
    for row in &conn.query("SELECT 42", &[]).unwrap() {
        let val: i32 = row.get(0);
        return val;
    }
    0
}

#[test]
fn test_pgquery() {
    assert_eq!(pgquery(), 42);
}
