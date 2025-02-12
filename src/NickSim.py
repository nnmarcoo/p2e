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

#Nick's Proposed method #1 (flatten the matrix and specify to "compressor" how many pixels per row)
R = R.flatten()
G = G.flatten()
B = B.flatten()

error_threshold = 1e-20
initial_guess = [1,.5,0,0,.5,1,1,.5,2]#The initial guess is arbitrary but still vitally important, as our model progresses we can have a more efficient starting point
zero_error = np.zeros(len(R))
print(zero_error)

#Learn Function: We input our data "x" and our any number of number of terms for combinations of sinusoids and polynomials
#The output of this function is a new model which will be tested for accuracy in "test"
def learn(x, *params):
    num_sinusoids = len(params) // 2  # Half the terms are for sinusoids
    sinusoids = sum([params[i] * np.sin(params[i+1] * x + params[i+2]) for i in range(0, num_sinusoids * 3, 3)])
    polynomial = sum([params[i+num_sinusoids*3] * x**(i//3) for i in range(num_sinusoids*3, len(params))])
    return sinusoids + polynomial

#Test Function: We have recieved a new model from "learn". We will now take the RMS of every point and find the mean and return it.
#This error is what we want to minimize in order to achieve 100% accuracy of our equation
def test(x,y,new_model,*params):
    y_hat = new_model(x,*params)
    error = np.mean((y-y_hat)**2)
    return error

params_opt = initial_guess
test_error = test(len(R),R)
errors = [test_error]
error = test(len(R),R)

#The Training

