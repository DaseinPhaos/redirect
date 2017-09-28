# unpublished
- remove `unsafe` from `Event::get()`.
- impl `Send` and `Sync` for `Event`.
- exclude `ComPtr` field access from crate-level API.
- introduce `ResourceAlignment`, change buffer creation API to reflect it.
- add type traits for heaps
- add type traits for resources
- add some type-safe(should be) heap/buffer types
- `ResourceDesc::buffer` now doesn't take an `alignment` argument
- remove `CommittedResource`
- `Device::create_committed_resource` now returns a `RawResource`

# 0.2.1
- Add a trait `factory::HwndProvider`, to bridge a hwnd provider with the swapchain creation API.
- Add a method `create_swapchain` to create a swapchain directly from a `HwndProvider`.
- Add a `event` module for win32 events.
- Add a `set_event_on` method for `Fence`.

# 0.2.0
- Descriptor types rework.
- Methods of command lists are now trait based.
