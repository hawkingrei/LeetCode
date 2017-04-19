class Solution:
    def parse(self, digit, index):
        NUMS = {
            1: 'I',
            2: 'II',
            3: 'III',
            4: 'IV',
            5: 'V',
            6: 'VI',
            7: 'VII',
            8: 'VIII',
            9: 'IX',
        }
        ROMAN = {
            'I': ['I', 'X', 'C', 'M'],
            'V': ['V', 'L', 'D', '?'],
            'X': ['X', 'C', 'M', '?']
        }
        
        s = NUMS[digit]
        return s.replace('X', ROMAN['X'][index]).replace('I', ROMAN['I'][index]).replace('V', ROMAN['V'][index])
        
    def intToRoman(self, num):
        s = ''
        index = 0
        while num != 0:
            digit = num % 10
            if digit != 0:
                s = self.parse(digit, index) + s
            num = num / 10
            index += 1
        return s
