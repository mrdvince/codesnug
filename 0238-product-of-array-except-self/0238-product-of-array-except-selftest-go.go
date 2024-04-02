func productExceptSelf(nums []int) []int {
    res := make([]int,len(nums))
    prefix := 1
    for idx, val := range nums {
        res[idx] = prefix
        prefix *= val
    }
    postfix := 1
    for idx := len(nums) - 1; idx >= 0; idx--{
        res[idx] *= postfix 
        postfix *= nums[idx]
    }
    return res
}