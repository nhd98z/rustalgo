#![allow(dead_code)]

impl Solution {
    /// # Mô tả:
    /// Hàm `last_remaining(n)` trả về số cuối cùng còn lại sau khi liên tục
    /// loại bỏ các số ở vị trí lẻ (lượt 1 từ trái sang phải, lượt 2 từ phải sang trái, ...)
    /// cho đến khi chỉ còn lại 1 số.
    ///
    /// # Công thức đệ quy:
    /// last_remaining(n) = 2 * ( (n/2) + 1 - last_remaining(n/2) )
    ///
    /// # Giải thích ngắn gọn:
    /// - Sau lượt loại bỏ đầu tiên (trái → phải), ta còn lại n/2 số chẵn: [2, 4, 6, ..., 2*(n/2)].
    ///   Nếu ta "co" dãy chẵn này thành [1..(n/2)] (bằng cách chia mỗi phần tử cho 2),
    ///   thì ta có bài toán con `last_remaining(n/2)`.
    /// - Tuy nhiên, vì bước tiếp theo trong "thế giới ban đầu" là phải → trái,
    ///   nên vị trí cuối cùng trong bài toán con cần được “phản chiếu” (mirroring).
    ///   Phép tính (n/2 + 1 - last_remaining(n/2)) chính là cách “lật ngược” vị trí
    ///   theo hướng mới (thay vì tiếp tục trái → phải, ta đang ở phải → trái).
    /// - Sau cùng, nhân với 2 để quy đổi vị trí trong dãy [1..(n/2)] trở lại
    ///   dãy chẵn [2, 4, 6, ..., 2*(n/2)] của thế giới ban đầu.
    ///
    /// # Ví dụ:
    /// - n = 6:
    ///   - Danh sách: [1, 2, 3, 4, 5, 6]
    ///   - Bước 1 (Trái→Phải): bỏ 1, 3, 5 → còn [2, 4, 6]
    ///   - Bước 2 (Phải→Trái): bỏ 6, 2 → còn [4] → Kết quả cuối: 4
    ///   - Theo công thức: last_remaining(6) = 2 * (3 + 1 - last_remaining(3)) = 4
    ///
    /// # Thời gian chạy:
    /// - O(log n) vì mỗi bước n giảm khoảng một nửa.
    ///
    /// # Tham số:
    /// - `n`: số nguyên dương (số phần tử ban đầu).
    ///
    /// # Trả về:
    /// - Số cuối cùng còn lại sau quá trình loại bỏ.
    pub fn last_remaining(n: i32) -> i32 {
        // Trường hợp cơ bản: nếu chỉ còn 1 phần tử thì đó chính là đáp án.
        if n == 1 {
            return 1;
        }

        // Công thức đệ quy:
        // - Sau lượt loại bỏ đầu, còn n/2 số (đều là chẵn).
        //   -> Bài toán con last_remaining(n/2) tương ứng với dãy [1..(n/2)].
        // - Vì bước kế tiếp là phải → trái trong dãy gốc,
        //   ta "phản chiếu" vị trí từ bài toán con qua (n/2 + 1 - last_remaining(n/2)).
        // - Nhân với 2 để quay lại khoảng giá trị ban đầu (các số chẵn).
        2 * (n / 2 + 1 - Self::last_remaining(n / 2))
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::last_remaining(9), 6);
    assert_eq!(Solution::last_remaining(1), 1);
}
