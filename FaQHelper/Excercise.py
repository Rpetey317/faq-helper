class Excercise:
    def __init__(self, assignature, exc_path, tags=[]) -> None:
        self.__assignature__ = assignature
        self.__tags__ = tags
        self.__path__ = exc_path

    def assignature(self):
        return self.__assignature__

    def has_tag(self, tag):
        return tag in self.__tags__
    
    def add_tag(self, tag):
        self.__tags__.append(tag)
        return self
    
    def remove_tag(self, tag):
        self.__tags__.remove(tag)
        return self

    def path(self):
        return self.__path__
