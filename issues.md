Projects issues and bugs:

1. Line stroke is not registered when passing out of screen
2. Line stroke is not pixel perfect [ENH]
3. When interacting with UI elements, draws on canvas behind
4. Undo not working with canvas resizes
5. When line is started outside of canvas, we do not register the line start
6. Need some way of controlling the repetition of keydown time [DONE]
7. Eraser: group edits while mouse is pressed, end edit when mouse released [ENH][DONE]
8. Eraser: similar issue as with brush, non-contiguous strokes
9. When ending a line outside of canvas, cannot undo
10. Sometimes when leaving the canvas, tool is not released and continue drawing
11. Improve the visibility of the tool cursors (e.g. on black) [ENH]
12. Data about tools is too spread around: Tool enum, data about icons, etc.
13. Now that we changed event processing, it's not possible to have 2 events at the same time, that is making it impossible to scroll horizontally and vertically together [DONE]
14. Change name to Tarsila or Portinari? [ENH]
15. When zooming out, camera needs to be readjusted [ENH][DONE]
16. Eyedropper should get visible color, not the one on the layer

