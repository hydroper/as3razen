# Flex

## Bindable

The `[Bindable]` meta-data is used to indicate that either a property or all properties of a class trigger a `PropertyChangeEvent` after write. The event is only dispatched if `newValue !== prevValue` (notice the strict `!==` operator).

The `[Bindable]` meta-data may be in one of the forms:

```
[Bindable]
[Bindable("eventName")]
[Bindable(event="eventName")]
```

If the event name is omitted, it defaults to `"propertyChange"`.

A setter may contain the `[Bindable]` meta-data, behaving similiarly as above, indicating that the parent virtual slot contains a specific `Bindable` event name.