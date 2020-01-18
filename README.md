
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
pub trait Datetime {
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

* watch: 定义watch接口，并根据adapter/terminal属性，生成服务端和客户端代码。

* report: 定义report接口，并根据adapter/terminal属性，生成服务端和客户端代码。

* notify: 定义notify接口，并根据adapter/terminal属性，生成服务端和客户端代码。

## 深入细节

在此，我们讨论下自动生成的代码，帮助理解其中的机制。

### invoke_interface生成的代码

对于如下的定义：

```rust
#[servant::invoke_interface]
pub trait Hello {
    fn hello(&self, n: i32) -> String;
    fn bye(&self);
}
```

自动生成的代码有三部分：

1. 在客户端和服务端共用的代码：

```rust
#[derive(serde::Serialize, serde::Deserialize)]
enum HelloRequest {
    Hello { n: i32 },
    Bye { },
}
```

2. 在服务端使用的代码：

```rust
pub trait Hello {
    fn hello(&self, ctx: Option<servant::Context>, n: i32) -> String;
    fn bye(&self, ctx: Option<servant::Context>);
}

pub struct HelloServant<S> {
    name: String,
    entity: S,
}
impl<S> HelloServant<S> {
    pub fn new(name: &str, entity: S) -> Self {
        Self { name: name.to_string(), entity }
    }
    pub fn category() -> &'static str {
        "Hello"
    }
}

// if persistency == true {
// 生成如下代码：
impl<S> servant::Servant for HelloServant<S>
where
    S: serde::Serialize + Hello + 'static,
{
    fn name(&self) -> &str {
        &self.name
    }
    fn dump(&self) -> servant::ServantResult<Vec<u8>> {
        bincode::serialize(&self.entity).map_err(|e| e.to_string().into())
    }
    fn serve(&mut self, ctx: Option<servant::Context>, req: Vec<u8>) -> Vec<u8> {
        let req: HelloRequest = bincode::deserialize(&req).unwrap();
        let reps = match req {
            HelloRequest::Hello { n } =>
                bincode::serialize(&self.entity.hello(ctx, n)),
            HelloRequest::Bye { } =>
                bincode::serialize(&self.entity.bye(ctx)),
        }
        .unwrap();
        reps
    }
}
// } else {
// persistency是false，生成如下代码：
impl<S> servant::Servant for HelloServant<S>
where
    S: Hello + 'static,
{
    fn name(&self) -> &str {
        &self.name
    }
    fn serve(&mut self, ctx: Option<servant::Context>, req: Vec<u8>) -> Vec<u8> {
        let req: HelloRequest = bincode::deserialize(&req).unwrap();
        let reps = match req {
            HelloRequest::Hello { n } =>
                bincode::serialize(&self.entity.hello(ctx, n)),
            HelloRequest::Bye { } =>
                bincode::serialize(&self.entity.bye(ctx)),
        }
        .unwrap();
        reps
    }
}
// }
```

3. 在客户端使用的代码：

```rust
#[derive(Clone)]
pub struct HelloProxy(servant::Context, servant::Oid, servant::Terminal);

impl HelloProxy {
    pub fn new(ctx: servant::Context, name: &str, t: &servant::Terminal) -> Self {
        let oid = servant::Oid::new(name, Self::category());
        Self(ctx, oid, t.clone())
    }
    pub fn category() -> &'static str {
        "Hello"
    }
    pub fn hello(&self, n: i32) -> String {
        let request = HelloRequest::Hello { n };
        let response = self
            .2
            .invoke(Some(self.0.clone()), Some(self.1.clone()), bincode::serialize(&request).unwrap())
            .await;
        response.map(|x| bincode::deserialize(&x).unwrap())
    }
    fn bye(&self) {
        let request = HelloRequest::Bye { };
        let response = self
            .2
            .invoke(Some(self.0.clone()), Some(self.1.clone()), bincode::serialize(&request).unwrap())
            .await;
        response.map(|x| bincode::deserialize(&x).unwrap())
    }

    // if callback == true {
    // 生成如下代码：
    pub fn hello_with_callback(&self, n: i32, __f_20101008__: F) -> servant::ServantResult<()>
        where F: 'static + Fn(servant::ServantResult<String>) + Send,
    {
        let request = HelloRequest::Hello { n };
        self.2
            .invoke_with_callback(Some(self.0.clone()), Some(self.1.clone()),
                bincode::serialize(&request).unwrap(), move |oid, v| {
                    __f_20101008__(v.map(|x| bincode::deserialize(&x).unwrap()));
                })
            .await
    }
    fn bye_with_callback(&self, __f_20101008__: F) -> servant::ServantResult<()>
        where F: 'static + Fn(servant::ServantResult<()>) + Send,
    {
        let request = HelloRequest::Bye { };
        self.2
            .invoke_with_callback(Some(self.0.clone()), Some(self.1.clone()),
                bincode::serialize(&request).unwrap(), move |oid, v| {
                    __f_20101008__(v.map(|x| bincode::deserialize(&x).unwrap()));
                })
            .await
    }
    // }
}
```

如上的代码是比较清楚的，请注意其中persistency、callback attributes不同，会生成不同的代码。