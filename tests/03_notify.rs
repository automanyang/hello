// -- hello.rs --

#[macro_use]
mod utilities;

// --

// these code are been tested.
#[servant::notify_interface]
pub trait Bar {
    fn f1(&self, count: i32);
    fn f2(&self, msg: String);
    fn f3(&mut self, count: usize, f: f64, b: Option<bool>, s: Vec<String>);
}

// --

struct BarEntity;
impl BarEntity {
    const F1_COUNT: i32 = 33;
    const F2_MSG: &'static str = "Message3";
    const F3_COUNT: usize = 345;
    const F3_B: bool = true;
    const F3_F64: f64 = 3.845;
}
impl Bar for BarEntity {
    fn f1(&self, count: i32) {
        assert_eq!(count, Self::F1_COUNT);
        dbg!(count);
    }
    fn f2(&self, msg: String) {
        assert_eq!(msg, Self::F2_MSG);
        dbg!(msg);
    }
    fn f3(&mut self, count: usize, f: f64, b: Option<bool>, s: Vec<String>) {
        assert_eq!(count, Self::F3_COUNT);
        assert_eq!(f, Self::F3_F64);
        assert_eq!(b, Some(Self::F3_B));
        assert_eq!(s.len(), 0);
        dbg!(count, f, b, s);
    }
}

// --

#[test]
fn test_bar_request() {
    show_type!(BarRequest);
}

#[test]
fn test_bar_trait() {
    fn get(n: BarEntity) -> Box<dyn Bar> {
        Box::new(n)
    }
    let mut r = get(BarEntity);

    assert_eq!(r.f1(BarEntity::F1_COUNT), ());
    assert_eq!(r.f2(BarEntity::F2_MSG.to_owned()), ());
    assert_eq!(
        r.f3(BarEntity::F3_COUNT, BarEntity::F3_F64, Some(BarEntity::F3_B), Vec::new()),
        ()
    );
}

#[test]
fn test_bar_receiver() {
    show_type!(BarReceiver<BarEntity>);

    let he = BarReceiver::new(BarEntity);
    assert_eq!(
        std::mem::size_of_val(&he),
        std::mem::size_of::<BarReceiver::<BarEntity>>()
    );
}

#[test]
fn test_pusher_report_notifier() {
    show_type!(BarNotifier);
}

#[test]
fn build_02_report() {
    let t = trybuild::TestCases::new();
    t.pass("tests/03_notify.rs");
}

// --

fn main() {
    show_type!(BarRequest);
    show_type!(BarNotifier);
    show_type!(BarEntity);
    show_type!(BarReceiver<BarEntity>);
}
