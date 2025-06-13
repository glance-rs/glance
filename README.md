# Glance
Glance aims to be a modular computer vision library written in Rust.

## Features

### Image Manipulation
- [x] Create and save images
- [x] Load images from files
- [x] Display images (cross platform)
- [x] Draw shapes
- [ ] Draw text

### Point Operations
- [x] Pixel-wise operations
- [x] Color adjustments (brightness, contrast, gamma)
- [ ] Color space conversions
    - [x] RGBA to Grayscale
    - [ ] RGBA to HSV
    - [ ] RGBA to YUV
- [x] Thresholding
- [x] Histogram equalization
- [ ] Adaptive histogram equalization (CLAHE)

### Linear Filters
- [x] Gaussian blur
- [x] Box blur
- [x] Sobel filter
- [x] Laplacian filter
- [ ] Unsharp masking
- [ ] Frequency domain filtering (FFT-based)

### Non-linear Filters
- [x] Median filter
- [ ] Bilateral filter
- [ ] Noise reduction filters
- [x] Morphological operations
    - [x] Erosion
    - [x] Dilation

### Detection & Recognition
- [ ] Edge detection (Canny, Harris corner detection)
- [ ] Feature detection and description (SIFT, ORB, FAST)
- [ ] Template matching
- [ ] Contour detection and analysis
- [ ] Blob detection
- [ ] Hough transforms (lines, circles)

### Geometric Transformations
- [ ] Scaling, rotation, translation
- [ ] Perspective transformation
- [ ] Image warping and rectification
- [ ] Image registration and alignment

### Segmentation
- [ ] Watershed segmentation
- [ ] Region growing
- [ ] K-means clustering for segmentation
- [ ] Graph-based segmentation

### Analysis & Metrics
- [ ] Histogram computation and analysis
- [ ] Image statistics (mean, variance, entropy)
- [ ] Image quality metrics (PSNR, SSIM)
- [ ] Connected component analysis

### Advanced Processing
- [ ] Image pyramids (Gaussian, Laplacian)
- [ ] Integral images
- [ ] Distance transforms
- [ ] Convex hull computation

### I/O & Utilities
- [ ] Video frame extraction
- [ ] Batch processing utilities
- [ ] Memory-efficient streaming for large images
- [ ] Zero-copy operations where possible

### Performance & Optimization
- [ ] SIMD optimizations
- [x] Multi-threading support (currently using Rayon)
