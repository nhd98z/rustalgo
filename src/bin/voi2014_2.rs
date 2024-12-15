use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn solve_one(m: usize, n: usize, a: Vec<usize>, b: Vec<usize>) {
    // dp[j] = độ dài dãy con chung thỏa mãn điều kiện 2 * c[i] <= c[i+1],
    //          mà kết thúc tại phần tử b[j].
    //
    // Nói cách khác, nếu ta nhìn vào dãy con chung đã tìm được, phần tử cuối cùng là b[j],
    // dp[j] cho biết độ dài tối đa của dãy đó.
    let mut dp = vec![0; n];

    // Ý tưởng chính:
    // Chúng ta muốn tìm một dãy con chung giữa a và b, sao cho nếu dãy có dạng c[1], c[2], ...,
    // thì luôn thỏa mãn 2 * c[i] <= c[i+1].
    //
    // Cách tiếp cận:
    // - Ta sẽ duyệt qua từng phần tử a[i] của mảng a.
    // - Ứng với mỗi a[i], ta sẽ thử "mở rộng" những dãy con chung đã có trong dp bằng cách
    //   kiểm tra xem a[i] có thể trở thành phần tử tiếp theo trong dãy đó hay không.
    //
    // Để làm được, ta cần một biến trợ giúp: curr.
    // curr sẽ lưu trữ độ dài LCS2X lớn nhất trong phạm vi đã xem xét, mà a[i] có thể nối thêm vào.
    // Cụ thể, "nối thêm" ở đây nghĩa là:
    //    Nếu ta đã có một dãy con c[] kết thúc bằng một phần tử nào đó thỏa mãn điều kiện bội 2,
    //    và a[i] có thể là phần tử kế tiếp (tức là a[i] ≥ 2 * (phần tử cuối)),
    //    thì curr sẽ giữ độ dài lớn nhất của dãy như thế.
    //
    // Sau khi xác định được curr (độ dài dãy con tốt nhất có thể nối),
    // nếu a[i] cũng xuất hiện trong b (tức là a[i] == b[j] cho một j nào đó),
    // ta có thể thực sự gắn a[i] vào dãy con kết thúc ở b[j], tạo thành một dãy con mới dài hơn.

    for i in 0..m {
        // curr: độ dài lớn nhất của một dãy con chung bội 2 có thể nối thêm bằng a[i].
        // Ban đầu, ta chưa xét gì nên curr = 0.
        let mut curr = 0;

        for j in 0..n {
            // prev lưu lại giá trị curr trước khi cập nhật bởi b[j].
            // prev sẽ giúp ta nắm được "độ dài tốt nhất" ngay trước khi xem xét b[j].
            let prev = curr;

            // Nếu 2 * b[j] ≤ a[i], điều này có nghĩa a[i] có thể "nối tiếp" phần tử b[j]
            // trong một dãy con bội 2. Tức là, nếu ta có một dãy con chung kết thúc tại b[j],
            // thì a[i] (chính là phần tử mới của mảng a mà ta đang xét) có thể trở thành phần tử tiếp theo.
            //
            // Việc cập nhật curr = max(curr, dp[j]) nghĩa là:
            // "Trong số các dãy con mà ta có thể nối thêm a[i],
            // ta đang tìm ra dãy con tốt nhất từng kết thúc tại một phần tử nào đó nhỏ hơn hoặc bằng a[i]/2,
            // và dp[j] là ứng viên như vậy. Do đó, nếu dp[j] lớn hơn curr,
            // ta chọn dp[j] làm ứng viên tốt nhất hiện tại."
            //
            // Nói cách khác, dp[j] là độ dài dãy con bội 2 kết thúc ở b[j], và vì a[i] ≥ 2 * b[j],
            // a[i] có thể trở thành phần tử kế tiếp. Ta cập nhật curr để ghi nhận rằng
            // "dãy con tốt nhất ta có thể mở rộng bằng a[i]" ít nhất có độ dài dp[j]."
            if 2 * b[j] <= a[i] {
                curr = usize::max(curr, dp[j]);
            }

            // Nếu a[i] == b[j], nghĩa là a[i] cũng là b[j],
            // ta có thể thực sự thêm a[i] vào dãy con.
            // Khi đó, chiều dài dãy mới = prev + 1 (prev là độ dài tốt nhất
            // mà ta có thể nối thêm a[i] trước đó).
            // dp[j] = max(dp[j], prev + 1) vì chúng ta muốn giữ kết quả tốt nhất cho dãy kết thúc ở b[j].
            if a[i] == b[j] {
                dp[j] = usize::max(dp[j], prev + 1);
            }
        }
    }

    // Cuối cùng, kết quả là độ dài lớn nhất trong dp,
    // tức là độ dài dài nhất của dãy con chung bội 2 tìm được.
    let res = dp.into_iter().max().unwrap();
    println!("{}", res);
}

fn solve<R: BufRead>(reader: &mut R) {
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    let testcase: usize = s.trim().parse().unwrap();
    for _ in 0..testcase {
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        let m: usize = iter.next().unwrap().parse().unwrap();
        let n: usize = iter.next().unwrap().parse().unwrap();
        s.clear();
        reader.read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        let mut a = vec![0; m];
        for i in 0..m {
            a[i] = iter.next().unwrap().parse().unwrap();
        }
        s.clear();
        reader.read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        let mut b = vec![0; n];
        for j in 0..n {
            b[j] = iter.next().unwrap().parse().unwrap();
        }
        solve_one(m, n, a, b);
    }
}

fn get_reader() -> Box<dyn BufRead> {
    if env::var("USER").unwrap_or_default() == "nhd98z" {
        let path = format!(
            "src/bin/{}.txt",
            Path::new(file!()).file_stem().unwrap().to_str().unwrap()
        );
        match File::open(&path) {
            Ok(file) => Box::new(BufReader::new(file)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                File::create(&path).expect(&format!("Failed to create input file: {}", &path));
                panic!("Input file not found. An empty file has been created.");
            }
            Err(e) => {
                panic!("Failed to open input file '{}': {}", &path, e);
            }
        }
    } else {
        Box::new(BufReader::new(io::stdin()))
    }
}

fn main() {
    let mut reader = get_reader();
    solve(&mut reader);
}
