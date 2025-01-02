# Trigger-Response
* `/response add`
  Opens a modal to add a number of [triggers](##Triggers) and their associated responses.
    ```
  +--------------+
  | Triggers:    |
  | ____________ |
  | ____________ |
  | ____________ |
  |              |
  | Responses:   |
  | ____________ |
  | ____________ |
  | ____________ |
  +--------------+
    ```

## Triggers
One trigger per line. A response is sent if *any* of the triggers match. Can be a simple regex (or substring matching) trigger, or if the line starts with `=`, the line will be parsed as an advanced trigger.

An advanced trigger can have a number of conditions to match for. Each condition can appear one or more times and *every* condition must be satisfied for a trigger to match. Conditions are specified in the format `<condition>(<args>)`.

The following conditions are available:
* `regex(<expression>)`: Is satisfied if the message content satisfies the expression. This bot use [Rust flavoured RegEx](https://docs.rs/regex/latest/regex/#syntax) and the expression must include the surrounding quotes (`"`). Any `"` within the expression itself must be escaped as `\"`. Example: `regex("hi")` is satisfies if the message contains 'hi'.
* `author(<id>)`: Is satisfied if the message is authored by the user specified using `<id>`. Example: `author(12345)`.
* `channel(<id>)`: Same as `author(<id>)` but for the channel the message was sent in.
* `guild(<id>)`: Same as `author(<id>)` but for the guild (server) the message was sent in.
* `rng(<probability>)`: Is satisfied based on RNG with the probability specified which must be between 0 and 1 (0 being never, 1 being always). Example: `rng(0.5)` would have a 50% chance of being satisfied.
* More may be added later.

The following is an example of an advanced trigger: `=regex("(?i)\bhi\b") author(123) guild(567)`. This trigger is matched by a message that satisfies the conditions:
* Contains the whole word 'hi', case insensitive using `(?i)`.
* Is sent by user with ID 123.
* Is sent in the server with ID 567.

## Responses
One response per line. Line breaks within responses may be specified as `\n`. A response may be biased if it starts with a number. Larger bias will result in that response being more frequent. If a bias is not specified, a default bias of `1` is assumed. Example of a list of responses, the numbers represent bias:
```
2 Hello!
2 Hi!
1 How's your day?
```
