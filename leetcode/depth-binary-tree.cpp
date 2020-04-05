/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    int maxDepth(TreeNode* root) {
        if (root == NULL)
            return 0;
        
        int left_depth = 0;
        if (root -> left != NULL)
            left_depth = maxDepth(root -> left);
        
        int right_depth = 0;
        if (root -> right != NULL)
            right_depth = maxDepth(root -> right);
        
        if (left_depth > right_depth)
            return left_depth + 1;
        else
            return right_depth + 1;
    }
};
