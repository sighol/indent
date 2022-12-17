# Indent

Parses the input and tries to create indented output.

## Usage

```shell
$ echo 'items: [Hello{world}, Foo(bar: 3), Bar: {hello:1, world:3}]' | indent
items: [
  Hello{
    world
  },
  Foo(
    bar: 3
  ),
  Bar: {
    hello:1,
    world:3
  }
]
```

### String and unicode support

```shell
$ echo '{unicode: "\u0f04", multiline string: "Multi\nline\nstring\n[]\n"}' | indent
{
  unicode: "༄",
  multiline string: "Multi\nline\nstring\n[]"
}
```
