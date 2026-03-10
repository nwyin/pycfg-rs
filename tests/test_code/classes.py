"""Class constructs — methods, nested classes, special methods."""


class Simple:
    def __init__(self, x):
        self.x = x

    def get_x(self):
        return self.x


class WithClassMethod:
    @classmethod
    def create(cls, value):
        return cls(value)

    @staticmethod
    def validate(value):
        if value < 0:
            raise ValueError("negative")
        return True


class Nested:
    class Inner:
        def inner_method(self):
            return 42

    def outer_method(self):
        return self.Inner().inner_method()


class WithProperties:
    def __init__(self, value):
        self._value = value

    @property
    def value(self):
        return self._value

    @value.setter
    def value(self, new_value):
        if new_value < 0:
            raise ValueError("negative")
        self._value = new_value
