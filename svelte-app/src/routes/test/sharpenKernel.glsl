
vec4 operation(float y, float x) {
  vec4 data = pickValue_tSrc(y, x);
  
    vec4 up = pickValue_tSrc(y - 1.0, x);
    vec4 left = pickValue_tSrc(y, x - 1.0);
    vec4 center = pickValue_tSrc(y, x);
    vec4 right = pickValue_tSrc(y + 1.0, x);
    vec4 down = pickValue_tSrc(y, x + 1.0);
    

return (1.0 + 4.0*SHARPEN_FACTOR)*center -SHARPEN_FACTOR*(up + left + right + down);
}