Store a PNG inside an ico, without converting to the traditional bitmap with
alpha channel. This saves a lot of space, since PNGs can be compressed and
there are tools to highly optimise them. Embedding PNG data inside a
favicon.ico file is well supported by browsers, as well.

Here's an example of converting a traditional bitmap ico file to a PNG, and
then into a PNG embedded into an ico file with this tool.

    # `convert' is from ImageMagick
    convert favicon-old.ico favicon.png

    # Optional: Shrink the png further
    pngquant -f --ext .png favicon.png
    optipng favicon.png

    # Embed into new favicon.ico
    icopng favicon.png favicon.ico

# Why not just use...?

## image::ico::ICOEncoder, or the ico crate

The image and ico crates _do_ export PNGs inside ICO files (nice!), but they
will re-encode them, thus losing all the benefits of things like
optipng/pngquant.

## ImageMagick

Tools like imagemagick can convert from PNG to ico, but they convert to bitmap
with alpha instead of just putting the PNG into an ico container, which means
the files are very large.

## GIMP

GIMP can also export as a PNG inside an ICO by exporting as ICO and selecting
"Compressed (PNG)", however then you don't get the benefits of
pngquant/optipng.
