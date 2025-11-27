cs python:
    import numpy as np
    
    def matrix_multiply(a, b):
        return np.dot(a, b).tolist()
    
    def analyze_data(data):
        arr = np.array(data)
        return {
            'mean': float(np.mean(arr)),
            'std': float(np.std(arr)),
            'min': float(np.min(arr)),
            'max': float(np.max(arr))
        }

mn main():
    matrix_a = [[1, 2], [3, 4]]
    matrix_b = [[5, 6], [7, 8]]
    result = python.matrix_multiply(matrix_a, matrix_b)
    print("Matrix product:", result)
    
    data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    stats = python.analyze_data(data)
    print("Statistics:", stats)
