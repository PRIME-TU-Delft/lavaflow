// Kernel for inverting the colors of an image
vec4 operation(float y, float x) {
  vec4 data = pickValue_tSrc(y, x);

  // Check if fill is valid
  if (RADIUS <= 0.0) return data;
  
  // Get the average of the surrounding pixels weighted by own value
  vec4 sum = vec4(0.0);
  for (i = -RADIUS; i <= RADIUS; i++) {
    for (j = -RADIUS; j <= RADIUS; j++) {
      vec4 value = pickValue_tSrc(y + i, x + j);
      float weight = 1.0 / (1.0 + sqrt(float(i * i + j * j)));
      sum += value * weight;
    }
  }

  // Return the average
  return sum / (2.0 * RADIUS + 1.0) / (2.0 * RADIUS + 1.0);
}