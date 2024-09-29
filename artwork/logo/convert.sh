cd $(dirname $0)

set -ex

# sudo snap install svgo
# sudo apt install librsvg2-bin

# Reader
original() {
    cat logo.svg
}

alias rpngconvert="convert -define png:exclude-chunks=date,time +set date:create +set date:modify +set date:timestamp "
# Writers
to_png_transparent() {
    rsvg-convert -w $1 -h $1 /dev/stdin -o $2
}
to_png_transparent_wide() {
    rsvg-convert -w $2 -h $2 /dev/stdin -o tmp.png 
    rpngconvert -size $1x$2 xc:none -background none -page +$(expr \( $1 - $2 \) / 2)+0 tmp.png -flatten $3
    rm tmp.png
}
to_png_white() {
    rsvg-convert -w $1 -h $1 --background-color white /dev/stdin -o $2
}
to_svg() {
    svgo /dev/stdin -o $1
}
to_ico() {
    convert -background transparent -define "icon:auto-resize=$2" /dev/stdin $1
}

# Sizes
margin() {
    SIZE=$(expr 48 + $1 + $1)
    OFFSET=$(expr 0 - $1)
    sed "s/viewBox=\"0 0 48 48\"/viewBox=\"$OFFSET $OFFSET $SIZE $SIZE\"/g"
}

# Colors
DEFAULT_STROKE_COLOR=000000
DEFAULT_OUTER_FILL_COLOR=ffffff
DEFAULT_INNER_FILL_COLOR=000000
parse_colors() {
    sed "s/fill:#$DEFAULT_OUTER_FILL_COLOR/fill:#OUTER_FILL_COLOR/g" |
    sed "s/fill:#$DEFAULT_INNER_FILL_COLOR/fill:#INNER_FILL_COLOR/g" |
    sed "s/stroke:#$DEFAULT_STROKE_COLOR/stroke:#STROKE_COLOR/g"
}
default_colors() {
    sed "s/fill:#OUTER_FILL_COLOR/fill:#$DEFAULT_OUTER_FILL_COLOR/g" |
    sed "s/fill:#INNER_FILL_COLOR/fill:#$DEFAULT_INNER_FILL_COLOR/g" |
    sed "s/stroke:#STROKE_COLOR/stroke:#$DEFAULT_STROKE_COLOR/g"
}
outer_fill_color() {
    sed "s/fill:#OUTER_FILL_COLOR/fill:#$1/g"
}
inner_fill_color() {
    sed "s/fill:#INNER_FILL_COLOR/fill:#$1/g"
}
stroke_color() {
    sed "s/stroke:#STROKE_COLOR/stroke:#$1/g"
}
# Stroke width
DEFAULT_STROKE_WIDTH=1
stroke_width() {
    sed "s/stroke-width:$DEFAULT_STROKE_WIDTH/stroke-width:$1/g"
}


# For Android
android_old() {
    original | to_png_white $1 ../../android/app/src/main/res/mipmap-$2/ic_launcher.png
}
android_old 48 mdpi
android_old 72 hdpi
android_old 96 xhdpi
android_old 144 xxhdpi
android_old 192 xxxhdpi
original | to_png_transparent 512 ../../android/app/src/main/res/drawable/ic_launcher_foreground.png
original | parse_colors | outer_fill_color 00000000 | default_colors | to_png_transparent 512 ../../android/app/src/main/res/drawable/ic_launcher_monochrome.png
rpngconvert -size 512x512 xc:white ../../android/app/src/main/res/drawable/ic_launcher_background.png
# # For Linux
# original | to_png_transparent 256 ../../platforms/Linux/TemplateAppDir/.DirIcon
# original | to_svg ../../platforms/Linux/TemplateAppDir/webrogue.svg

# # For Web
original | margin -6 | to_ico ../../web/root/logo.ico "16,24,32,64"

# # For Windows
# original | margin -6 | to_ico ../../platforms/Windows/logo.ico "16,32,48,256"
# original | margin -5 | to_png_transparent 48 ../../platforms/Windows/Images/LockScreenLogo.png
# original | margin  1 | to_png_transparent_wide 1240 600 ../../platforms/Windows/Images/SplashScreen.png
# original | to_png_transparent 300 ../../platforms/Windows/Images/Square150x150Logo.png
# original | to_png_transparent 88 ../../platforms/Windows/Images/Square44x44Logo.png
# original | to_png_transparent 24 ../../platforms/Windows/Images/Square44x44Logo.targetsize-24_altform-unplated.png
# original | margin -5 | to_png_transparent 50 ../../platforms/Windows/Images/StoreLogo.png
# original | margin  1 | to_png_transparent_wide 620 300 ../../platforms/Windows/Images/Wide310x150Logo.png #620x300

# For MacOS
macos_ico() {
    SIZE=$1
    STROKE_WIDTH=$2
    FILENAME=$3

    MACOS_ICO_DIR=../../apple/macOS/launcher/Assets/Assets.xcassets/AppIcon.appiconset
    original | stroke_width $STROKE_WIDTH | margin 8 | to_png_transparent $SIZE $MACOS_ICO_DIR/$FILENAME.foreground.png
    MIN_POS=$(expr 100 '*' $SIZE / 1024)
    MAX_POS=$(expr 924 '*' $SIZE / 1024)
    CORNER_RADIUS=$(expr 184 '*' $SIZE / 1024)
    # rounded rectangle(background)
    rpngconvert -size $SIZE'x'$SIZE xc:transparent -fill white -draw "roundrectangle $MIN_POS,$MIN_POS $MAX_POS,$MAX_POS $CORNER_RADIUS,$CORNER_RADIUS" $MACOS_ICO_DIR/$FILENAME.background.png
    # merge foreground and background
    rpngconvert -background none $MACOS_ICO_DIR/$FILENAME.background.png $MACOS_ICO_DIR/$FILENAME.foreground.png -layers flatten -resize $SIZE'x'$SIZE $MACOS_ICO_DIR/$FILENAME.png
    rm $MACOS_ICO_DIR/$FILENAME.background.png $MACOS_ICO_DIR/$FILENAME.foreground.png
}
macos_ico   16      2   macos16
macos_ico   32      2   macos16_x2
macos_ico   32      1.5 macos32
macos_ico   64      1.5 macos32_x2
macos_ico   128     1   macos128
macos_ico   256     1   macos128_x2
macos_ico   256     1   macos256
macos_ico   512     1   macos256_x2
macos_ico   512     1   macos512
macos_ico   1024    1   macos512_x2

# For iOS
original | margin  2 | to_png_white 1024 ../../apple/iOS/Assets/Assets.xcassets/AppIcon.appiconset/ios1024.png
