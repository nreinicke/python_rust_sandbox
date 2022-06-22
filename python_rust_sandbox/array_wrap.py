import numpy as np

def _array_wrap_to_numpy(self):
    return np.array(self.to_list())