// -- lib.rs --

//! servant-macro是servant库的辅助库，本库定义了四种类型的接口属性，会自动生成客户端和服务端的代码，方便应用开发。
//!
//! ### 四种类型的接口
//!
//! 分别通过[`invoke_interface`]、[`watch_interface`]、[`report_interface`]和[`notify_interface`]定义。
//!
//! ```rust
//! // 定义invoke类型的接口，每个Server可以有多个invoke类型的接口
//! #[servant::invoke_interface(proxy: "HelloProxy", servant: "HelloServant", persistency: true, callback: false)]
//! pub trait Hello {
//!     fn hello(&self, n: i32) -> String;
//! }
//!
//! // 定义watch类型的接口，每个Server只能有一个watch类型的接口
//! #[servant::watch_interface(proxy: "DogProxy", servant: "DogServant")]
//! fn export_servants(&self) -> Vec<Oid>;
//! pub trait WatchDog {
//!     fn login(&self, name: String, password: String) -> UserCookie;
//!     fn export_report_servants(&self) -> Vec<Oid>;
//!     fn version(&self) -> String;
//! }
//!
//! // 定义report类型的接口，每个Server可以有多个report类型的接口
//! #[servant::report_interface(proxy: "Reporter", servant: "Editor")]
//! fn report(&self, msg: String);
//! pub trait Report {
//!     fn post(&self, email: Vec<String>);
//! }
//!
//! // 定义notify类型的接口，每个Server只能有一个notify类型的接口
//! pub trait Notice {
//! #[servant::notify_interface(proxy: "Notifier", servant: "Receiver")]
//!     fn time(&self, h: u8, m: u8, s: u8);
//!     fn date(&self, y: u16, m: u8, d: u8);
//! }
//! ```
//!
//! ### 接口定义的attributes
//!
//! 在定义接口时，可以使用不同的attributes定制化自动生成的代码。每种接口能使用的attributes是不同的，具体可以参考前面代码中的举例。
//!
//! 有四种attributes可以使用：
//!
//! 1. proxy：类型是&str，定义客户端使用的proxy的名称。缺省生成的名称是在trait name后直接增加Proxy，就像如下代码一样：
//!
//! ```rust
//! let proxy_name = format!("{}Proxy", trait_name);
//! ```
//!
//! 2. servant：类型是&str，定义服务端使用的servant的名称。缺省生成的名称是在trait name后直接增加Servant，就像如下代码一样：
//!
//! ```rust
//! let servant_name = format!("{}Servant", trait_name);
//! ```
//!
//! 3. persistency：类型是Bool，如果是true，支持对象的持久化；如果是false，对象不支持持久化。
//!
//! 4. callback：类型是Bool，如果是true，客户端可以使用异步调用；如果是false，不支持异步调用。
//!
//! ### Cargo.toml文件中可以使用的Features说明
//!
//! * client: 引入客户端的代码。
//!
//! * adapter: 引入服务端的代码。
//!
//! [`invoke_interface`]: attr.invoke_interface.html
//! [`watch_interface`]: attr.watch_interface.html
//! [`report_interface`]: attr.report_interface.html
//! [`notify_interface`]: attr.notify_interface.html

// --

#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;

// --

use proc_macro::TokenStream;

// --

mod parse;
mod utilities;

// --

/// 定义带返回值方法的接口，接口中可以有多个带返回值的方法。
///
/// 在一个服务端中，可以定义多个invoke接口，客户端调用这些接口的方法，请求服务端的服务，服务端
/// 通过方法的返回值，响应客户端的请求。
///
/// 在服务端会生成同名的trait，服务端要实现这个trait，在每个方法中提供服务。只有将每个实现添
/// 加到ServantRegister中，客户端才能请求该实现的服务。
///
/// 在客户端会生成proxy，自动实现了该trait的方法，可以通过Terminal的proxy方
/// 法，生成这个proxy，调用接口的方法，向服务端请求服务。
///
/// # Notice
/// 接口中方法的第一个参数必须是&self或&mut self，因为接口在服务端都是按照对象提供服务的，每个接口
/// 可以有不同的实现类，每个类也可以有不同名字的对象分别提供服务。
///
/// # Example
/// ```
/// #[servant::invoke_interface]
/// pub trait Hello {
///     fn hello(&self, n: i32) -> String;
/// }
/// ```
#[proc_macro_attribute]
pub fn invoke_interface(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attributes = parse_macro_input!(attr as parse::InvokeInterfaceAttributes);
    let trait_context = parse_macro_input!(input as parse::TraitContext);
    trait_context.render_invoke_interface(&attributes)
}

