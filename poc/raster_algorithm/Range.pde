class Range {
    public float min;
    public float max;

    public Range(float min, float max) {
        this.min = min;
        this.max = max;
    }

    public float length() {
        return this.max - this.min;
    }

    public float portion(int divisor, int part) {
        return this.min + (this.length()/divisor)*part;
    }
}