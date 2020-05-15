# json parser

*之前看到知乎上有人问，会写`Parser`， `Tokenizer`是什么水平，绝大情况下，屁用没有。小部分情况，就看你运气了。因为这东西，面试又不会加分，而且，如果你面试的小公司，可能面试官甚至都不懂你在说啥。*

`json`这种数据格式，应该算是人人皆知的了，其语法规则不必赘述。

我想借助编写一份`json parser`来讲解语法解析，通过实践来学习。

--------------------------------------------------------------

简单来说，parser就是个转换器，输入是一个字符串，而输出是一个你自己定义一个数据结构。
对于字符串来说，他有各种各样的符号， 例如字符串`r"{ "x": 10, "y": [20]， "z": "some" }"`,
有左右花括号（一般来说，左括号叫开放括号，右括号叫做闭合括号），有逗号，有分号，有字符串，数字等等。

对于`JSON`,我们需要实现两个方法:

- 用于解析JSON的 `parse()` 方法.
- 以及将对象/值转换为`JSON`字符串的`stringify()`方法。

### 第一步，编写`Tokenizer`!

我们将一个字符串进行初次解析，将一个一个的符号，变成我们的数据结构（Token），每个`Token`会标识，“它”是什么， 例如：

一个字符串`"some"`可能会被转换成：
```
Token {
    type: string,
    content: "some"
}
```

对于Rust语言来说，提供了枚举，那么我们就可以借助枚举来定义我们的`Token`。

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

上述`Token`枚举就包含了`JSON`字符串里所有出现‘符号’的种类：逗号，分号，左方括号，右方括号，左花括号，右花括号，字符串，数字，布尔，和null。

对于将字符串解析成一系列`Token`的东西，我们称之为：`Tokenizer`。

*src/tokenizer.rs*
```rust
pub struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
}
```

对于`Tokenizer`，我们希望它能够有一个方法：`next`，每次调用这个方法，会返回
给我们一个`Token`，当没有Token返回的时候，则表示输入的字符串已经全部解析完。

很幸运，Rust的内置接口里面（`trait`我一般称作特征，这里写作了接口，这样子大众也更容易方便理解。），含有这么一个接口：

`Iterator`接口。

*src/tokenizer.rs*
```rust
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        'lex: while let Some(ch) = self.source.next() {
            return Some(match ch {
                ',' => Token::Comma,
                ':' => Token::Colon,
                '[' => Token::BracketOn,
                ']' => Token::BracketOff,
                '{' => Token::BraceOn,
                '}' => Token::BraceOff,
                '"' => Token::String(self.read_string(ch)),
                '0'..='9' => Token::Number(self.read_number(ch)),
                'a'..='z' => {
                    let label = self.read_symbol(ch);
                    match label.as_ref() {
                        "true" => Token::Boolean(true),
                        "false" => Token::Boolean(false),
                        "null" => Token::Null,
                        _ => panic!("Invalid label: {}", label),
                    }
                }
                _ => {
                    if ch.is_whitespace() {
                        continue 'lex;
                    } else {
                        panic!("Invalid character: {}", ch);
                    }
                }
            });
        }

        None
    }
}
```

我们一个一个读入字符串的字符，判断他归属于哪一个类型（token type），

从上面代码里看,对于那些符号的判断,最为简单,直接返回它对应的Token就可以了.
对于字符串,数字,符号(null, true, false),就稍微难一点判断了.

对于字符串，它的样子就像`"this is a string"`，由一对双引号包围，更复杂一些的字符串，其含有转义字符：
`"This is a string\\n"`.

对于解析字符串，当我们首次遇到双引号字符时，我们判定，其随后的内容是一个字符串，当第二次遇到双引号的时候，我们判断，其字符串结束。

