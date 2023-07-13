# 4D-Game
Not really a game yet, but an interactive renderer for 4D geometry.

## What am I looking at?
In `Degenerate4D` mode you are seeing two things: a slice and skeletons. The slice is a 3D slice of the 4D world. The slice rotates with the camera, seemingly deforming objects (in reality, you are simpy seeing different cross-sections of the objects). The wireframes you see are skeletons. Skeletons are projected onto the slice volume, making it possible to see outside the slice. Skeletons become less visible when futher from the slice (measured as an angle), and are tinted either red or blue, depending on which side of the slice they are.

The same things can be seen in `Degenerate3D` mode, but a dimension down. The world is 3D, screen 1D and skeletons are made of points instead of lines. See controls for more visual modes.

## Controls

### Moving
 - Walking: WASD + QE
 - Up & down: Space & shift
 - Rotating the camera: Move the mouse. Axes of rotation are different in 4D, depending on wheter or not the left button is pressed.

### Switching visual modes
 - Normal3D (nothing unusual): 1 (not on the numpad)
 - Combined3D (Normal3D and Degenerate3D overlayed on top of each other): 2
 - Degenerate3D (to help understand Degenerate4D): 3
 - Degenerate4D (the 4D world): 4

### Extra functions
 - Reload options: F1 (options are at `Resources/options.json` and `Resources/dev_options.json`)
 - Free the mouse: F2
 - Debug info: F3
 - Render points or lines: F4 (also disables skeleton rendering)
