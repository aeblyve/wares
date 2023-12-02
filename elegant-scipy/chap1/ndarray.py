import numpy as np

gene0 = [100, 200]
gene1 = [50, 0]
gene2 = [350, 100]
expression_data = [gene0, gene1, gene2]

array1d = np.array([1,2,3,4])
print(array1d)
print(type(array1d))
print(array1d.shape)

array2d=np.array(expression_data)
print(array2d)
print(type(array2d))
print(array2d.shape)
print(array2d.ndim)
