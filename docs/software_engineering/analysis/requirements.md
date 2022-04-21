### Functional requirements

1. [MUST] Students and teachers should be able to open a camera and point it at the contour
map.
2. [MUST] The contour map must be recognizable from any angle above 15 degrees from the
table.
3. [MUST] The application must recognize the contour map with black ink on a white
surface.
4. [MUST] The 3d model must be virtually placed on the paper model.
5. [MUST] The height of the table-top must be considered the zero-point for the 3d model,
and every contour level in a contour map represents a fixed increase of one height
unit.
6. [MUST] The 3d model has a maximum height of 1 meter above the table.
7. [MUST] The application is able to recognize invalid contour maps.
8. [MUST] Users should get a notification if they draw an invalid contour map.
9. [MUST] After scanning their contour map, users must be able to preview their model in a
non-AR view.
10. [MUST] A user should be able to press a button to start a lava simulation over the 3d model.
11. [MUST] In a lava simulation, lava will flow from a certain point on the 3d-model and flow
towards the bottom of the mountain, following the steepest gradients.
12. [SHOULD] Users will be able to place markers on the contour map that represent the point
where the lava should flow from. There may be maximally one such crater.
13. [SHOULD] Students and teachers should be able to place houses on the contour map, by
moving physical markers (in the form of e.g. pieces of paper) with a contrasting
color. This is done in a second scan, after the contour map itself has already been
loaded.
14. [SHOULD] Houses, placed on the contour map, must be presented on the 3d model.
15. [SHOULD] The 3d model should rotate and move with the paper if it is moved.
16. [SHOULD] Students should be able to walk around the contour map to see the virtual 3d-
model from all sides.
17. [SHOULD] The application should start with a ‘landing page’, containing information on how
to use the application.
18. [SHOULD] A user can configure the increase in height per contour line.
19. [COULD] Users will be able to place multiple markers on the contour map that represent
points where the lava should flow from. Passing this requirement will overwrite
requirement [12].
20. [COULD] Houses will be visualized with randomized textures, to support the gamification
aspect of this application.
21. [COULD] Users will be able to select different weather-modes.
22. [COULD] Users will be rewarded points for every house that does not get destroyed by the
lava flow.
23. [COULD] Houses, that were placed on a more challenging place on the mountain, will be
rewarded more points than houses that were placed on trivially-safe places.
24. [COULD] Users should be able to set a difficulty-mode. Each difficulty-mode changes the
lava-flow-parameters of the 3d-model.
25. [COULD] When users point their phone away from the 3d-model, the application should
remember, by referencing the internal gyroscope, where the 3d-model was placed.
26. [COULD] Teachers could be able to share their phone’s display with the classroom’s pro-
jector, such that the students can easily follow.
27. [WON'T] Students and teachers will not be able to view multiple mountains at the same
time, by placing multiple contour maps on the same white surface.
28. [WON'T] We will not store any user data.

### Non-functional Requirements

29. [MUST] The application should be supported on iOS- and Android-phones.
30. [MUST] Up to 1500 students are able to use the application simultaneously.
31. [MUST] The application should be easy to use, such that the average non-computer-
science teacher and average non-computer-science student will encounter at
most 2 application-errors during the first use of the application.
32. [MUST] The live view should support a minimum frame rate of 15 FPS on phones that were
released after 2015.
33. [MUST] The codebase is well-documented; every larger method has documentation, and
there should be a list of all libraries used in the project.
34. [MUST] The application should be documented using a README with instructions for
running the code. This README should be present in the Git-repository.
35. [MUST] The application should work on A-series paper.
