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

对于Tokenizer，我们希望它能够有一个方法：next，每次调用这个方法，会返回
给我们一个Token，当没有Token返回的时候，则表示输入的字符串已经全部解析完。

很幸运，Rust的内置接口里面（trait我一般称作特征，这里写作了接口，这样子大众也更容易方便理解。）。

`Iterator`接口。

*src/token.rs*
```rust
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        'lex: while let Some(ch) = self.source.next() {
            return Some(match ch {
                ',' => Token::Comma,
                ':' => Token::Colon,
                '[' => Token::BraceOn,
                ']' => Token::BraceOff,
                '{' => Token::BracketOn,
                '}' => Token::BracketOff,
                '"' => Token::String(self.read_string(ch)),
                '0'..='9' => Token::Number(self.read_number(ch)),
                'a'..='z' | 'A'..='Z' => {
                    let label = self.read_label(ch);
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

对于符号(null, true, false)这样子的, `{ "is_symbol": true }`比如这样子的json字符串.
`true`不向字符串,会有双引号包围,那么对于判断符号,它只要是字母符号开头就可以了.当遇到其他
字符(空格),便可以视作符号结束.

*src/token.rs*

```rust
fn read_symbol(&mut self, first: char) -> String {
    let mut symbol = first.to_string();

    while let Some(&ch) = self.source.peek() {
        match ch {
            'a'..='z' => {
                symbol.push(ch);
                self.source.next();
            }
            _ => break,
        }
    }

    symbol
}
```


对于数字,包含十个数字以及小数点,那么进行数字的判断,只要他以数字开头便可.
并且,有且只可能有0个或1个小数点.

*src/token.rs*

```rust
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
```

对于字符串来说,由一对双引号包围,并且,字符串中可能会出现转义字符.

*src/token.rs*

```rust
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
```

等到这里,如果实现了Tokenizer,让他能够不断地给我们解析Token,基本上第一个难点就算结束了.
因为,当我们把输入的字符串一个一个的解析成了一系列Token之后,剩下的很大一部分就是天高任鸟飞
的时候,为什么?很简单,Token也是我们自己定义的数据结构,而且它在内存中,我们想怎么用它就可以
怎么用它.

下面,我们将我们一些列的Tokens解析成我们的Json.

*src/value*

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

如果你不清楚Json, 你可以看下: https://www.json.org/json-zh.html

