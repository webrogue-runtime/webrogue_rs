cmake_minimum_required(VERSION 3.13)

set(CMAKE_OSX_DEPLOYMENT_TARGET 14.0)

project(webrogue C CXX Swift)

set(CMAKE_C_STANDARD 99)
set(CMAKE_CXX_STANDARD 14)


set(WEBROGUE_SDL_VERSION "2.30.6")
set(WEBROGUE_SDL_HASH "5aef5ee599a1bbfad081f6f05755059f77d8a13248e7eb3d0c556b117c45249e")

set(WEBROGUE_SDL_DIR_NAME "SDL-release-${WEBROGUE_SDL_VERSION}")
set(WEBROGUE_SDL_DIR "${CMAKE_CURRENT_LIST_DIR}/external/SDL")

file(
    DOWNLOAD
   	"https://codeload.github.com/libsdl-org/SDL/zip/refs/tags/release-${WEBROGUE_SDL_VERSION}"
    "${CMAKE_CURRENT_LIST_DIR}/external/${WEBROGUE_SDL_DIR_NAME}.zip"
	EXPECTED_HASH "SHA256=${WEBROGUE_SDL_HASH}"
	SHOW_PROGRESS
)
if(NOT EXISTS "${WEBROGUE_SDL_DIR}")
    file(
        ARCHIVE_EXTRACT
        INPUT "${CMAKE_CURRENT_LIST_DIR}/external/${WEBROGUE_SDL_DIR_NAME}.zip"
        DESTINATION "${CMAKE_CURRENT_LIST_DIR}/external"
    )
    file(
        RENAME 
        "${CMAKE_CURRENT_LIST_DIR}/external/${WEBROGUE_SDL_DIR_NAME}" 
        "${WEBROGUE_SDL_DIR}"
    )
endif()

set(SDL_STATIC ON CACHE BOOL "")
set(SDL_STATIC ON)
set(SDL_SHARED OFF CACHE BOOL "")
set(SDL_SHARED OFF)
set(BUILD_SHARED_LIBS OFF CACHE BOOL "")
set(BUILD_SHARED_LIBS OFF)
# set(SDL2TTF_VENDORED ON CACHE BOOL "")
# set(SDL2TTF_VENDORED ON)

# patch foo.c < patch.diff
exec_program(patch ${CMAKE_CURRENT_LIST_DIR}/external/SDL ARGS --forward src/video/uikit/SDL_uikitappdelegate.m ../sdl.patch)
add_subdirectory("${WEBROGUE_SDL_DIR}" sdl)
# add_subdirectory(external/SDL_ttf sdl_ttf)

