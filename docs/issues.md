Projects issues and bugs:

1. Line stroke is not registered when passing out of screen [DONE]
2. Line stroke is not pixel perfect [ENH]
3. When interacting with UI elements, draws on canvas behind
4. Undo not working with canvas resizes [DONE]
5. When line is started outside of canvas, we do not register the line start [DONE]
6. Need some way of controlling the repetition of keydown time [DONE]
7. Eraser: group edits while mouse is pressed, end edit when mouse released [ENH][DONE]
8. Eraser: similar issue as with brush, non-contiguous strokes [DONE]
9. When ending a line outside of canvas, cannot undo [DONE]
10. Sometimes when leaving the canvas, tool is not released and continue drawing [DONE]
11. Improve the visibility of the tool cursors (e.g. on black) [ENH][DONE]
12. Data about tools is too spread around: Tool enum, data about icons, etc.
13. Now that we changed event processing, it's not possible to have 2 events at the same time, that is making it impossible to scroll horizontally and vertically together [DONE]
14. Change name to Tarsila or Portinari? [ENH][DONE]
15. When zooming out, camera needs to be readjusted [ENH][DONE]
16. Eyedropper should get visible color, not the one on the layer [DONE]
17. When the active layer is deleted, move active layer to the one below [DONE]
18. Inconsistent use of types for 2D points and colors [DONE]
19. Optimize syncing of preview and palette images (don't need to generate every single frame [ENH]
20. Create mouse manager, and store events to be ran on mouse release [DONE]
21. Cannot type on spritesheet size textboxes if intermediate values are not valid spritesheet sizes[DONE]
22. Preview window gets too big and cannot be closed or resized [DONE]
23. Add tooltips to everything [ENH]
24. Use default mouse pointer out of canvas [DONE]
25. Preview animation not affected by layer's alpha
26. When leaving the window, tool is not released and continues drawing
27. When pasting in another layer, floating object is anchored and selection is on canvas [DONE]
28. When pasting in another layer, tool is set to selection instead of move [DONE]
29. Limit the amount of data on Undo list [ENH]
30. If selection is not fully in canvas, panic [DONE]
31. When dragging preview window, draws on canvas behind
32. CTRL shortcuts not working on Mac
33. File dialog not working on Mac
