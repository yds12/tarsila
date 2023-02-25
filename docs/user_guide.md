# Tarsila User Guide

In this manual we will see how to use Tarsila. These are its main features:

* Basic drawing with brush, eraser, lines, rectangles, bucket (fill with color);
* Color selector, editable palette and eyedropper (pick color from canvas);
* Resize or completely erase the canvas;
* Move the camera, zoom in and out;
* Parts of the drawing can be selected (rectangular selection only, for now),
  deleted, copied and pasted; selection can be flipped horizontally or
  vertically;
* Layers can be created, removed, moved up or down, can be made invisible or
  have its opacity changed;
* Spritesheet mode: specify how many columns and rows your image has, and an
  animated preview will be displayed in the preview window. Scale of the preview
  can be specified;
* Save and load projects (with its layers and palette), export and import PNG
  and JPG;
* Status bar with information about canvas size, selected tool, canvas position
  and color under mouse.

## The Canvas

You can move the camera by pressing and holding the arrow keys in the keyboard.

You can zoom in with `=` and out with `-`.

The canvas can be resized by clicking on the menu on `File > Resize canvas`. It
can also be erased completely via `File > Erase canvas`.

## Drawing

A transparent canvas is at the center of Tarsila's screen. On the left side of
the canvas you can find a toolbox which shows the names of the tools and the
keyboard shortcuts to select them. These are:

* brush: the basic drawing tool. Click and drag around the canvas to draw pixel
  by pixel;
* eraser: similar to the brush, but instead of placing colors, places
  transparent pixels (effectively erasing anything you click);
* bucket: paint a contiguous area (of the same color of the place where you
  clicked) with the selected color;
* line: click and drag to draw lines;
* rectangle: click and drag to draw rectangles;
* eyedropper: click anywhere in the canvas to select the color under the mouse;
* selection: click and drag to select an area of the canvas; after a selection
  is created, you can click on it and drag it to move it around; you can also
  press CTRL+C to copy it, and CTRL+V to paste it somewhere. The pasted image
  can be moved around and is subject to the same rules as any selection. A
  selection can also be flipped horizontally with the `H` keyboard key, and
  vertically with `V`;

## Colors and Palette

Most drawing tools use the active color to determine which color to draw. The
active color can be changed in the Toolbox panel, by clicking on the first
button on it (which starts as black). You can choose the RGB value in the
colorpicker, and set the alpha (a transparency from 0 to 255) on the `a:`
textbox next to it. Next to this there is a `+` button that allows you to add
the selected color to your palette, in case it's not there yet.

Colors can be removed from the palette by right-clicking on them. The `Load`
button on the palette panel allows you to load a palette from an image. Note
that if the image has too many colors, not all of them will be added (the
palette has a small maximum number of colors).

## Layers

Images in Tarsila can be composed of multiple overlapping layers. Each layer has
its own canvas image that is completely independent of the other layers. The
final visible image will be the result of drawing layers on top of each other.
Layer `1` is the first to be drawn, then layer `2`, and so on. If layer `2` is
completely solid (without transparency), layer `1` will be completely covered by
it and therefore invisible. The same logic can be applied between any two
layers.

On the left of the canvas you can see the layers in the Layers panel. You can
create new layers by clicking the `+` button. Each layer is identified by a
number (under `#`). Other attributes of the layers shown here are:
* `act.`: whether the layer is active; if a layer is active, anything you draw
  is applied to this layer, regardless of what you see in the canvas; be
  careful: if the active layer is below another one, you might not see what you
  are drawing; you can change the active layer by clicking on the radio button
  in this column;
* `vis.`: whether the layer is visible; you can toggle this to avoid the problem
  mentioned above, making any layers visible or invisible;
* `alpha`: the opacity of the layer (from 0 to 255). Note that a fully opaque
  layer (with alpha = 255) can still have transparent pixels if you choose a
  transparent color;

Layers can be moved up or down, or deleted, by using the buttons next to each
layer on the Layers panel. If you export an image, it will be exported
respecting the layer settings. For example, if a layer is invisible, it will not
be exported to the final image.

## Spritesheet

Tarsila is a spritesheet editor, so it aims to have tools to make the
manipulation of sprites easy. Currently, the only such tool is in the menu
`File > Change spritesheet`. You can choose how many rows and columns your image
consists of. If there are more than 1 column or row, you will see lines in the
canvas separating the frames of the sprite, and also an animation preview in the
preview window on the bottom right corner of the screen. The preview can be
scaled via the preview window.

## Saving, Loading, Importing and Exporting

To save your whole project (so that you can continue working on it later) you
can use the `File > Save Project` item. Note that the file generated by this is
only usable by Tarsila itself, and cannot be opened by any other programs. This
file contains data about each individual layer of your image, your palette,
spritesheet settings, etc.

Once you saved a `.tarsila` file (a project file), you can load it again to
continue working via the `File > Load project` menu item.

> Note: Tarsila project files have some metadata recording the version of Tarsila
> that was used to create the file. If your file cannot be opened with some
> version of Tarsila, you can inspect this version number and use the correct
> version of the program to open it. In future versions, we intend to inform you
> about the version of the file when you try to open a file that is incompatible
> (for now you will have to inspect the binary with a utility like `xxd` or ask
> someone to help you). This should not happen at all, but since we are in our
> first release (0.1.0), it's almost certain that big, backwards-incompatible
> changes are still coming. Once we stabilise (or reach 1.0), this will only
> possibly happen with new major versions.

Eventually you will want your finished work to be usable as a regular image
outside of Tarsila. For this, chose the `File > Export image` option. You can
choose the `.png` or `.jpg` extensions while naming your file.

Similarly, to import an image into your canvas, use the menu item
`File > Import image`. The image will appear as a selected floating image, that
can be moved, deleted, copied etc. If the image is too big, it will resize your
canvas so that it can fit. If you don't want that, you can resize your canvas
back to its previous size with the `File > Resize canvas` option.

