# Implicit Surface Generation

Implicit surfaces are those of the form:
```
F(x, y, z) = 0
```

where x, y and z are basis coordinates in 3-space, and the surface itself is defined by the set of coordinates that evalute to zero:
```
{ v ∈ R3 : F(v) = 0 }
```

They are interesting in that a very simple `F` can describe a very complex surface, such as a surface-of-revolution or a Steiner surface. Even something as a cube that would be very complicated to paramterise can be represented easily as an implicit surface:
```
F(x, y, z) = max(|𝑥|,|𝑦|,|𝑧|)
```

They can even be trivially combined to ith boolean operation. For example the union of two implicit surfaces, `G` and `H` is simply:
```
{ v ∈ R3 : G(v) = 0 or H(v) = 0}
```

The difficulty comes in their visualisation since they cannot be evaluated, and the set of zeros must be found. Luckily this can be done using ray-tracing. The functions that define an implicit surface are in fact _scalar_fields_ and give us a _signed distance function_. Therefore, if we follow the path of a ray its intersection with the surface will be the point on the ray that evaluates to zero. Any point on the array with a positive signed distance will be outside of the surface, and any negative distance will be inside.

Why do I care? I spent almost a decade working in computation geometry dealing with parameteric and mesh surfaces. I never explored implicit surfaces, and so want to do so now. There are a lot of interesting results from the 80s I would like to relicate.

# Where I am and where I'm headed

Step 1 build a basic ray tracer (COMPLETE):
I will be using this as an excuse to learn Rust and will start by following the following tutorial to help get something bootstrapped: https://the-ray-tracing-road-to-rust.vercel.app/

This will produce a scene but the ray-tracing is solved analytically as opposed to ray-marching.

Step 2 implement signed-distance ray marching for spheres (IN PROGRESS):
This will use a basic signed distance function `f(v) = | p - v | - r` where `p` is the centre of the sphere and `r` is its radius

Step 3 build a library of primative signed-distance functions:
When spheres are working we should be able to trivially implement a set of signed-distance functions for primative surfaces such as torii, cubes.

Step 4 add composibility:
Boolean and blend operations can be implemented by combining sign-distance functions for implicit surface

Step 3 configurability:
I'll need a way to either dynamically define the implicit function and feed it into the tracer via a config file and some equation parse

Step 4 output as STL:
At this point if I'm feeling brave I can implement the marching cube algorithm and outpoint a point cloud in STL which can then be imported into CAD products.


Step 5 ???




