class Solution {
public:
    vector<int> pascal(vector<int> previous, int level) {
        if (previous.size() == level + 1) {
            return previous;
        } else {
            vector<int> row {1};
            for (int i = 1; i < previous.size(); i++) {
                row.push_back(previous[i-1] + previous[i]);
            }
            row.push_back(1);
            //row.push_back(previous[previous.size() - 1] + 1);
            return pascal(row, level);
        }
    }
    vector<int> getRow(int rowIndex) {
        vector<int> start {1};
        return pascal(start, rowIndex);
    }
};