当遇到转移字符`\`的时候，我们所需要做的就是忽略第一个`\`，将之后的字符保存。

对于其他的字符，仅仅是遍历一遍保存便可。

*src/tokenizer.rs*

```rust
impl<'a> Tokenizer<'a> {
    fn read_string(&mut self, first: char) -> String {
        let mut value = String::new();
        let mut escape = false;
    
        while let Some(ch) = self.source.next() {
            if ch == first && escape == false {
                return value;
            }
            match ch {
                '\\' => {
                    if escape {
                        escape = false;
                        value.push(ch);
                    } else {
                        escape = true;
                    }
                }
                _ => {
                    value.push(ch);
                    escape = false;
                }
            }
        }
    
        value
    }
}
```

对于数字，其特征为数字开头，随后为数字，其中也可能包含一个小数点。所以，只要它是一数字开头，
我们便可判断它及其后面的字符串是一个完整的数字。并且，有且只可能有0个或1个小数点。

*src/tokenizer.rs*

```rust
impl<'a> Tokenizer<'a> {
    fn read_number(&mut self, first: char) -> f64 {
        let mut value = first.to_string();
        let mut point = false;
    
        while let Some(&ch) = self.source.peek() {
            match ch {
                '0'..='9' => {
                    value.push(ch);
                    self.source.next();
                }
                '.' => {
                    if !point {
                        point = true;
                        value.push(ch);
                        self.source.next();
                    } else {
                        return value.parse::<f64>().unwrap();
                    }
                }
                _ => return value.parse::<f64>().unwrap(),
            }
        }
    
        value.parse::<f64>().unwrap()
    }
}
```

对于符号`null`, `true`, `false`, `{ "is_symbol": true }`比如这样子的json字符串。
它们不像字符串，由两个双引号包围，它们只是由单纯的英文小写字母组成。

*src/tokenizer.rs*

```rust
impl<'a> Tokenizer<'a> {
    fn read_symbol(&mut self, first: char) -> String {
        let mut symbol = first.to_string();
    
        while let Some(&ch) = self.source.peek() {
            match ch {
                'a'..='z' => {
                    symbol.push(ch);
                    self.source.next();
                }
                _ => break, // 遇到非英文小写字母，判定它结束
            }
        }
    
        symbol
    }
}
```

等到这里,如果实现了`Tokenizer`,让他能够不断地给我们解析`Token`,基本上第一个难点就算结束了。
因为,当我们把输入的字符串一个一个的解析成了一系列`Token`之后,剩下的很大一部分就是天高任鸟飞
的时候,为什么?很简单,`Token`也是我们自己定义的数据结构,而且它在内存中,我们想怎么用它就可以
怎么用它.

### 第二步，编写`Parser`!

下面,我们将我们一系列的`Token`解析成我们的`JSON`.

*src/value.rs*

```rust
pub enum Json {
    Null,
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}
```

如果你不清楚Json, 你可以看下: [Introducing JSON](https://www.json.org/json-zh.html)

Parser接受字符串,借助我们刚才编写的Tokenizer, 然输出抽样语法树(一般来说,Parser接受字符串,然后输出抽象语法书,不过,管他呢,我们能实现我们想要实现的便可,管它具体的定义呢),
对于我们的Json Parser,输出的就是我们刚才定义的`Json`结构.

*src/parser*

```rust
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}
```

这就是我们`Parser`的定义，它内含一个`Tokenizer`，要借助它生成的`Toekn`去变成`Json`。

*src/parser*

```rust
impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Json {
        let token = self.step();

        self.parse_from(token)
    }

    fn step(&mut self) -> Token {
        self.tokenizer.next().expect("Unexpected end of JSON!!!")
    }

    fn parse_from(&mut self, token: Token) -> Json {
        match token {
            Token::Null => Json::Null,
            Token::String(s) => Json::String(s),
            Token::Number(n) => Json::Number(n),
            Token::Boolean(b) => Json::Boolean(b),
            Token::BracketOn => self.parse_array(),
            Token::BraceOn => self.parse_object(),
            _ => panic!("Unexpected token: {:?}", token),
        }
    }
}
```

以上代码就是`Parser`的核心了，其运行原理与`Tokenizer`相仿。

`Json`中的数据结构：`boolean`，`string`，`null`，以及`array`（以左方括号开头，右方括号结尾），`object`（以左花括号开头，右花括号结尾）。

借助于`Tokenizer`生成`Token`，进行模式匹配，当遇到`Token::Null`，`Token::String(s)`，`Token::Number(n)`，`Token::Boolean(b)`，
这些时，直接返回就可以了，毕竟我们已经在实现`Tokenizer`的时候处理过了。

当遇到`Token::BracketOn`，左括号！这是`array`开始的符号，那么我们交给`self.parse_array()`处理：

*src/parser*

```rust
impl<'a> Parser<'a> {
    fn parse_array(&mut self) -> Json {
        let mut array = Vec::new();

        match self.step() {
            Token::BracketOff => return array.into(),
            token => array.push(self.parse_from(token)),
        }

        loop {
            match self.step() {
                Token::Comma => array.push(self.parse()),
                Token::BracketOff => break,
                token => panic!("Unexpected token {:?}", token),
            }
        }

        array.into()
    }
}
```

如上使我们如何处理`array`，当遇到右方括号的时候，表明`array`结束了，返回即可。
对于我们`array`类型，其每一个元素都可以为`Json`，并且，元素之间用逗号分割，
那么当遇到逗号`Token：：Comma`的时候，就可以断定一个新的元素出现。
