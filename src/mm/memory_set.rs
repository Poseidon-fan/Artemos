use alloc::collections::BTreeMap;
use crate::mm::address::{VPNRange, VirtPageNum};
use crate::mm::frame_allocator::FrameTracker;

#[derive(Copy, Clone, PartialEq, Debug)]
/// map type for memory set: identical or framed
pub enum MapType {
    Identical,
    Framed,
}

bitflags! {
    /// map permission corresponding to that in pte: `R W X U`
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

pub struct MapArea {
    // 是一个迭代器，描述一段虚拟页号的连续区间
    vpn_range: VPNRange,
    // 当逻辑段采用 MapType::Framed 方式映射到物理内存的时候
    // data_frames 是一个保存了该逻辑段内的每个虚拟页面和它被映射到的物理页帧的一个键值对容器
    // IIRA
    data_frames: BTreeMap<VirtPageNum, FrameTracker>,
    // 描述该逻辑段内的所有虚拟页面映射到物理页帧的方式：Identical, Framed
    map_type: MapType,
    // URWX 四个标志位
    map_perm: MapPermission,
}