file(GLOB_RECURSE WEBROGUE_SWIFT_SOURCES ${CMAKE_CURRENT_LIST_DIR}/src/*.swift)

add_executable(
    webrogue 
    ${CMAKE_CURRENT_LIST_DIR}/src/main.h
    ${CMAKE_CURRENT_LIST_DIR}/src/main.mm
    ${CMAKE_CURRENT_LIST_DIR}/src/webrogue-Bridging-Header.h
    ${CMAKE_CURRENT_LIST_DIR}/../crates/gfx_ffi/src/webrogue_gfx_ffi_sdl2.c

    ${WEBROGUE_SWIFT_SOURCES}
)

target_include_directories(
    webrogue
    PRIVATE
    ${CMAKE_CURRENT_LIST_DIR}/external/SDL/include
)

set_target_properties(webrogue PROPERTIES XCODE_ATTRIBUTE_LIBRARY_SEARCH_PATHS "$(SRCROOT)/cmake_build/sdl/$(CONFIGURATION)$(EFFECTIVE_PLATFORM_NAME)\n$(SRCROOT)/cmake_build/sdl_ttf/$(CONFIGURATION)$(EFFECTIVE_PLATFORM_NAME)\n$(SRCROOT)/cmake_build/sdl_ttf/external/freetype/$(CONFIGURATION)$(EFFECTIVE_PLATFORM_NAME)")
set_target_properties(webrogue PROPERTIES XCODE_ATTRIBUTE_SWIFT_OBJC_BRIDGING_HEADER "$(SRCROOT)/src/webrogue-Bridging-Header.h")
set(webrogue PROPERTIES CMAKE_XCODE_ATTRIBUTE_INFOPLIST_KEY_UILaunchStoryboardName "LaunchScreen.storyboard")

target_link_libraries(
    webrogue
    ${CMAKE_CURRENT_LIST_DIR}/target/universal/libwebrogue_ios.a
    SDL2d SDL2maind 
    # SDL2_ttfd freetyped

    "-framework AVFoundation"
    "-framework AudioToolbox"
    "-framework CoreAudio"
    "-framework CoreBluetooth"
    "-framework CoreFoundation"
    "-framework CoreGraphics"
    "-framework CoreHaptics"
    "-framework CoreMotion"
    "-framework CoreVideo"
    "-framework GameController"
    "-framework Metal"
    "-framework OpenGLES"
    "-framework QuartzCore"
    "-framework UIKit"

    # "-framework SystemConfiguration"
    "-framework JavaScriptCore"

    "iconv"
)

add_custom_target("cmake_configure" COMMAND sh ${CMAKE_CURRENT_SOURCE_DIR}/configure.sh)

function(GroupSourcesByFolder target)
  set(SOURCE_GROUP_DELIMITER "/")
  set(last_dir "")
  set(files "")

  get_target_property(sources ${target} SOURCES)

  foreach(file ${sources})
    file(RELATIVE_PATH relative_file "${PROJECT_SOURCE_DIR}" ${file})
    get_filename_component(dir "${relative_file}" PATH)
    if(NOT "${dir}" STREQUAL "${last_dir}")
      if(files)
        source_group("${last_dir}" FILES ${files})
      endif()
      set(files "")
    endif()
    set(files ${files} ${file})
    set(last_dir "${dir}")
  endforeach()

  if(files)
    source_group("${last_dir}" FILES ${files})
  endif()
endfunction()

source_group(
    TREE "${CMAKE_CURRENT_LIST_DIR}"
    FILES 
    ${CMAKE_CURRENT_LIST_DIR}/src/main.h
    ${CMAKE_CURRENT_LIST_DIR}/src/main.mm
    ${CMAKE_CURRENT_LIST_DIR}/src/webrogue-Bridging-Header.h

    ${WEBROGUE_SWIFT_SOURCES}
)


set_target_properties(webrogue PROPERTIES
    BUNDLE True
    MACOSX_BUNDLE_GUI_IDENTIFIER io.github.webrogue_runtime
    MACOSX_BUNDLE_BUNDLE_NAME webrogue
    MACOSX_BUNDLE_BUNDLE_VERSION "0.1"
    MACOSX_BUNDLE_SHORT_VERSION_STRING "0.1"
    MACOSX_BUNDLE_INFO_PLIST ${CMAKE_CURRENT_LIST_DIR}/info.plist.in
)

target_sources(webrogue PUBLIC ${CMAKE_CURRENT_LIST_DIR}/LaunchScreen.storyboard)
set_source_files_properties(${CMAKE_CURRENT_LIST_DIR}/LaunchScreen.storyboard PROPERTIES MACOSX_PACKAGE_LOCATION Resources)

target_sources(webrogue PUBLIC ${CMAKE_CURRENT_LIST_DIR}/colors.xcassets)
set_source_files_properties(${CMAKE_CURRENT_LIST_DIR}/colors.xcassets PROPERTIES MACOSX_PACKAGE_LOCATION Resources)

target_sources(webrogue PUBLIC ${CMAKE_CURRENT_LIST_DIR}/Assets.xcassets)
set_source_files_properties(
    ${CMAKE_CURRENT_LIST_DIR}/Assets.xcassets
    PROPERTIES 
    MACOSX_PACKAGE_LOCATION Resources
)
set(CMAKE_XCODE_ATTRIBUTE_ASSETCATALOG_COMPILER_APPICON_NAME "AppIcon")
