// Công ty xây dựng SVI phải lựa chọn các dự án cần thực hiện để lợi nhuận đem lại là nhiều nhất.
// Công ty có một danh sách gồm n dự án đánh số từ 1 đến n. Sau khi công ty rà soát năng lực thực hiện các dự án,
// công ty đưa ra bảng đánh giá hiệu quả (có thể là lợi nhuận hoặc thua lỗ) từ việc thực hiện dự án i là p[i]
// (nếu p[i] > 0 đó là lợi nhuận, ngược lại nếu p[i] < 0 thì đó là thua lỗ phải chịu từ việc thực hiện dự án i, |p[i]| < 1e6).

// Việc lựa chọn các dự án cần thực hiện để lợi nhuận đem lại là lớn nhất không phải là đơn giản,
// bởi vì công ty không thể chỉ lựa chọn các công việc đem lại lợi nhuận để thực hiện.
// Có một danh sách gồm m điều kiện liên quan đến việc lựa chọn thực hiện các dự án. Điều kiện thứ j yêu cầu:
// Nếu thực hiện dự án u[j] thì phải thực hiện dự án v[j], j = 1,2,...,m

// Một tập con các dự án được gọi là lựa chọn được nếu mỗi dự án trong nó luôn thỏa mãn các điều kiện trong danh sách.

// Hãy giúp công ty tìm tập các dự án lựa chọn được mà việc thực hiện chúng đem lại tổng hiệu quả lớn nhất.

// Input
// Dòng đầu ghi một số nguyên dương n (n <= 500)
// Dòng thứ hai ghi n số nguyên tương ứng là tính hiệu quả của từng công việc
// Dòng thứ bai ghi một số nguyên dương m (m <= 1e4)
// Dòng thứ j trong số m dòng tiếp theo ghi hai số nguyên dương u[j] và v[j] chỉ sự ràng buộc rằng
//     nếu thực hiện công việc u[j]thì phải thực hiện công việc v[j]

// Output
// Ghi ra một số nguyên là tổng hiệu quả của tập các dự có thể án thực hiện tìm được. Ghi ra số 0 nếu như không chọn dự án nảo cả.

// Sample Input
// 6
// 10 4 -5 3 -1 -2
// 4
// 2 3
// 2 5
// 6 5
// 4 3

// Sample Output
// 11

// Để giải được bài này, cần học:
// Cơ bản về đồ thị: Biểu diễn đồ thị, DFS, BFS
// Luồng cực đại/cắt cực tiểu: Thuật toán Ford-Fulkerson hoặc Dinic
// Biến đổi bài toán thành mô hình luồng: Cách xây dựng đồ thị luồng từ bài toán thực tế

// Các bước tiến tới giải bài này:
// [x] https://leetcode.com/problems/flower-planting-with-no-adjacent/description/ (Bài tập về tô màu đồ thị, dễ)
// [x] https://leetcode.com/problems/network-delay-time/description/ (Bài về đường đi ngắn nhất)
// [] https://leetcode.com/problems/course-schedule-iv/description/ (Đồ thị có hướng và ràng buộc)
// [] https://leetcode.com/problems/parallel-courses-iii/description/ (DAG và tối ưu)
// [] https://leetcode.com/problems/critical-connections-in-a-network/description/ (Cầu trong đồ thị)
// [] https://wiki.vnoi.info/algo/graph-theory/flow (Lý thuyết về Max flow)
// [] https://oj.vnoi.info/problem/flow1 (Bài mẫu để xử lý vấn đề Max flow)
// [] https://leetcode.com/problems/maximum-students-taking-exam/description/ (Thêm một bài leetcode nữa về Max flow)

fn main() {}
