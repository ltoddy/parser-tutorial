# json parser

**之前看到知乎上有人问，会写Parser， Tokenizer是什么水平，绝大情况下，屁用没有。小部分情况，就看你运气了。**

`json`这种数据格式，应该算是人人皆知的了，其语法规则不必赘述。

所以，我想借助编写一份`json parser`来讲解语法解析。通过实践学习。

--------------------------------------------------------------

简单来说， parser就是个转换器，输入是一个字符串，而输出是一个你自己定义一个数据结构。
对于字符串来说，他有各种各样的符号， 例如字符串r"{ "x": 10, "y": [20]， "z": "some" }",
有左右花括号（一般来说，左括号叫开放括号，右括号叫做闭合括号），有逗号，有分号，有字符串，数字等等。

那么，第一步，我们将一个字符串进行初次解析，讲一个一个的符号，变成我们的数据结构（Token），每个Token
会标识，“它”是什么， 例如：

一个字符串"some"可能会被转换成：
```
Token {
    type: string,
    content: "some"
}
```

对于Rust语言来说，提供了枚举，那么我们就可以借助枚举来定义我们的Token。

*src/token.rs*
```rust
#[derive(Debug)]
pub enum Token {
    Comma,
    Colon,
    BracketOn,
    BracketOff,
    BraceOn,
    BraceOff,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}
```

对于将字符串解析成一些列Token的东西，我们称之为：Tokenizer。

*src/token.rs*
```rust
pub struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
}
```

