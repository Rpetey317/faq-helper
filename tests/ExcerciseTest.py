from FaQHelper.Excercise import Excercise
import unittest

class ExcerciseTest(unittest.TestCase):
    def test_01_excercise_belongs_to_assignature(self):
        exc = Excercise(assignature='Análisis Matemático 2', exc_path='exc1.md')
        self.assertEqual(exc.assignature(), 'Análisis Matemático 2')

    def test_02_excercise_can_be_tagged(self):
        exc = Excercise(assignature='Análisis Matemático 2', exc_path='exc1.md', tags=['Integral'])
        self.assertTrue(exc.has_tag('Integral'))
    
    def test_03_can_add_tags_to_excercise(self):
        exc = Excercise(assignature='Análisis Matemático 2', exc_path='exc1.md', tags=['Integral'])
        self.assertFalse(exc.has_tag('Derivada'))
        exc.add_tag('Derivada')
        self.assertTrue(exc.has_tag('Derivada'))

    def test_04_can_remove_tags_from_excercise(self):
        exc = Excercise(assignature='Análisis Matemático 2', exc_path='exc1.md', tags=['Integral', 'Derivada'])
        self.assertTrue(exc.has_tag('Integral'))
        exc.remove_tag('Integral')
        self.assertFalse(exc.has_tag('Integral'))

    def test_05_excercise_has_path(self):
        exc = Excercise(assignature='Análisis Matemático 2', exc_path='exc1.md')
        self.assertEqual(exc.path(), 'exc1.md')

if __name__ == '__main__':
    unittest.main()
