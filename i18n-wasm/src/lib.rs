pub mod wasm;
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> = unsafe {
    AssumeSingleThreaded::new(FreeListAllocator::new())
};