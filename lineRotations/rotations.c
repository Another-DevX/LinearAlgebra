#include <OpenGL/gl.h>  // Header for OpenGL functionalities
#include <OpenGL/glu.h> // Header for OpenGL utilities
#include <GLUT/glut.h>  // Header for GLUT library
#include <stdio.h>      // Header for standard I/O functions
#include <math.h>       // Header for math functions

float angle = 0.0; // Global variable to store the rotation angle

// Display function, where all the drawing takes place
void display() {
    glClear(GL_COLOR_BUFFER_BIT);  // Clear the color buffer
    glBegin(GL_LINES);  // Start drawing lines
    glVertex2f(0, 0);  // Start point of the line at the origin
    // End point of the line, calculated using trigonometry
    glVertex2f(cos(angle), sin(angle));  
    glEnd();  // End drawing lines
    glFlush();  // Flush the OpenGL buffers
}

// Idle function, updates when no events to handle
void idle() {
    angle += 0.1;  // Increment the angle
    // Reset the angle if it exceeds 2*PI
    if (angle > 2 * M_PI) angle -= 2 * M_PI;  
    glutPostRedisplay();  // Request to redraw the scene
}

// Main function
int main(int argc, char **argv) {
    glutInit(&argc, argv);  // Initialize GLUT
    glutInitDisplayMode(GLUT_SINGLE);  // Set up a basic display buffer
    glutInitWindowSize(500, 500);  // Initial window size
    glutCreateWindow("Line in OpenGL");  // Create a window with a title
    glutDisplayFunc(display);  // Set the display function
    glutIdleFunc(idle);  // Set the idle function
    glutMainLoop();  // Enter the GLUT event loop
    return 0;
}
