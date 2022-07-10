# floui-rs

Rust bindings for [floui](https://github.com/MoAlyousef/floui).

## Usage
Build your library as a static-lib:
```toml
# Cargo.toml
[lib]
crate-type = ["static-lib"]

[dependencies]
floui = { git = "https://github.com/MoAlyousef/floui-rs" }
```

### Rust
```rust
use floui::*;
use std::os::raw::c_void;

#[no_mangle]
extern "C" fn floui_handle_events(arg1: *mut c_void) {
    ViewController::handle_events(arg1);
}

#[no_mangle]
extern "C" fn floui_main(arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> *mut c_void {
    let fvc = ViewController::new(arg1, arg2, arg3);
    MainView::new(&fvc, &[
        &Button::new("Increment").action(|_| log("Increment clicked")),
        &Text::new("0"),
        &Button::new("Decrement").action(|_| log("Increment clicked"))
    ]).inner() as _
}
```

### iOS
- Add the required ios rustup targets.
- Add the built library to your xcode project (under Build Phases > Link Binary with Libraries).
- Modify the library search path to find the library (under Build Settings > Library Search Paths).
- Modify your ViewController.m file:
```objc
#import "ViewController.h"

extern void *floui_main(void *, void *, void *);

@interface ViewController ()

@end

@implementation ViewController
- (void)viewDidLoad {
    [super viewDidLoad];
    floui_main((void *)CFBridgingRetain(self), nil, nil);
}

@end
```

### Android
- ANDROID_SDK_ROOT should be set to your android sdk directory.
- ANDROID_NDK_ROOT should be set to your android ndk directory.
- Add the required android rustup targets.
- Create an Android Studio Native C++ project, choose toolchain C++ 17 in the last step.
- Modify your CMakeLists.txt:
```cmake
cmake_minimum_required(VERSION 3.4.1)

find_library(log-lib log)

add_library(native-lib SHARED native-lib.cpp)
add_library(rust-lib STATIC IMPORTED)

if (ANDROID_ABI STREQUAL x86)
    set(RUST_ARCH i686-linux-android)
elseif (ANDROID_ABI STREQUAL armeabi-v7a)
    set(RUST_ARCH armv7-linux-androideabi)
elseif (ANDROID_ABI STREQUAL arm64-v8a)
    set(RUST_ARCH aarch64-linux-android)
elseif (ANDROID_ABI STREQUAL x86_64)
    set(RUST_ARCH x86_64-linux-android)
else ()
    message(FATAL "Unknown architecture")
endif ()

set_property(TARGET rust-lib PROPERTY IMPORTED_LOCATION_DEBUG ${CMAKE_CURRENT_LIST_DIR}/app/target/${RUST_ARCH}/debug/libapp.a)
set_property(TARGET rust-lib PROPERTY IMPORTED_LOCATION_RELEASE ${CMAKE_CURRENT_LIST_DIR}/app/target/${RUST_ARCH}/release/libapp.a)

target_link_libraries(native-lib android rust-lib ${log-lib})
```
- Modify your C++ file to just call the Rust lib.
```c++
#include <jni.h>
#include <string>

extern "C" void *floui_main(void *, void *, void *);

extern "C" void floui_handle_events(void *);

extern "C" JNIEXPORT jobject JNICALL
Java_com_example_myapplication_MainActivity_mainView(JNIEnv* env, jobject main_activity, jobject view) {
    return (jobject) floui_main(env, main_activity, view);
}

extern "C" JNIEXPORT void JNICALL
Java_com_example_myapplication_MainActivity_handleEvent(JNIEnv *env, jobject thiz, jobject view) {
    floui_handle_events(view);
}
```
- Modify your MainActivity.java to look like:
```java
package com.example.myapplication;

import androidx.annotation.NonNull;
import androidx.appcompat.app.AppCompatActivity;
import androidx.constraintlayout.widget.ConstraintLayout;

import android.os.Bundle;
import android.view.View;

import com.google.android.material.slider.Slider;

public class MainActivity extends AppCompatActivity implements View.OnClickListener, Slider.OnChangeListener {
    static {
        System.loadLibrary("myapplication");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        ConstraintLayout layout = new ConstraintLayout(this);
        setContentView(layout);
        mainView(layout);
    }
    public native View mainView(View view);
    public native void handleEvent(View view);

    @Override
    public void onClick(View view) {
        handleEvent(view);
    }

    @Override
    public void onValueChange(@NonNull Slider slider, float value, boolean fromUser) {
        handleEvent(slider);
    }
}
```