/// 定义watch接口，watch接口实际上就像是一个invoke接口。
///
/// 在一个服务端中，只可以有一个watch接口。这个watch接口向客户端提供本服务端的基本信息，比如，
/// 有那些提供服务的对象，有哪些接收报告的对象。
///
/// 在服务端会生成同名的trait，服务端要实现这个trait，在每个方法中提供服务。只有将每个实现添
/// 加到ServantRegister中，客户端才能请求该实现的服务。
///
/// 在客户端会生成proxy，自动实现了该trait的方法，可以通过Terminal的proxy方
/// 法，生成这个proxy，调用接口的方法，向服务端请求服务。
///
/// 实际应用中，开发者可以定义自己的watch接口。
///
/// # Notice
/// 接口中方法的第一个参数必须是&self或&mut self，因为接口在服务端都是按照对象提供服务的。
///
/// # Example
/// ```
/// // 定义watch类型的接口，每个Server只能有一个watch类型的接口
/// #[servant::watch_interface]
/// pub trait WatchDog {
///     fn export_servants(&self) -> Vec<Oid>;
///     fn export_report_servants(&self) -> Vec<Oid>;
///     fn login(&self, name: String, password: String) -> UserCookie;
///     fn version(&self) -> String;
/// }
/// ```
#[proc_macro_attribute]
pub fn watch_interface(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attributes = parse_macro_input!(attr as parse::WatchInterfaceAttributes);
    let trait_context = parse_macro_input!(input as parse::TraitContext);
    trait_context.render_watch_interface(&attributes)
}

/// 定义不带返回值方法的接口，接口中可以有多个不带返回值的方法。
///
/// report接口是单向的，只能从客户端上报到服务端。
///
/// 在一个服务端中，可以定义多个report接口，客户端调用这些接口的方法，向服务端发送报告，服务端
/// 通在实现接口的类中，使用这些报告信息。
///
/// 在服务端会生成同名的trait，服务端要实现这个trait，在每个方法中接收和处理客户端上报的信息。
/// 只有将每个实现添加到ServantRegister中，才能接收并处理客户端上报的信息。
///
/// 在客户端会生成proxy，自动实现了该trait的方法，可以通过Terminal的proxy方
/// 法，生成这个proxy，调用接口的方法，向服务端请求服务。
///
/// # Notice
/// 接口中方法的第一个参数必须是&self或&mut self，因为接口在服务端都是按照对象提供服务的，每个接口
/// 可以有不同的实现类，每个类也可以有不同名字的对象分别提供服务。
///
/// # Example
/// ```
/// // 定义report类型的接口，每个Server可以有多个report类型的接口
/// #[servant::report_interface]
/// pub trait Report {
///     fn report(&self, msg: String);
///     fn post(&self, email: Vec<String>);
/// }
/// ```
#[proc_macro_attribute]
pub fn report_interface(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attributes = parse_macro_input!(attr as parse::ReportInterfaceAttributes);
    let trait_context = parse_macro_input!(input as parse::TraitContext);
    trait_context.render_report_interface(&attributes)
}

/// 定义notify接口，notify接口中的方法不能有返回值。
///
/// notify接口是单向的，只是用来从服务器向每个连接的客户端发送通知。
///
/// 在一个服务端中，只可以有一个notify接口。这个notify接口向客户端发布信息，比如，状态改变或定
/// 时性的通知信息。客户端在实现notify接口的ServantEntry中，接收并处理服务端的通知信息。
///
/// 在客户端会生成同名的trait，客户端要实现这个trait，在每个方法中接收并处理来自服务端的通知。
/// 只有将该实现添加到Terminal中，客户端才能收到并处理服务器端的通知。
///
/// 在服务端会生成notifier，自动实现了该trait的方法。在服务端调用notifier的方法，向客户端发送通知。
///
/// # Notice
/// 接口中方法的第一个参数必须是&self或&mut self，因为接口在服务端都是按照对象提供服务的。
///
/// # Example
/// ```
/// // 定义notify类型的接口，每个Server只能有一个notify类型的接口
/// #[servant::notify_interface]
/// pub trait Datetime {
///     fn date(&self, y: u16, m: u8, d: u8);
///     fn time(&self, h: u8, m: u8, s: u8);
/// }
/// ```
#[proc_macro_attribute]
pub fn notify_interface(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attributes = parse_macro_input!(attr as parse::NotifyInterfaceAttributes);
    let trait_context = parse_macro_input!(input as parse::TraitContext);
    trait_context.render_notify_interface(&attributes)
}
