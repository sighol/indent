# Indent

Stupid and lacking parser and reformatter that reads from stdin and produces indentent output.

## Usage

```shell
$ echo "items: [Hello{world}, Foo(bar: 3), Fizz: {hello:1, world:3}]" | indent
items: [
  Hello{
    world
  },
  Foo(
    bar: 3
  ),
  Fizz: {
    hello:1,
    world:3
  }
]
```

## TODO

- Make it understand strings so that it doesn't indent inside strings.
- Right now it replaces "\n" with "". It should probably not do that.
- RIght now it replace ",\s+" with ",". It should probably not do that either.