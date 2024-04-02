from functools import reduce
import operator

class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
#   time limit exceeds      
#         ans = []
#         n = len(nums)
#         arr = []
#         for i in range(n):
#             n_arr = nums[:0+i] + nums[1+i:]
#             arr.append(n_arr)
#         for i in arr:
#             ans.append(reduce(operator.mul, i))
#         return ans
        res = [1] * len(nums)
        print(res)
        p = 1
        for i in range(len(nums)):
            res[i] *= p
            p *= nums[i]
        print(res)
        p = 1
        for i in range(len(nums)-1, -1, -1):
            res[i] *= p
            p *= nums[i]
        print(res)
        return res
#         pre = list(accumulate(nums, operator.mul))
#         suf = list(accumulate(nums[::-1], operator.mul))[::-1]
#         n = len(nums)
#         ans = []
#         for i in range(n):
#             if i:
#                 pre_num = pre[i-1]
#             else:
#                 pre_num = 1
#             if (i + 1 < n):
#                 suf_num = suf[i+1]
#             else:
#                 suf_num = 1
            
#             ans.append(pre_num * suf_num)
#         return ans


#         n_list = []
#         for i in range(len(nums)):
            

#         res = []
#         # O(n^2)
#         if not nums: return False
#         for idx, num in enumerate(nums):
#             n_arr = [x for x in nums if x != num]
#             n_sum = reduce(operator.mul, n_arr,1)
#             res.append(n_sum)
#         return res