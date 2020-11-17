// -- hello.rs --

#[macro_use]
mod utilities;

// --

// these code are been tested.
#[servant::report_interface]
pub trait Foo {
    fn f1(&self, count: i32);
    fn f2(&self);
    fn f3(&mut self, s: String);
}

// --

struct FooEntity;
impl FooEntity {
    const NAME_VALUE: &'static str = "report1";
    const F1_COUNT: i32 = 33;
    const F3_S: &'static str = "sss";
}
impl Foo for FooEntity {
    fn f1(&self, count: i32) {
        assert_eq!(count, Self::F1_COUNT);
        dbg!(count);
    }
    fn f2(&self) {
        dbg!("f2 called");
    }
    fn f3(&mut self, s: String) {
        assert_eq!(s, Self::F3_S);
        dbg!(s);
    }
}

// --

#[test]
fn test_foo_request() {
    show_type!(FooRequest);

    let req = [
        FooRequest::F1 {
            count: FooEntity::F1_COUNT,
        },
        FooRequest::F2 {},
        FooRequest::F3 {
            s: FooEntity::F3_S.to_owned(),
        },
    ];
    req.iter().for_each(|r| match r {
        FooRequest::F1 { count } => {
            assert_eq!(
                std::mem::size_of_val(r),
                std::mem::size_of::<FooRequest>()
            );
            assert_eq!(std::mem::size_of_val(count), std::mem::size_of::<i32>());
            assert_eq!(*count, FooEntity::F1_COUNT);
        }
        FooRequest::F2 {} => {
            assert_eq!(
                std::mem::size_of_val(r),
                std::mem::size_of::<FooRequest>()
            );
            assert!(true);
        }
        FooRequest::F3 { s } => {
            assert_eq!(
                std::mem::size_of_val(r),
                std::mem::size_of::<FooRequest>()
            );
            assert_eq!(s, &FooEntity::F3_S.to_owned());
        }
    });
}

#[test]
fn test_foo_trait() {
    fn t(r: FooEntity) -> Box<dyn Foo> {
        Box::new(r)
    }
    let mut r = t(FooEntity);

    assert_eq!(r.f1(FooEntity::F1_COUNT), ());
    assert_eq!(r.f2(), ());
    assert_eq!(r.f3(FooEntity::F3_S.to_owned()), ());
}

#[test]
fn test_foo_officer() {
    show_type!(FooOfficer<FooEntity>);
    assert_eq!(FooOfficer::<FooEntity>::category(), "Foo");

    let he = FooOfficer::new(FooEntity::NAME_VALUE, FooEntity);
    assert_eq!(
        std::mem::size_of_val(&he),
        std::mem::size_of::<FooOfficer::<FooEntity>>()
    );
}

#[test]
fn test_foo_staff() {
    show_type!(FooStaff);
    assert_eq!(FooStaff::category(), "Foo");
}

#[test]
fn build_02_report() {
    let t = trybuild::TestCases::new();
    t.pass("tests/02_report.rs");
}

// --

fn main() {
    show_type!(FooRequest);
    show_type!(FooStaff);
    show_type!(FooEntity);
    show_type!(FooOfficer<FooEntity>);
}
