#include <OpenGL/gl.h>
#include <OpenGL/glu.h>
#include <GLUT/glut.h>
#include <stdio.h>
#include <math.h>

float angle = 0.0;

void display(){
  glClear(GL_COLOR_BUFFER_BIT);
  glBegin(GL_LINES);
  glVertex2f(0,0);
  glVertex2f(cos(angle), sin(angle));
  glEnd();
  glFlush();
}

void idle (){
  angle += 0.1;
  if(angle > 2 * M_PI){
    angle -= 2 * M_PI;
  }

  glutPostRedisplay();
}

int main(int argc, char**argv){
  glutInit(&argc, argv);
  glutInitDisplayMode(GLUT_SINGLE);
  glutInitWindowSize(500,500);
  glutCreateWindow("Linea en OpenGL");
  glutDisplayFunc(display);
  glutIdleFunc(idle);
  glutMainLoop();
  return 0;
}
