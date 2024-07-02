cmake_minimum_required(VERSION 3.10)

set(WASMER_USER_CARGO_BUILD_OPTIONS "" CACHE STRING "Additional cargo flags (such as --features) to apply to the build command")
set(WASMER_CARGO_BUILD_COMMAND "build" CACHE STRING "cargo build command")
set(WASMER_BUILD_SHARED_LIBS OFF)
option(WASMER_ALWAYS_BUILD "If cmake should always invoke cargo to build wasmer" ON)

if(CMAKE_BUILD_TYPE STREQUAL "Release")
	set(WASMER_BUILD_TYPE_FLAG "--release")
	set(WASMER_BUILD_TYPE "release")
else()
	set(WASMER_BUILD_TYPE "debug")
endif()

if(ANDROID)
	# TODO wasmer only supports arm64-v8a right now
	if(ANDROID_ABI STREQUAL "armeabi-v7a")
		set(ANDROID_TARGET "armv7-linux-androideabi")
		set(ANDROID_ARCH_SHORT "arm")
	elseif(ANDROID_ABI STREQUAL "arm64-v8a")
		set(ANDROID_TARGET "aarch64-linux-android")
		set(ANDROID_ARCH_SHORT "aarch64")
	elseif(ANDROID_ABI STREQUAL "x86")
		set(ANDROID_TARGET "i686-linux-android")
		set(ANDROID_ARCH_SHORT "i386")
	elseif(ANDROID_ABI STREQUAL "x86_64")
		set(ANDROID_TARGET "x86_64-linux-android")
		set(ANDROID_ARCH_SHORT "x86_64")
	else()
		message(FATAL_ERROR "e1")
	endif()

	set(WASMER_BUILD_TARGET "--target=${ANDROID_TARGET}")
else()
	message(FATAL_ERROR "e1")
endif()

# if (WASMER_BUILD_SHARED_LIBS AND ANDROID)
# 	message(FATAL_ERROR "Wasmtime cannot be built with WASMER_BUILD_SHARED_LIBS on Android")
# endif()

if(WASMER_BUILD_SHARED_LIBS)
	if(WIN32)
		set(WASMER_BUILD_PRODUCT
			${CMAKE_CURRENT_SOURCE_DIR}/../target/${WASMER_BUILD_TYPE}/webrogue_android.dll.lib)
	elseif(APPLE)
		set(WASMER_BUILD_PRODUCT
			${CMAKE_CURRENT_SOURCE_DIR}/../target/${WASMER_BUILD_TYPE}/libwebrogue_android.dylib)
	else()
		set(WASMER_BUILD_PRODUCT
			${CMAKE_CURRENT_SOURCE_DIR}/../target/${WASMER_BUILD_TYPE}/libwebrogue_android.so)
	endif()
else()
	if(WIN32)
		set(WASMER_BUILD_PRODUCT
			${CMAKE_CURRENT_SOURCE_DIR}/../target/${WASMER_BUILD_TYPE}/webrogue_android.lib)
	elseif(ANDROID)
		set(WASMER_BUILD_PRODUCT
			${CMAKE_CURRENT_SOURCE_DIR}/../target/${ANDROID_TARGET}/${WASMER_BUILD_TYPE}/libwebrogue_android.a)
	else()
		set(WASMER_BUILD_PRODUCT
			${CMAKE_CURRENT_SOURCE_DIR}/../target/${WASMER_BUILD_TYPE}/libwebrogue_android.a)
	endif()
endif()

if(ANDROID)
	# Rust attempts to use libgcc.a on NDK versions r23-beta3 and up
	# but it has been replaced with libunwind.a (rust-lang/rust#85806)
	file(WRITE ${CMAKE_BINARY_DIR}/libgcc.a "INPUT(-lunwind)")
	# The version of the clang compiler is part of the libunwind.a path
	file(STRINGS ${ANDROID_TOOLCHAIN_ROOT}/AndroidVersion.txt CLANG_VERSION_FILE)
	list(GET CLANG_VERSION_FILE 0 CLANG_VERSION)

	# Some crates use the compiler directly, environment variables
	# are set to make them use the Android compiler
	set(WASMER_PREBUILD_COMMAND ${CMAKE_COMMAND} -E env
	CC=${ANDROID_TOOLCHAIN_ROOT}/bin/clang
	AR=${ANDROID_TOOLCHAIN_ROOT}/bin/llvm-ar
	"RUSTFLAGS=-L ${CMAKE_SYSROOT}/usr/lib/${ANDROID_TARGET}/${ANDROID_NATIVE_API_LEVEL} \
		-L ${ANDROID_TOOLCHAIN_ROOT}/lib64/clang/${CLANG_VERSION}/lib/linux/${ANDROID_ARCH_SHORT} \
		-L ${CMAKE_BINARY_DIR} -C linker=${ANDROID_TOOLCHAIN_ROOT}/bin/ld")
endif()
include(ExternalProject)
find_program(WASMER_CARGO_BINARY cargo)
ExternalProject_Add(
	webrogue_android-crate
	SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../
	DOWNLOAD_COMMAND ""
	CONFIGURE_COMMAND ""
	INSTALL_COMMAND "${WASMER_INSTALL_COMMAND}"
        BUILD_COMMAND ${WASMER_PREBUILD_COMMAND} ${WASMER_CARGO_BINARY} ${WASMER_CARGO_BUILD_COMMAND} ${WASMER_BUILD_TYPE_FLAG} ${WASMER_USER_CARGO_BUILD_OPTIONS} ${WASMER_BUILD_TARGET}
	BINARY_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../target
        BUILD_ALWAYS ${WASMER_ALWAYS_BUILD}
	BUILD_BYPRODUCTS ${WASMER_BUILD_PRODUCT}
)
add_library(webrogue_android INTERFACE)
add_dependencies(webrogue_android webrogue_android-crate)

if(WASMER_BUILD_SHARED_LIBS)
	if(NOT WIN32)
		set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,-rpath='$ORIGIN'")
	endif()
	target_link_libraries(webrogue_android INTERFACE ${WASMER_BUILD_PRODUCT})
else()
	if(WIN32)
		target_compile_options(webrogue_android INTERFACE -DWASM_API_EXTERN= -DWASI_API_EXTERN=)
		target_link_libraries(webrogue_android INTERFACE ${WASMER_BUILD_PRODUCT}
			ws2_32 advapi32 userenv ntdll shell32 ole32 bcrypt)
	elseif(APPLE OR ANDROID)
		target_link_libraries(webrogue_android INTERFACE ${WASMER_BUILD_PRODUCT})
	else()
		target_link_libraries(webrogue_android INTERFACE ${WASMER_BUILD_PRODUCT}
			pthread dl m)
	endif()
endif()
