vec4 operation(float y, float x) {
  vec4 data = pickValue_tSrc(y, x);

  return vec4(
    (1.0 - data.r),
    (1.0 - data.g),
    (1.0 - data.b),
    1.0
  );
}