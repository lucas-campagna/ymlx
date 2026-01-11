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

3. Components can be called with arguments, the result depends on the **component** and **argument** types. The following rules are applied in order:

    1. **argument** is **null**, then **component** value is returned
    2. **component** has only one valid variable, the result is the application of **argument** into this **component**
    3. **argument** is **object** and **component** has any valid and common variable with the **arguments**, the result is the replacement of **argument** variables into **component** variables
    4. **argument** and **component** are **object**s, the result is the result of assignment of **argument** into **component**
    5. **component** and **argument** are **sequence**s, the result is the concatenation of **component** with **argument**
    6. **component** or **argument** is **sequence**, the result is a call to each **sequence** item using the other non-**sequence** counterpart as **component** or **argument**
    7. **component** contains `_` variable, then the **return** is the pasting of **argument** into all occourences of it or, if it's an `object`, only the remaining variables from step `4.` will be pasted
    8. Otherwise **component** is returned

> A variable is any word starting with dollar sign (`$`) and may contain `_`.
> After replacing variables all unused variables are discarted into `_` the variable (if present).

Example: consider the following components

```yml
comp1: Hello, $name
comp2:
  - a
  - b
  - c
comp3:
  - Hello, $name
  - full_name: $name $last
    year: 2026
  - null
  - 123
comp4:
  a: 1
  b: 2
  c: 3
comp5:
  - $_
  - a: $_
  - The ticket is $_
comp6:
  id: $name
  props: $_
```

- Calling **component** `comp1` with **argument** `"Alice"}` you get `"Hello, Alice"`.
- Calling **component** `comp1` with **argument** `{"name": "Alice"}` you get `"Hello, Alice"`.
- Calling **component** `comp2` with **argument** `["d", "e"]` you get `["a", "b", "c", "d", "e"]`.
- Calling **component** `comp3` with **argument** `{"name": "Bob", "last": "Rock"}` you get `["Hello, Bob", {"full_name": "Bob Rock", "year": 2026}, null, 123]`.
- Calling **component** `comp4` with **argument** `$a + $b = $c` you get `"1 + 2 = 3"`.
- Calling **component** `comp4` with **argument** `{"d": 4, "e": 5}` you get `{"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}`.
- Calling **component** `comp5` with **argument** `123` you get `[123, {"a": 123}, "The ticket is 123"]`.
- Calling **component** `comp6` with **argument** `{"name": "Alice", "age": 123, "role": "president"}` you get `{"id": "Alice", "props": {"age": 123, "role": "president"}}`.

3. You can automate component calling by using the `yx-from` property.

Example:

```yml
div: <div>$body</div>
comp:
  yx-from: div
  body: Hello World!
```

When `comp` is called it calls `div` component with `{"body": "Hello World!"}` as argument, which returns `<div>Hello Word!</div>`, then it's returned to the first (`comp`) call.

> Intead of `yx-from` you may prefer use `from!` or `From`.

4. Components can be functions to be called with expected arguments

Example:

```rs
fn sql(args: Props) -> Vec[String] {
  let query = args.query;
  conn.execute(query).unwrap()
}
```

```yml
users:
  yx-from: sql
  query: SELECT name FROM users
```

Calling `users` you get the list of users in your database.

5. You can shortcut the call to other components using it's name with the prefix `yx-`.

Example: The example of item `3.` can be shortcuted to

```yml
div: <div>$body</div>
comp:
  yx-div: Hello World!
```

The same result could be obtained with

```yml
div: <div>$body</div>
comp:
  yx-div: null
  body: Hello World!
```

In the first example `div` is being called with `"Hello World!"` as argument, in the second one with `{"default": null, "body": "Hello World!"}`.

Example:

```yml
div: <div class="$class" $action>$body</div>
comp:
  yx-div: Hello World!
  class: bg-red-500
  action: onclick="alert("clicked!")"
```

Because, `body` is the remaining variable it's deducted to be `"Hello World!"`, the default variable value.


5. Components can have templates by just creating components with the same name but starting with `$`. This is useful to hide call details on the template leting the interface on the component. The component calls the template after all calls.

Example: The last example can be rewritten using templates

```yml
$comp: <div>$body</div>
comp: Hello World!
```

6. Call to components can be simplified in the callee component name by using the caller betewen brackets `()`.

Example: The last example can be written as

```yml
div: <div>$body</div>
comp(div): Hello World!
```

8. The last way to call components using `yx-` and the component name in strings with only this content. The call is made with `null` as **argument**

Example:

```yml
div: <div>$body</div>
comp(div): Hello World!
app: yx-comp
```

Calling the `app` component the `comp` is call with `null` arg (just a copy and paste).

9. You can eval math in strings using `$()`

Example:

```yml
app: cossine of 60 deg is $(cos(60*pi/180))
```

Calling `app` you get `cossine of 60 deg is 0.5`

8. You can define generic components using RegEx just by starting the name with `~`

> Before calling the component itself a first call is performed to replace `$name` with the component name.

Example:

```yml
~\$(a|b)c: "$name: $value"
ac: foo
bc: bar
app:
  - yx-ac
  - yx-bc
```

Here we are defining the template components `$ac` and `$bc`. Calling `app` we get `["$ac: foo", "$bc: bar"]`.