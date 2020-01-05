
servant-macro是servant库的辅助库，本库定义了四种类型的接口属性，会自动生成客户端和服务端的代码，方便应用开发。

# Features说明
### adapter
生成服务端的代码。
### terminal
生成客户端的代码
### invoke
定义invoke接口，并根据adapter/terminal属性，生成服务端和客户端代码。
### query
定义query接口，并根据adapter/terminal属性，生成服务端和客户端代码。
### report
定义report接口，并根据adapter/terminal属性，生成服务端和客户端代码。
### notify
定义notify接口，并根据adapter/terminal属性，生成服务端和客户端代码。