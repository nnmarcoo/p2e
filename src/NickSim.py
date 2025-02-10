#Regression of p-dimensional Function (Image processing)
import numpy as np
import random 
#Nicholas Sydorenko
#Each Pixel Has 5 parameters: n,m,R,G,B
#R,G,B can all be split into three functions so we have 3, 3 dimensional funtions of r(n,m,R), g(n,m,G), b(n,m,B)

#Simulate an image as 3 matrices of RGB values with identical sizes
n=8
m=8

#define function to fill image with random integers for RGB
def fill_img(n,m,min,max):
    return np.random.randint(min,max+1, size = (n,m))

#Create RGB Matrices with Random Seeds for replicability
random.seed(0)
R = fill_img(n,m,0,255)
random.seed(1)
G = fill_img(n,m,0,255)
random.seed(2)
B = fill_img(n,m,0,255)