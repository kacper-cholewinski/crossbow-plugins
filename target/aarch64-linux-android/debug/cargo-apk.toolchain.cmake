set(ANDROID_PLATFORM android-31)
        set(ANDROID_ABI arm64-v8a)
        string(REPLACE "--target=aarch64-linux-android" "" CMAKE_C_FLAGS "${CMAKE_C_FLAGS}")
        string(REPLACE "--target=aarch64-linux-android" "" CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}")
        unset(CMAKE_C_COMPILER CACHE)
        unset(CMAKE_CXX_COMPILER CACHE)
        include("C:\Users\kchfo\AppData\Local\Android\Sdk\ndk\21.4.7075529/build/cmake/android.toolchain.cmake")