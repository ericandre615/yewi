# YEWI
## UI Components for Yew rust/wasm framework

Collection of UI related components for use with the yew framework.

### Components
- `Transition`: A basic transition component that tracks transition state and uses callback to emit those states
- `CSSTransition`: An component utilizing the base `Transition` component that will add css classnames for transition states

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

_Example_
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

