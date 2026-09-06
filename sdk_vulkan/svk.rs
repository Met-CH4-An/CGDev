// SPDX-License-Identifier: None
// Copyright (c) 2026 None

pub type VkFlag = u32;
pub struct SomethingFlags(VkFlag);

type SomethingBits = SomethingFlags;

impl SomethingBits {
    pub const A: u32 = 0;
}

 fn asd() {
     let a: SomethingFlags;
     let b: SomethingBits;
     SomethingFlags::A;
     SomethingBits::A;
 }