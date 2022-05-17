# OpenCV perspective warping

We have created a [simple demo page](/poc/image_processing_web/) that allows users to upload an image, which is then shown to them. They can place markers on the image, which are then used to transform the perspective.

p5.js shows the image on a canvas, together with the perspective markers, which the user can drag around. When the user is done, they can click the "warp perspective!" button, which triggers the OpenCV part of the demo.

Because OpenCV.js can only load images from an image element in the DOM (and not from a JavaScript file object), a hidden dummy image element is created. When the user uploads an image, this dummy loads that image, and OpenCV loads the image from there.

The [removePerspective](/poc/image_processing_web/src/removePerspective.js) function takes as input four points (which are the locations of the markers) and the image to be transformed. It first creates a matrix corresponds to a perspective transformation, then applies it to the input image.  
The removePerspective function requires the points as input to be in clockwise order. Giving the points out of order gives strange output. Giving the points in counter-clockwise order will result in the output image being mirrored. In addition, rotating the points will result in the output image being rotated.
