# Form Components
## and other utilities

### Components
A collection of input `Components`

#### `Form`
The basic html `form` this also exports `FormState`, `FormItem`, and `FormMethod`.
`FormMethod` is an enum that has the standard form http methods including
    - `FormMethod::Get`
    - `FormMethod::Post`
    - `FormMethod::Dialog`: a special method for closing html `<dialog>`

`FormState` is `struct` with a single field `inputs`, which is a `Vec` of `FormItem`s.

`FormItem` a struct for each input within a `Form`

```rust
pub struct FormItem {
    kind: InputType,
    id: String,
    name: String,
    value: String,
    touched: bool,
    valid: bool,
    checked: bool,
}
```

Notice that `FormItem` has a field `kind` that uses the `InputType` instead of a string like `text`, `checkbox`, `radio`, etc.
However, if you need the `FormItem`s type as a string you can use the conversion methods on `InputType`

`Form` is a bit limited right now, but is planned to be able to handle the form state completely
this includes handle the form state, checking inputs to see if they are `valid`, checking if an input
has been `touched`/ is dirty, etc.

**NOTE**: _many form components will have an optional `label` prop. If this prop is omitted
Then no label is added. However, if you pass a `String` as the `label` it will wrap the input
in an html `<label>` element and use the `id` as the `for` and the `label` `String` will come after the input
to the right. This is opinionated, however, you are free to also use the `Label` component if you need your
forms labels/inputs arranged in a different way (just omit the `label` prop when doing this)_

#### `TextInput`
_uses the `label` functionality_

#### `Checkbox`
_uses the `label` functionality_

#### `Radio`
_uses the `label` functionality_


### Enums
`components::form::InputType` enum contains a special list of input elements (and input types).
For instance the html `input` element can have several types that change how their characteristics
and functionality. Some include `email`, `password`, `color`, `range`, etc.
These are represented here as `InputType::Email`, `InputType::Password`, etc.
It also includes the same ulitiy functions as the `html::Tag` `enum` for comparing `InputTypes`,
converting to and from `String`s and comapring the `enum` `InputTypes` to `String`s, which is
usually what you'll get when dealing with web/browser APIs.
Full list is [here](src/components/form/input_type.rs)

#### Basic Usage

