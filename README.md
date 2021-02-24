# YEWI
## UI Components for Yew rust/wasm framework

Collection of UI related components for use with the yew framework.

Stand alone working and runable examples for each component are listed under the [`examples`](examples/README.md) directory.

**NOTE**: _until the next version of yew (`0.18`) is released there is no great way to do optional html attributes.
However, the components here allow for optional `id` which if present on an `html` element can not be empty (`""`) and have to be 
unique. For this reason any component that currently accepts an optional `id` prop will by default if not given a value
generate a `uuid` as the `id` attribute. This will be replaced by optional html attributes once that feature is released
in yew_

### Components
- [`Transition`](src/components/transition/): A basic transition component that tracks transition state and uses callback to emit those states
- [`CSSTransition`](src/components/transition/css_transition.rs): An component utilizing the base `Transition` component that will add css classnames for transition states
- [`Access`](src/components/access.rs): A very simple component to make conditional rendering more declarative
- [`Table`](src/components/table/): A data table that allows filtering/sorting (remotely)
- [`InnerHtml`](src/components/inner_html.rs): A component that can take raw html and render it properly
- [`ErrorMessage`](src/components/messages/error_message.rs): A component to display error messages. 
- [`Form`](src/components/form/README.md) A set of form components including `Form` and `inputs`

#### `form`
See [`components::form` README.md](src/component/form/README.md) for more info.

#### `CSSTransition`
Note that durations given here must match those the transition times in your `css`

##### `Properties`
 - class: `String(Optional)` string of custom css classnames
 - name: `String(Required)` a name for the component. These will be added to all transition classnames.
For example `todo-entering`, `todo-entered`, `todo-exited`
 - duration: `std::time::Duration(Optional)` - Defaults to `0` if supplied this can be used as the time for all states of the transition. It is also used for any transition duration that is not 
passed in as a prop
 - appear: `Option<Duration>` Defaults to `None`/`duration` time for appear transitions. Appear only happens on first render.
 - enter: `Option<Duration>` Defaults to `None`/`duration` time for enter transitions. Enter does not happen on first render.
 - exit: `Option<Duration>` Default to `None`/`duration` time for exit transitions. 

##### Classnames
 - `{name}-appear` ie `todo-appear`
 - `{name}-appearing`
 - `{name}-appeared`
 - `{name}-enter`
 - `{name}-entering`
 - `{name}-entered`
 - `{name}-exit`
 - `{name}-exiting`
 - `{name}-exited`

Each set represents the start value, the value to transition to, and the final value. 
For instance, mount will add `*-appear` then immediately remove `*-appear` and add `*-appearing` for the `duration` of the transition. Then it will finally remove `*-appearing` and add `*-appeared` which will not be removed.

_Examples_

#### CSSTransition
Basic Fade-in

Rust .rs
```
use yewi::components::transition::CSSTransition;
use std::time::Duration;

impl Component for YourComponent {
    view(&self) -> Html {
        html! {
            <CSSTransition
                name="hello"
                appear=Duration::from_millis(800)
                exit=Duration::from_millis(600)
            >
                <p>{ "Watch me transition" }</p>
            </CSSTransition>
        }
    }
}
```
CSS .css
```
.hello-appear { opacity: 0; }

.hello-appearing {
    opacity: 1;
    transition: opacity 800ms ease-in-out;
}

.hello-appeared { opacity: 1; }
```

#### Access
Access takes one prop `access` which is a `boolean` and wraps some `children`.
If `access` prop evaluates to `true` the children are rendered. If `access` evaluates
to `false` children are not renderer.
This can be used to wrap a single navigation element (if user does not have permission). Or to wrap 
entire views. In the below example if a user can only read post then the edit link will not appear. 
What children should or should not be rendered can be as simple or as complex as you want. How you determine
whether the `access` prop is `true` or `false` can be as simple or complex as you need.
This is also nice for hidding things behind `feature` flags for certain users/environments/platforms/etc. 

```
use yewi::components::access::Access;

impl Component for YourComponent {
    view(&self) -> Html {
        html! {
            <nav>
                <Access access=user_can_edit(&some_user)>
                    <a href="/edit-post">{ "Edit Post" }</a>
                </Access>
                <Access access=user_can_read(&some_user)>
                    <a hred="/view-post">{ "View Post" }</a>
                </Access>
            </nav>
        }
    }
}
```

#### Table
Table takes 2 `Properties` a `columns` prop that is a description of each column (header, field, etc) and
a `data` prop which is the data in `JSON` (`Serialize/Deserialize`) format. 2 handler props include
`onfilter` and `onsort`. No actual filtering or sorting happens internally in the `Table` Component. It
is up to the user to filter data based on the `field` and `value` of the filter or sort. Typically this would involve
using the parent component's `ComponentLink` callback with the parent components `Messages` enum.

