# Indent

Stupid and lacking parser and reformatter that reads from stdin and produces indentent output.

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
