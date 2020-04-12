class Solution:
    def minimumTotal(self, triangle: List[List[int]]) -> int:
        if len(triangle) == 0:
            return 0
        if len(triangle) == 1:
            return triangle[0][0]
        minpath = 0
        for x in range(1, len(triangle)):
            for y in range(len(triangle[x])):
                if y == 0:
                    triangle[x][y] = triangle[x][y] + triangle[x-1][y]
                elif y == len(triangle[x]) - 1:
                    triangle[x][y] = triangle[x][y] + triangle[x-1][y-1]
                else:
                    triangle[x][y] = triangle[x][y] + \
                        min(triangle[x-1][y], triangle[x-1][y-1])
                minpath = min(triangle[x])
        return minpath
