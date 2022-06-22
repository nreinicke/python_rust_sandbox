from .python_rust_sandbox import *
from python_rust_sandbox.array_wrap import _array_wrap_to_numpy

setattr(ArrayWrap, "__array__", _array_wrap_to_numpy)
