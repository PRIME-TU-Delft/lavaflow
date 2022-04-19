# Functional Requirements

1. Students should be able to open a camera and point it at the 2d map
1. The 2d model is recognizable from any angle above 15 degrees from the table.
1. The 2d model must be drawed with black ink on a white surface.
1. The 3d model must be placed on the paper model
1. The height of the table-top is considered the zero-point for the 3d model.
1. Every contour level in a countour map represents an increase of one height unit.
1. The 3d model had a maximum height of 1 meter above the table.

# Non Functional requirements

1. Up to 1500 students are able to use the application simultaneously
1. The live view should support a minimum framerate of 15FPS on phones that were released after 2015.
1. Altitude levels on the 3d model are represented with positive floating-point numbers between [0, 1]
1. The codebase is well-documented; every larger method has documentation.15FPS

# Design

1. 2d map is printed with symbols on the map
