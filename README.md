# floui-rs

Rust bindings for [floui](https://github.com/MoAlyousef/floui), pronounced "flowy", a proof-of-concept single header C++17 lib inspired by SwiftUI, which wraps native iOS and Android controls/widgets, and integrates into the de facto build environments of each platform (XCode and Android Studio).

![image](https://user-images.githubusercontent.com/37966791/178625452-4edc4d01-157e-413c-9610-826a94f407e7.png)

## Currently available controls:
- Text
- TextField
- Button
- VStack (Vertical UIStackView on iOS and LinearLayout on Android)
- HStack (Horizontal UIStackView on iOS and LinearLayout on Android)
- Spacer
- Toggle/Check (tvOS doesn't support it)
- Slider (tvOS doesn't support it)
- ImageView
- WebView
- ScrollView

## Usage
You can check out the [floui-rs-template](https://github.com/MoAlyousef/floui-rs-template), which is structured to be able to build for both ios or android from the command-line.

If you would like to build purely for iOS and only using Rust (no Objective-C), check the example [here](https://github.com/MoAlyousef/floui-rs-ios).

Otherwise, if you would like to do it manually:

Build your library as a static-lib:
```toml
# Cargo.toml
[lib]
crate-type = ["static-lib"]

[dependencies]
floui = "0.1"
```
Build against the android and ios architectures you might need.

### Rust
```rust
use floui::{enums::*, prelude::*, widgets::*};
use std::cell::RefCell;
use std::rc::Rc;

fn mygui(vc: &ViewController) -> MainView {
    let count = Rc::from(RefCell::from(0));
    MainView::new(
        &vc,
        &[
            &Button::new("Increment").foreground(Color::Blue).action({
                let count = count.clone();
                move |_| {
                    log("Increment clicked");
                    let mut c = count.borrow_mut();
                    *c += 1;
                    let t: Text = from_id("mytext").unwrap();
                    t.text(&format!("{}", c));
                }
            }),
            &Text::new("0").id("mytext").center().bold(),
            &Button::new("Decrement")
                .foreground(Color::Red)
                .action(move |_| {
                    log("Decrement clicked");
                    let mut c = count.borrow_mut();
                    *c -= 1;
                    let t: Text = from_id("mytext").unwrap();
                    t.text(&format!("{}", c));
                }),
        ],
    )
}

use std::os::raw::c_void;

#[no_mangle]
extern "C" fn floui_handle_events(arg1: *mut c_void) {
    unsafe { ViewController::handle_events(arg1); }
}

#[no_mangle]
extern "C" fn floui_main(arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> *mut c_void {
    let vc = unsafe { ViewController::new(arg1, arg2, arg3) };
    mygui(&vc).underlying() as _
}
```

## Notes on certain usages
- Sliders on Android take the full width of the LinearLayout, so this must be taken into consideration if code is shared also with iOS.
- Adding images has to be in the project's resource file.
    - In Android Studio: Resource Manager, Import Drawables. This will add the file to res/drawable. The file can be accessed directly ImageView::load("MyImage.jpg").
    - In XCode: You can simply drag images into Assets.xcassets, then the image can be accessed directly ImageView::load("MyImage.jpg").

- Using the WebView widget
    - on iOS: 
        - Requires adding WebKit.framework under General > Frameworks, Libraries and Embedded Content.
        - Requires enabling the `ios-webview` flag in your Cargo.toml.
        - Local files can be loaded using WebView::load_url() but need to be preceded by `file:///`, the files should be added to your xcode project.
    - On Android:
        - To load local files, precede them with `file:///` and the path of the file, which should be added to an assets folder (File > New > Folder > Assets folder). This then can be loaded using WebView::load_url().
        - To load http requests, you need to enable the internet permission in your AndroidManifest.xml: `<uses-permission android:name="android.permission.INTERNET" />`

## Creating new widgets
Wrapping platform widgets doesn't require using C++, it requires that the widget implements WidgetExt, and the underlying (JNI jobject pointer for Android, UIView on iOS) can be retrieved. You can use the jni-rs crate and objc crates for such purposes.

## Target-specific structure
### iOS
- Add the required ios rustup targets.
- Install cargo-lipo.
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
project(myapplication)

find_library(log-lib log)

add_library(myapplication SHARED native-lib.cpp)
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

target_link_libraries(myapplication android rust-lib ${log-lib})
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
