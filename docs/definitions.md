# ymx

YMX is a component parser system, using a declarative language.

## Definitions

1. Every component is a key/value pair item in the main body.

Example:

```yml
comp: Hello World!
```

2. You can call a component from its key (name) and you get its value:

Example: calling `comp` from previous example you get the string `"Hello World!"`.

3. Components are called with properties by passing an `object` as unique argument. To access the properties you use `$<property_name>` (`$<property_name>` matches `\$w+`)

```yml
comp: Hello $who
```

Calling `comp` with `{"who": "World"}` you get `"Hello World"`

4. The `body` property maps to the hole argument.

Example:

```yml
comp: Hello $body
```

Calling `comp` with:
  - `"World"` -> `"Hello World"`
  - `123` -> `"Hello 123"`
  - `["W", "o", "r", "l", "d"]` -> `"Hello ["W", "o", "r", "l", "d"]"`
  - `{"who": "World"}` -> `"Hello {"who": "World"}"`

> You could write

```yml
comp: Hello ${body.who}
```

And call it with `{"who": "World"}` so you get the same result.

5. You can do more complex thigs inside processing contexts `${` and `}`.

Example:

```yml
comp: ${a + b}
```

Calling `comp` with `{"a": 1, "b": 2}` you get `3`

> Because components are just functions you can call them inside `${` and `}`:

Example:

```yml
fibo: "${n < 2 ? 1 : n + fibo(n - 1)}"
main: Math is cool because fibo of 10 is ${fibo({"n":10})}
```

Calling `main` you get `"Math is cool because fibo of 10 is 55"`

> The available commands and code format running inside `${` and `}` depends on the interpreter you choose before hand, on the example above we choose [Boa js](https://boajs.dev/) but you could choose [RustPython](https://rustpython.github.io/) to run Python.
> In fact, the above `$<property_name>` is just a shortcut for `${property_name}`, before the yaml string is parsed all `\$(\w+)` is rewriten to `\${\1}`, where `\1` is the match content inside parentesis.

6. You can automate component calling by using the `from!` or `yx-from` or `From` property.

Example:

```yml
div: <div>$body</div>
comp:
  from!: div
  body: Hello World!
```

When `comp` is called it calls `div` component with `{"body": "Hello World!"}` as argument, which returns `<div>Hello Word!</div>`, then it's returned to the first (`comp`) call.

7. Components can be functions to be called with expected arguments

Example:

```rs
fn sql(args: Props) -> Vec[String] {
  let query = args.query;
  conn.execute(query).unwrap()
}
```

```yml
sql(fn):
  lib: /path/to/lib.dll
  args:
    - query
Users:
  From: sql
  query: SELECT name FROM users
```

Calling `Users` you get the list of users in your database.

8. You can shortcut the call to other components using it's name with the prefix `yx-` or appending `!` at the end or just writing it with first letter in uppercase.

Example: The example of item `6.` can be shortcuted to

```yml
div: <div>$body</div>
comp:
  div!: Hello World!
```

The same result could be obtained with

```yml
div: <div>$body</div>
comp:
  yx-div: null
  body: Hello World!
```

In the first example `div` is being called with `"Hello World!"` as argument, in the second one with `{"body": null, "body": "Hello World!"}`.

Example:

```yml
div: <div class="$class" $action>$body</div>
comp:
  yx-div: Hello World!
  class: bg-red-500
  action: onclick="alert("clicked!")"
```

9. Call to components can be simplified in the caller component name by using the caller betewen brackets `()`.

Example: The last example can be written as

```yml
div: <div>$body</div>
comp(div): Hello World!
```

5. In order to hide implementation details components can have templates by just creating components with the same name but starting with `$`. Template are just a last component call component.

Example: The last example can be rewritten using templates

```yml
$comp: <div>$body</div>
comp: Hello World!
```

10. You can define generic components using RegEx just by starting the name with `~`

> Before calling the component itself a first call is performed to replace `$name` with the component name.

Example:

```yml
~\$(a|b)c: "$name: $value"
ac: foo
bc: bar
app:
  - ac!
  - bc!
```

Here we are defining the template components `$ac` and `$bc`. Calling `app` we get `["$ac: foo", "$bc: bar"]`.

10. Property substitution follow the expected usage of the value in the component

Example:

```yaml
comp: $prop
```

- Calling `comp` with `prop` as 'Hello World', you obtain the string 'Hello World'.
- Calling it with `{"a": 1}`, you get the object `{"a": 1}`
- Calling with sequence, you get a sequence

11. You can merge objects with the key `..`

Example:

```yml
comp:
  a: 1
  ..: $prop
```

Calling `comp` with `prop` as `{"b": 2}`, you get `{"a": 1, "b": 2}`, otherwise you get an object with key `".."` and value as the one from `props`

The same applies for sequences, you just need to pre-append the variable with `..`:

Example:

```yml
comp:
  - 1
  - ..$prop
```

Calling `comp` with `[2,3]` you get `[1, 2, 3]`. The other valid value is string, you get the concatenation of `".."` with the string from `prop`. Otherwise you get an error.
