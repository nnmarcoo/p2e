#Regression of p-dimensional Function (Image processing)
import numpy as np
import random
from scipy.optimize import curve_fit
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
initial_guess = [1,.5,0,.02,.5,1]#The initial guess is arbitrary but still vitally important, as our model progresses we can have a more efficient starting point
learning_rate = 1e-5#Known mathematically as "alpha" this is a constant that we apply to our weight (obtained from gradient function) that affects how big of a jump we make in each iterative guess
max_iters = 10000#Maximum iterations, just for safety
max_terms = 500#I dont want longer than 500 parameters for a single equation

#Model Function: We input our data "x" and our any number of number of terms for combinations of sinusoids and polynomials
#The output of this function is a new model which will be tested for accuracy in "cost_function"
def model(x, *params):
    num_sinusoids = len(params) // 2  # Half the parameters are for sinusoids (amplitude, frequency, phase)
    sinusoids = np.sum(params[:num_sinusoids*3:3] * np.sin(params[1:num_sinusoids*3:3] * x + params[2:num_sinusoids*3:3]), axis=0)
    polynomial = np.sum(params[num_sinusoids*3:] * x**(np.arange(len(params[num_sinusoids*3:]))), axis=0)
    return sinusoids + polynomial

#Gradient Function: This essentially gives us a weight for how much we need to change our parameters
def gradient(x, y, model, params):
    y_pred = model(x, params)
    error = y - y_pred
    grad = np.zeros_like(params)
    
    # Split parameters into sinusoidal and polynomial components
    num_sinusoids = len(params) // 2
    sin_params = params[:num_sinusoids*3]
    poly_params = params[num_sinusoids*3:]
    
    # Gradient for sinusoids: (a_i, b_i, c_i) parameters
    for i in range(0, len(sin_params), 3):
        amplitude = sin_params[i]
        frequency = sin_params[i+1]
        phase = sin_params[i+2]
        
        # Gradient w.r.t. amplitude, frequency, and phase
        grad[i] = -2 * np.sum(error * np.sin(frequency * x + phase))  # w.r.t amplitude
        grad[i+1] = -2 * np.sum(error * amplitude * x * np.cos(frequency * x + phase))  # w.r.t frequency
        grad[i+2] = -2 * np.sum(error * amplitude * np.cos(frequency * x + phase))  # w.r.t phase
    
    # Gradient for polynomial coefficients
    for i in range(len(poly_params)):
        grad[num_sinusoids*3 + i] = -2 * np.sum(error * x**i)
    
    return grad

#Cost Function: This is what we are attempting to minimize, the RMS of every point of our model's curve
def cost_function(x, y, model, params):
    y_hat = model(x, params)
    error = y - y_hat
    return np.mean(error**2)

#Gradient Descent: We begin to follow the weights given by gradient function and modify parameters
def gradient_descent(x, y, model, initial_params, learning_rate=1e-5, max_iters=10000, error_threshold=1e-20):
    params = np.array(initial_params)
    prev_cost = float('inf')
    
    for iteration in range(max_iters):
        # Compute the gradient
        grad = gradient(x, y, model, params)
        # Update parameters
        params -= learning_rate * grad
        
        # Compute the current cost
        current_cost = cost_function(x, y, model, params)
        
        # Print the cost every 100 iterations
        if iteration % 100 == 0:
            print(f"Iteration {iteration}, Cost: {current_cost}")
        
        # Check for convergence
        if abs(prev_cost - current_cost) < error_threshold:
            print(f"Converged after {iteration} iterations")
            break
        
        prev_cost = current_cost
    
    return params

#Add new parameters as necessary
def add_terms(params, num_new_terms=3):
    for n in range(num_new_terms):
        params += [np.random.randn(), np.random.randn(), np.random.randn()]  # Add random amplitude, frequency, phase for sinusoids
        params.append(np.random.randn())  # Add random coefficient for polynomial term
    return params

#The Training
def train(x, y, initial_params, error_threshold=1e-20, max_iterations=1000, max_terms=500):
    params = initial_params
    error = cost_function(x, y, model, params)
    iteration = 0
    
    # Iteratively add terms and fit until the error is sufficiently small
    while error > error_threshold and iteration < max_iterations:
        
        # Perform gradient descent to fit the current model
        params = gradient_descent(x, y, model, params)
        
        # Check the current error
        error = cost_function(x, y, model, params)
        
        # Add more terms if the error is still too large
        if iteration % 10 == 0 and len(params) < max_terms:
            params = add_terms(params, num_new_terms=3)  # Add 3 more terms
        
        iteration += 1
    
    return params, error

#send it
optimized_params, final_error = train(len(R), R, initial_guess)