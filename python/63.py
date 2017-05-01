class Solution(object):
    def uniquePathsWithObstacles(self, obstacleGrid):
        
        mp={}
        if obstacleGrid[0][0] == 0:
            mp[(0,0)] =1
        for x in range(len(obstacleGrid)):
            for y in range(len(obstacleGrid[0])):
                if obstacleGrid[x][y] == 0:
                    if x == 0 and y ==0:
                        continue
                    if x == 0:
                        mp[(x,y)] = mp[(x,y-1)]
                        continue
                    if y == 0:
                        mp[(x,y)] = mp[(x-1,y)]
                        continue
                    else:
                        mp[(x,y)] = mp[(x-1,y)] + mp[(x,y-1)]
                else:
                    mp[(x,y)] = 0
        return mp[len(obstacleGrid)-1,len(obstacleGrid[0])-1]
