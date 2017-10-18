# unpublished
- update `bitflags` dependency to 1.0, rename constants accordingly.

# 0.3.0
- remove `unsafe` from `Event::get()`.
- impl `Send` and `Sync` for `Event`.
- exclude `ComPtr` field access from crate-level API.
- introduce `ResourceAlignment`, change buffer creation API to reflect it.
- add type traits for heaps
- add type traits for resources
- add some type-safe(should be) heap/buffer types
- `ResourceDesc::buffer` now doesn't take an `alignment` argument
- remove `CommittedResource`, `PlacedResource`
- `Device::create_committed/placed_resource` now returns a `RawResource`
- add methods for `PlacedBuffer` initialization
- remove some descriptor binding helper from crate-level public interface
- fix a bug in `CsuHeap::create_uav`, it should take a `UavDesc` instead of a `SrvDesc`
- add a `BufferSlice` to represent a structured slice into a buffer
- add `create_cbv/srv/uav/uav_with_counter/vbv/ibv` utility methods to `Buffer`
- add a default option for DsvFlags
- add `Texture` and `Tex2D` traits, and associated safe functions
- add a `texture` module, defining safe `DefaultTex2D`, `DsableTex2D` and `RenderableTex2D`

# 0.2.1
- Add a trait `factory::HwndProvider`, to bridge a hwnd provider with the swapchain creation API.
- Add a method `create_swapchain` to create a swapchain directly from a `HwndProvider`.
- Add a `event` module for win32 events.
- Add a `set_event_on` method for `Fence`.

# 0.2.0
- Descriptor types rework.
- Methods of command lists are now trait based.
