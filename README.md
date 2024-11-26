# Rusty Rays

Learning Rust by implementing a ray tracer.

My latest rendered image:

![Latest rendered image](./image.png)

## To-Do

- [ ] Unit tests
- [ ] GitHub Actions
- [ ] Closed loop ray casting per pixel
- [ ] GUI mode for adjusting parameters
- [ ] Sequential rendering across the full image
- [ ] Loading scenes from files
- [ ] Render triangles
- [ ] Subsurface scattering
- [ ] Render STLs
- [ ] Render OBJs
- [ ] Unified material struct instead of enums-per-material
- [ ] Batch mode to only render part of image and return
- [X] Multithreaded CPU rendering
- [ ] GPU compute

## Project Goals

- [ ] Cornel box render
- [ ] "Icy" bouncy ball material render
- [ ] Stanford dragon model render
- [ ] Batch process pipeline
- [ ] Rerender a Processing Daily
- [X] Pre-commit hook to upload latest rendered image automatically
- [X] Git LFS to limit repo size creep with images

## Book(s) Used

This is not me deriving computer graphics theory, just getting familiar with the Rust language.

As such, I'm following along with existing resources implementing their solutions in Rust as an exercise.



### Ray Tracing The Next Week

[_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html)

- [ ] Chapter  2 - Motion Blur
- [ ] Chapter  3 - Bounding Volume Hierarchies
- [ ] Chapter  4 - Texture Mapping
- [ ] Chapter  5 - Perlin Noise
- [ ] Chapter  6 - Quadrilaterals
- [ ] Chapter  7 - Lights
- [ ] Chapter  8 - Instances
- [ ] Chapter  9 - Volumes
- [ ] Chapter 10 - A Scene Testing All New Features

### Ray Tracing In One Weekend

[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

- [X] Chapter  1 - Overview
- [X] Chapter  2 - Output an Image
- [X] Chapter  3 - Vec3 Implementation
- [X] Chapter  4 - Rays, Simple Camera, and Background
- [X] Chapter  5 - Adding a Sphere
- [X] Chapter  6 - Surface Normals and Multiple Objects
- [X] Chapter  7 - Camera Code Refactoring
- [X] Chapter  8 - Antialiasing
- [X] Chapter  9 - Diffuse Materials
- [X] Chapter 10 - Metal
- [X] Chapter 11 - Dielectrics
- [X] Chapter 12 - Positionable Camera
- [X] Chapter 13 - Defocus Blur
- [X] Chapter 14 - Next Steps
