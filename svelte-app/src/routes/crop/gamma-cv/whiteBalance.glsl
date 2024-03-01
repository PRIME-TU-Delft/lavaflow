// Kernel for white balancing paper
// We assume the corner pixels are "white". We use the average of the corner pixels to white balance the image.
// By deviding the data by the average of the corner pixels, we are effectively making "similar" pixels to the corner pixels closers to true white.
vec4 operation(float y, float x) {
  vec4 topLeft = pickValue_tSrc(0.0, 0.0);
  vec4 topRight = pickValue_tSrc(0.0, 1.0);
  vec4 bottomLeft = pickValue_tSrc(1.0, 0.0);
  vec4 bottomRight = pickValue_tSrc(1.0, 1.0);

  vec4 avgCorner = (topLeft + topRight + bottomLeft + bottomRight) / 4.0;

  vec4 data = pickValue_tSrc(y, x);


  return data / avgCorner;
}