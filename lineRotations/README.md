# Rotating Line using OpenGL in C

## Mathematical Foundations

The project aims to visualize the rotation of a line around the origin in a 2D plane. This rotation is modeled using basic trigonometry. The rotation matrix for R2 (2D space) is given by:

```math
\begin{pmatrix}
\cos(\theta) & -\sin(\theta) \\
\sin(\theta) & \cos(\theta)
\end{pmatrix}
```

A rotation matrix essentially performs a change of basis, transforming coordinates from one frame of reference to another. In our case, we're using it to rotate a vector (line) around the origin. Specifically, the \( x \) and \( y \) coordinates of the line's endpoint are calculated using the cosine and sine of the rotation angle, respectively. 

This corresponds to the following transformation:


```math
\begin{pmatrix}
x' \\
y'
\end{pmatrix}
=
\begin{pmatrix}
\cos(\theta) & -\sin(\theta) \\
\sin(\theta) & \cos(\theta)
\end{pmatrix}
\begin{pmatrix}
x \\
y
\end{pmatrix}
```

## How to Run

To compile and run the project, use the following command:

```bash
gcc rotations.c -o rotations -framework OpenGL -framework GLUT -Wno-deprecated-declarations -lm
```

This will produce an executable named `rotations`. Run it to see the rotating line in action.
