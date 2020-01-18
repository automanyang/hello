
## servant-macro

servant-macro是servant库的辅助库，本库定义了四种类型的接口属性，会自动生成客户端和服务端的代码，方便应用开发。

### 四种类型的接口

分别通过invoke_interface、watch_interface、report_interface和notify_interface定义。

```rust
// 定义invoke类型的接口，可以有多个invoke类型的接口
#[servant::invoke_interface(proxy: "HelloProxy", servant: "HelloServant", persistency: true, callback: false)]
pub trait Hello {
    fn hello(&self, n: i32) -> String;
}

// 定义watch类型的接口，每个Server只能有一个watch类型的接口
#[servant::watch_interface(proxy: "DogProxy", servant: "DogServant")]
pub trait WatchDog {
    fn export_servants(&self) -> Vec<Oid>;
    fn export_report_servants(&self) -> Vec<Oid>;
    fn login(&self, name: String, password: String) -> UserCookie;
    fn version(&self) -> String;
}

// 定义report类型的接口，每个Server可以有多个report类型的接口
#[servant::report_interface(proxy: "Reporter", servant: "Editor")]
pub trait Report {
    fn report(&self, msg: String);
    fn post(&self, email: Vec<String>);
}

// 定义notify类型的接口，每个Server只能有一个notify类型的接口
#[servant::notify_interface(proxy: "Notifier", servant: "Receiver")]
pub trait Notice {
    fn date(&self, y: u16, m: u8, d: u8);
    fn time(&self, h: u8, m: u8, s: u8);
}
```

### 接口定义的attributes

在定义接口时，可以使用不同的attributes定制化自动生成的代码。每种接口能使用的attributes是不同的，具体可以参考前面代码中的举例。

有四种attributes可以使用：

1. proxy：类型是&str，定义客户端使用的proxy的名称。缺省生成的名称是在trait name后直接增加Proxy，就像如下代码一样：
```rust
let proxy_name = format!("{}Proxy", trait_name);
```

2. servant：类型是&str，定义服务端使用的servant的名称。缺省生成的名称是在trait name后直接增加Servant，就像如下代码一样：
```rust
let servant_name = format!("{}Servant", trait_name);
```

3. persistency：类型是Bool，如果是true，支持对象的持久化；如果是false，对象不支持持久化。

4. callback：类型是Bool，如果是true，客户端可以使用异步调用；如果是false，不支持异步调用。

### Cargo.toml文件中可以使用的Features说明

* adapter: 引入服务端的代码。
* terminal: 引入客户端的代码
* invoke: 定义invoke接口，并根据adapter/terminal属性，生成服务端和客户端代码。
* watch: 定义query接口，并根据adapter/terminal属性，生成服务端和客户端代码。
* report: 定义report接口，并根据adapter/terminal属性，生成服务端和客户端代码。
* notify: 定义notify接口，并根据adapter/terminal属性，生成服务端和客户端代码。
