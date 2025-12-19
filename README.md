### If you like my work and want to support what I do, support me on Ko-Fi 💜!
[![Ko-Fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/cyberite)
---

![Crates.io MSRV](https://img.shields.io/crates/msrv/amffi?style=for-the-badge) ![Crates.io License](https://img.shields.io/crates/l/amffi?style=for-the-badge) ![GitHub Repo stars](https://img.shields.io/github/stars/AlsoSylv/amffi?style=for-the-badge)

amffi is a wrapper of [Advanced Media Framework's](https://github.com/GPUOpen-LibrariesAndSDKs/AMF) C++ API.

This works as a cross platform COM style API, in a similar way to AMDs own examples.

32-bit environments are currently supported through the usage of macros to change the calling convention between x86_64 and x86, and any breakage is considered a bug

```c++
#include "public/common/AMFFactory.h"

int main() {
    AMF_RESULT res = AMF_OK;
    res = g_AMFFactory.Init();

    amf::AMFContextPtr context;
    res = g_AMFFactory.GetFactory()->CreateContext(&context);
}
```

```rust
use amf_bindings::amf_init;
use amf_bindings::version::AMF_VERSION;
use amf_bindings::factory::AMFFactory;
use amf_bindings::context::AMFContext;

fn main() {
    let library = amf_init().unwrap();

    let factory: AMFFactory = library.init_factory(AMF_VERSION).unwrap();
    let context: AMFContext = factory.create_context().unwrap();
}
```

This creates an AMF context, which can then be used like a COM-API

Certain APIs, such as buffer observers, have been wrapped to create a more idiomatic feeling API, such as 

```rust
fn import_dx_buffer(context: &AMFContext, buffer: &ID3D11Texture2D) {
    let surface: AMFSurface = context.create_surface_from_dx11_native(buffer);
    surface.add_observer(|surface: AMFSurface| {
        println!("Releasing!");
    });
}
```

This is done by creating trait wrappers for the equivalent C++ interface, such as:
```rust
pub trait SurfaceObserver {
    fn on_surface_data_release(&mut self, surface: AMFSurface);
}
```

And wrapping them in a struct equivalent to how the AMD driver expects them, like so:

```rust
#[repr(C)]
pub(crate) struct InternalSurfaceObserver<T: SurfaceObserver> {
    vtbl: &'static AMFSurfaceObserverVtbl,
    this: T,
}

impl<T: SurfaceObserver> InternalSurfaceObserver<T> {
    pub(crate) fn new(observer: T) -> Self {
        Self {
            vtbl: &AMFSurfaceObserverVtbl {
                on_surface_data_release: internal_observer::<T>,
            },
            this: observer,
        }
    }
}

stdcall! {
    fn internal_observer<T: SurfaceObserver>(this: *mut *const AMFSurfaceObserverVtbl, surface: AMFSurface) {
        let this = unsafe { &mut *(this as *mut InternalSurfaceObserver<T>) };
        this.this.on_surface_data_release(surface);
    }
}
```
```c++
class AMF_NO_VTABLE AMFSurfaceObserver
{
public:
    virtual void AMF_STD_CALL OnSurfaceDataRelease(AMFSurface* pSurface) = 0;
};
```

Not all features are currently implemented, but are planned.
Things such as Compute, D3D12 and AV1 are all going to be implemented as time and testing allow.