// -- 01_invoke.rs --

#[macro_use]
mod utilities;

// --

// these code are been tested.
#[servant::invoke_interface(
    proxy = "HelloProxy2",
    servant = "HelloServant2",
    callback = false,
    persistency = true
)]
pub trait Hello {
    fn hello(&self, n: i32) -> String;
    fn bye(&self);
}

// --

struct HelloEntity;
impl HelloEntity {
    const N_VALUE: i32 = 88;
    const NAME_VALUE: &'static str = "he1";
}
impl Hello for HelloEntity {
    fn hello(&self, _ctx: Option<servant::Context>, n: i32) -> String {
        assert!(_ctx.is_none());
        return n.to_string();
    }
    fn bye(&self, _ctx: Option<servant::Context>) {
        assert!(_ctx.is_none());
    }
}

// --

#[test]
fn test_hello_request() {
    show_type!(HelloRequest);
    let req = [
        HelloRequest::Hello {
            n: HelloEntity::N_VALUE,
        },
        HelloRequest::Bye {},
    ];
    req.iter().for_each(|r| match r {
        HelloRequest::Hello { n } => {
            assert_eq!(
                std::mem::size_of_val(r),
                std::mem::size_of::<HelloRequest>()
            );
            assert_eq!(std::mem::size_of_val(n), std::mem::size_of::<i32>());
            assert_eq!(*n, HelloEntity::N_VALUE);
        }
        HelloRequest::Bye {} => {
            assert_eq!(
                std::mem::size_of_val(r),
                std::mem::size_of::<HelloRequest>()
            );
            assert!(true);
        }
    });
}

#[test]
fn test_hello_trait() {
    fn get(he: HelloEntity) -> Box<dyn Hello> {
        Box::new(he)
    }
    let h = get(HelloEntity);

    assert_eq!(
        h.hello(None, HelloEntity::N_VALUE),
        HelloEntity::N_VALUE.to_string()
    );
    assert_eq!(h.bye(None), ());
}

#[test]
fn test_hello_servant() {
    show_type!(HelloServant2<HelloEntity>);
    assert_eq!(HelloServant2::<HelloEntity>::category(), "Hello");

    let he = HelloServant2::new(HelloEntity::NAME_VALUE, HelloEntity);
    assert_eq!(
        std::mem::size_of_val(&he),
        std::mem::size_of::<HelloServant2::<HelloEntity>>()
    );
}

#[test]
fn test_hello_proxy() {
    show_type!(HelloProxy2);
    assert_eq!(HelloProxy2::category(), "Hello");
}

#[test]
fn build_01_invoke() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01_invoke.rs");
}

fn main() {
    show_type!(HelloRequest);
    show_type!(HelloProxy2);
    show_type!(HelloEntity);
    show_type!(HelloServant2<HelloEntity>);
}