Basic Usage will look like this. For a full working examples please see [`examples/table`](examples/table)
```
<Table
    columns=columns
    data=data
    onfilter=self.link.callback(AppMsg::FilterData)
    onsort=self.link.callback(AppMsg::SortData)
/>
```
Where columns are a `vec` of [`Column`](src/components/table/columns.rs)
An important field here is the `accessor` field, which is optional. If it is `None`
then it does a simple property access on the data for the `field` property. However,
if you give `accessor` `Some(Accessor::callback)` it hands you the `row` data and allows you
to do something as simple as combining two fields for display. Like `name` using `first_name + last_name`
or anything as complex as return buttons, inputs, tables, other custom components, etc. by returning `html`. 

```
use yewi::components::table::{Columns, Column};

let columns: Columns = vec![
    Column {
        field: String::from("name"),
        header_key: None,
        filter: ColumnFilter::Empty,
        width: CSSWidth::Px(200.0),
        ..Default::default()
    },
    Column {
        field: String::from(""),
        accessor: Some(Accessor::callback(|row| {
            let data: UserData = serde_json::from_value(row).unwrap();
            html! { <button id=data.id>{ "Remove" }</button> }
        })),
        ..Default::default()
    },
];
```

```
let data =
```

`onfilter` prop is optional

```
#[prop_or(Callback::noop())]
pub onfilter: Callback<(String, String)>,
```

```
pub enum Msg {
    FilterData((String, String))
};

let onfilter = self.link.callback(Msg::FilterData);
```

`onsort` prop is also optional. 

```
use yewi::components::table::columns::{Sort};

#[prop_or(Callback::noop())]
pub onsort: Callback<(Sort, String)>
```

```
pub enum Msg {
    SortData((Sort, String))
}
let onsort = self.link.callback(Msg::SortData);
```

#### InnerHtml
This component will render html. One common use-case example for this is rendering from Markdown to Html using something like `comrak.rs`.
It has 3 properties 
- `classes` for a string of css class names
- `element` which is a `Tag` enum to render as the parent. Defaults to Tag::Div
- `html` the html string to render

```
use yewi::components::InnerHtml;

fn view(&self) -> Html {
    html! {
        <InnerHtml
            element=Tag::Section
            classes="dangerouslySetInnerHtml"
            html=r#"
                <h2>Headline</h2>
                <p>Some inner html content</p>
            "#
        />
    }
}
```

#### Messages
##### ErrorMessage
Several examples can be found in [`examples/messages`](examples/messages).
- `error`: an `Option<String>` if `None` then no message is displayed. If `Some(String)` then the message is displayed.
- `dismissible`: `bool` determines whether the message will have a dismiss button and can be dismissed. However, you can also implement this yourself.
- `handle_dismiss`: `Callback<()>` called when `dismissible` message is `dismissed`. Internally it will be removed from display automatically. However, this
callback can allow you to update your Error state/global state, or do other things on dismiss.

```
use yewi::components::messages::ErrorMessage;

view(&self) -> Html {
    html! {
        <ErrorMessage
            error=Some("Error Fetching Resource")
            dismissible=true
        />
    }
}
```

### Non Components
#### Html
`use yewi::html::Tag` currently has a `Tag` module that is a enum with many utility functions for dealing
with valid html elements. Default is `Tag::Div`

Basic usage. You can use the enum variante directly or use the `From` implementation to try and create a new enum.
```
let section = Tag::from("section"); // casing does not matter here;
let header = Tag::Header;
// etc.
```

The `Tag` functions include 
- `as_html_str()`: getting a string representation of the element (all lowercase)
- `is_element(&str) -> bool`: returns a bool is the string (can be any casing) matches the `Tag` 
- `is_tag(Tag) -> bool`: same as `is_element` but compares against another `Tag` variant rather than a string

```
let section = Tag::Section;
section.as_html_str(); // return "section"

section.is_element("SECTION"); // true
section.is_element("div"); // false

section.is_tag(Tag::Section); // true
section.is_tag(Tag::Paragraph); // false
```

#### Styles
Styles will contain css utilities. Current we have `dimension` which is a enum for various dimension related CSS values
such as `px`, `%`, `em`, etc. Currently all enum variants take in an `f32` value. 4 basic operators are implemented for 
each one to do math on the types. `Div`, `Sub`, `Add`, `Mul`. After the value is calculated it will come back as the type that is calling the operations (or as the first type).

`use yewi::styles::CSSDimension`

Functions include
- `as_css_str`: will return a string with the `f32` value and the proper css unit. 
- `as_value`: returns the inner value 

```
let px = CSSDimension::Px(200.0);
let percent = CSSDimension::Percent(50.0);

px.as_css_str(); // returns "200.0px"
percent.as_css_str(); // returns "50.0%"

px.as_value(); // returns 200.0
percent.as_value(); // returns 50.0
```

#### InputType
See [`components::form::InputType`](src/components/form/input_type.rs) for more types.
This is an `enum` with similar methods to `html::Tag` enum, but specific to more types of valid form inputs
for example it includes all normal form type elements (from `html::Tag`) but goes deeper because html
`input` can have different `type`s, including `password`, `email`, `color`, etc.

