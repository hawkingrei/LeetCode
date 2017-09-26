class Solution(object):
    def findRedundantConnection(self, edges):
        """
        :type edges: List[List[int]]
        :rtype: List[int]
        """
        f = [-1] * 2001
        def father(i):
            if f[i] == -1:
                return i
            f[i] = father(f[i])
            return f[i]
        for u, v in edges:
            fu = father(u)
            fv = father(v)
            if fu == fv:
                return [u, v]
            f[fv] = fu
