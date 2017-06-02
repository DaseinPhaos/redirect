// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! resource barriers

use super::{PlacedResource, ResourceStates, RawResource};

/// resource barriers
#[derive(Copy, Clone, Debug)]
pub struct ResourceBarrierBuilder{
    pub flags: ResourceBarrierFlags,
    pub barrier_type: ResourceBarrierType,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ResourceBarrierBindHelper {
    pub barrier_type: ::winapi::D3D12_RESOURCE_BARRIER_TYPE,
    pub flags: ResourceBarrierFlags,

}

#[derive(Copy, Clone, Debug)]
pub enum ResourceBarrierType {
    Transition(ResourceTransitionBarrier),
    Aliasing(ResourceAliasingBarrier),
    Uav(ResourceUavBarrier),
}

/// represents a transition of subresource between different usages
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ResourceTransitionBarrier {
    resource: *mut ::winapi::ID3D12Resource,
    subresource: u32,
    before: ResourceStates,
    after: ResourceStates,
}

impl ResourceTransitionBarrier {
    #[inline]
    pub fn new(
        resource: &RawResource, subresource: u32, 
        before: ResourceStates, after: ResourceStates
    ) -> ResourceTransitionBarrier {
        ResourceTransitionBarrier{
            resource: resource.ptr.as_mut_ptr(), 
            subresource, before, after,
        }
    }
}

/// describes the transition between usage of two different
/// resources having mappings into the same heap.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ResourceAliasingBarrier {
    before: *mut ::winapi::ID3D12Resource,
    after: *mut ::winapi::ID3D12Resource,
}

impl ResourceAliasingBarrier {
    #[inline]
    pub fn new(before: &mut PlacedResource, after: &mut PlacedResource) -> Self {
        debug_assert_eq!(
            before.get_placed_heap().ptr.as_ptr(),
            after.get_placed_heap().ptr.as_ptr()
        );
        debug_assert_eq!(before.get_heap_offset(), after.get_heap_offset());
        debug_assert_eq!(before.get_alloc_info(), after.get_alloc_info());
        ResourceAliasingBarrier{
            before: before.as_raw().ptr.as_mut_ptr(),
            after: after.as_raw().ptr.as_mut_ptr(),
        }
    }
}

/// describes a barrier in which all uav access to a resource must complete
/// before subsequent uav accesses can begin
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ResourceUavBarrier {
    resource: *mut ::winapi::ID3D12Resource,
}

impl ResourceUavBarrier {
    #[inline]
    pub fn new(resource: &RawResource) -> Self {
        ResourceUavBarrier{
            resource: resource.ptr.as_mut_ptr()
        }
    }
}

bitflags!{
    /// misc resource barrier flags. [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn986741(v=vs.85).aspx)
    #[repr(C)]
    pub struct ResourceBarrierFlags: u32 {
        const RESOURCE_BARRIER_FLAG_NONE = 0;
        const RESOURCE_BARRIER_FLAG_BEGIN_ONLY = 0x1;
        const RESOURCE_BARRIER_FLAG_END_ONLY = 0x2;
    }
}

impl Default for ResourceBarrierFlags {
    #[inline]
    fn default() -> Self {
        RESOURCE_BARRIER_FLAG_NONE
    }
}
