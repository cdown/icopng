Store a png inside an ico, without converting to the traditional bitmap with
alpha channel. This saves a lot of space, since pngs can be compressed and
there are tools to highly optimise them, and is well supported by browsers for
use as favicon.ico.

Tools like imagemagick can convert from png to ico, but they convert to bitmap
with alpha instead of just putting the png into an ico container, which means
the files are very large.

GIMP can also export as a PNG inside an ICO by exporting as ICO and selecting
"Compressed (PNG)", however then you don't get the benefits of
pngquant/optipng.

Here's an example of converting a traditional bitmap ico file to a png, and
then into a png embedded into an ico file with this tool.

    # `convert' is from ImageMagick
    convert favicon-old.ico favicon.png

    # Optional: Shrink the png further
    pngquant -f --ext .png favicon.png
    optipng favicon.png

    # Embed into new favicon.ico
    icopng favicon.png favicon.ico
