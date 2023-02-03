# Lapix: Pixel Art and Sprite Editor

Goal: create a aseprite-like program, with similar amount of functionality and
quality.

Features (+ means done, (-) means least important):

- Canvas to edit sprite (+)
  - Resizable (+)
  - Multiple frames
  - Layers (+)
    - control visibility (+)
    - control editability (+)
    - layer panel allow to change layer position, remove and add (+) layers
    - group/ungroup layers (-)
  - Transparent (+) or solid background
  - Zoom in and out (+) and predefined zoom levels
  - Pan (+)
  - Background and helpers
    - color
    - checked (+)
    - grid
      - editable size
    - rulers (-)
  - Simetry
    - Vertical/horizontal
    - By any line (not in aseprite)
    - single or double
- Color palette
  - Add and remove colors
  - Save and load palettes
  - Default palettes
  - Sorting based on multiple possible properties (-)
  - possible to edit a color in the palette based on full color picker
- Save and load images (+)
  - Choose Scaled
  - Different formats (PNG (+), JPEG, GIF)
- Preview
  - Real size
  - Animation
- Commands
  - Customizable shortcuts (-)
  - Try to have good defaults
  - Undo (+) and redo
- Tools
  - Brush (+)
    - Resizable
    - Different shapes (-)
    - Erasing mode (+)
    - Pixel perfect mode
    - Different color on right and left click
  - Bucket (+)
    - adjustable tolerance
    - all areas with same color
    - gradient
  - Shapes
    - Lines (+)
    - Rectangles
    - Ovals
      - properties of recently created objects (lines, shapes)
  - Selection
    - Rectangular
    - Lasso
    - Oval (-) and poligonal
    - Add and remove to/from selection (-)
    - By color area (with customizable tolerance) or all of the same color
      (magic wand)
    - Copy and paste
    - Move
    - When selection is active, might affect other tools, like bucket
    - create selection based on what's on a layer (-)
  - Eyedrop to select colors (+)
  - Transform
    - Resize
    - Rotate
    - Flip (horizontal or vertical)
  - Effects
    - add outline
    - change colors (hue, saturation, brightness, contrast, etc)
- UI
  - assume we may change GUI, so make everything generic if possible (+)
  - preferences menu
  - help menu

