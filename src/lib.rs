// -- lib.rs --

//! servant-macro是servant库的辅助库，本库定义了四种类型的接口属性，会自动生成客户端和服务
//! 端的代码，方便应用开发。
//!
//! # Features说明
//!
//! ### adapter
//!
//! 生成服务端的代码。
//!
//! ### terminal
//!
//! 生成客户端的代码
//!
//! ### invoke
//!
//! 定义invoke接口，并根据adapter/terminal属性，生成服务端和客户端代码。
//!
//! ### watch
//!
//! 定义query接口，并根据adapter/terminal属性，生成服务端和客户端代码。
//!
//! ### report
//!
//! 定义report接口，并根据adapter/terminal属性，生成服务端和客户端代码。
//!
//! ### notify
//!
//! 定义notify接口，并根据adapter/terminal属性，生成服务端和客户端代码。

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
/// 在一个应用中，可以定义多个invoke接口，客户端调用这些接口的方法，请求服务端的服务，服务端
/// 通过方法的返回值，响应客户端的请求。
///
/// 在服务端会生成同名的trait，服务端要实现这个trait，在每个方法中提供服务。只有将每个实现添
/// 加到ServantRegister中，客户端才能请求该实现的服务。
///
/// 在客户端会生成后缀Proxy的struct，自动实现了该trait的方法，可以通过Terminal的proxy方
/// 法，生成这个proxy，调用接口的方法，向服务端请求服务。
///
/// # Notice
/// 接口中方法的第一个参数必须是&self或&mut self，因为接口在服务端都是按照对象提供服务的，每个接口
/// 可以有不同的实现类，每个类也可以有不同名字的对象分别提供服务。
///
/// # Example
/// ```
/// #[servant::invoke_interface]
/// pub trait Dog: Clone {
///     fn speak(&self, count: i32) -> String;
///     fn owner(&self) -> servant::Oid;
///     fn age(&mut self, i: u32) -> u32;
/// }
/// ```
#[cfg(feature = "invoke")]
#[proc_macro_attribute]
pub fn invoke_interface(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attributes = parse_macro_input!(attr as parse::InvokeInterfaceAttributes);
    let trait_context = parse_macro_input!(input as parse::TraitContext);
    trait_context.render_invoke_interface(&attributes)
}

/// 定义query接口，query接口实际上就是一个invoke接口。
///
/// 在一个应用中，只可以有一个query接口。这个query接口向客户端提供本服务端的基本信息，比如，
/// 有那些提供服务的对象，有哪些接收报告的对象。
///
/// 在服务端会生成同名的trait，服务端要实现这个trait，在每个方法中提供服务。只有将每个实现添
/// 加到ServantRegister中，客户端才能请求该实现的服务。
///
/// 在客户端会生成后缀Proxy的struct，自动实现了该trait的方法，可以通过Terminal的proxy方
/// 法，生成这个proxy，调用接口的方法，向服务端请求服务。
///
/// 在servant中，缺省实现的query接口是Export，方便客户端查询服务端的信息。
///
/// 实际应用中，开发者可以定义自己的query接口。
///
/// # Notice
/// 接口中方法的第一个参数必须是&self或&mut self，因为接口在服务端都是按照对象提供服务的。
///
/// # Example
/// ```
/// #[servant_macro::watch_interface]
/// pub trait Export {
///     fn export_servants(&self) -> Vec<Oid>;
///     fn export_report_servants(&self) -> Vec<Oid>;
///     fn shutdown(&self, passcode: usize);
/// }
/// ```
#[cfg(feature = "watch")]
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
/// 在一个应用中，可以定义多个report接口，客户端调用这些接口的方法，向服务端发送报告，服务端
/// 通在实现接口的类中，使用这些报告信息。
///
/// 在服务端会生成同名的trait，服务端要实现这个trait，在每个方法中接收和处理客户端上报的信息。
/// 只有将每个实现添加到ServantRegister中，才能接收并处理客户端上报的信息。
///
/// 在客户端会生成后缀Proxy的struct，自动实现了该trait的方法，可以通过Terminal的proxy方
/// 法，生成这个proxy，调用接口的方法，向服务端请求服务。
///
/// # Notice
/// 接口中方法的第一个参数必须是&self或&mut self，因为接口在服务端都是按照对象提供服务的，每个接口
/// 可以有不同的实现类，每个类也可以有不同名字的对象分别提供服务。
///
/// # Example
/// ```
/// #[servant::report_interface]
/// pub trait Pusher {
///     fn f1(&self, count: i32);
///     fn f2(&self);
///     fn f3(&mut self, s: String);
/// }
/// ```
#[cfg(feature = "report")]
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
/// 在一个应用中，只可以有一个notify接口。这个notify接口向客户端发布信息，比如，状态改变或定
/// 时性的通知信息。客户端在实现notify接口的ServantEntry中，接收并处理服务端的通知信息。
///
/// 在客户端会生成同名的trait，客户端要实现这个trait，在每个方法中接收并处理来自服务端的通知。
/// 只有将该实现添加到Terminal中，客户端才能收到并处理服务器端的通知。
///
/// 在服务端会生成后缀Notifier的struct，自动实现了该trait的方法。在服务端调用该struct的
/// instance()关联方法，得到该struct的一个引用，调用该引用的接口，向客户端发送通知。
///
/// # Notice
/// 接口中方法的第一个参数必须是&self或&mut self，因为接口在服务端都是按照对象提供服务的。
///
/// # Example
/// ```
/// #[servant::notify_interface]
/// pub trait StockNews {
///     fn f1(&self, count: i32);
///     fn f2(&self, msg: String);
///     fn f3(&mut self, count: usize, f: f64, b: Option<bool>, s: Vec<String>);
/// }
/// ```
#[cfg(feature = "notify")]
#[proc_macro_attribute]
pub fn notify_interface(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attributes = parse_macro_input!(attr as parse::NotifyInterfaceAttributes);
    let trait_context = parse_macro_input!(input as parse::TraitContext);
    trait_context.render_notify_interface(&attributes)
}
