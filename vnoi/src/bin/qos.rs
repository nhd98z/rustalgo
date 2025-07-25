// Bài toán định tuyến kèm theo chất lượng dịch vụ bảo đảm trong các ứng dụng đa phương tiện như tuyến tính hình ảnh và âm thanh theo yêu cầu là vấn đề thời sự trong những năm gần đây. Trong bài toán này, chúng ta quan tâm đến độ trễ của các đường truyền tin.

// Công ty cung cấp dịch vụ mạng ESI vừa thiết lập một mạng truyền thông giữa các điểm cung cấp dịch vụ và khách hàng, bao gồm n nút và m kênh nối trực tiếp một chiều giữa hai nút. Các nút được đánh số từ
// 1 đến n, trong đó nút 1 là nút nguồn. Các kênh nối được đánh số từ  1 đến m. kênh nối thứ i cho phép truyền tin (một chiều) từ nút ui tới nút vi và có độ trễ là c(ui, vi). Có không quá một kênh truyền tin từ một nút đến một nút khác.

// Một đường truyền tin từ nút nguồn đến nút t được biểu diễn dưới dạng một dãy liên tiếp các chỉ số của các nút, xuất phát từ 1 và kết thúc tại t. Độ trễ của đường truyền tin được định nghĩa là tổng độ trễ của các kênh nối trực tiếp trên đường truyền tin đó. Để khảo sát các đường truyền tin từ nút nguồn đến một nút trong mạng, công ty ESI xác định c_min là độ trễ nhỏ nhất trong tất cả các độ trễ của các kênh trong mạng và t_min là độ trễ của đường truyền tin từ nút nguồn đến nút t với độ trễ nhỏ nhất.

// Để đảm bảo dịch vụ đường truyền với chất lượng cao, đường truyền tin từ nút nguồn đến nút t phải thỏa mãn điều kiện QoS (Quality of Service) sau đây: độ trễ của đường truyền tin phải nhỏ hơn hoặc bằng tổng số
// t_min + c_min. Sau đó ESI sắp xếp tất cả các đường truyền tin từ nút nguồn đến nút t thỏa mãn điều kiện QoS theo thứ tự từ điển. Cho trước số nguyên dương k, hãy tìm đường truyền tin từ 1 đến t thỏa mãn điều kiện QoS thứ k trong thứ tự từ điển.

// Input
// Dòng đầu tiên chứa bốn số nguyên dương n, m, t, k (k <= 1e9)
// Dòng thứ i trong số m dòng tiếp theo ghi ba số nguyên dương ui, vi, c(ui, vi) lần lượt là chỉ số đầu, chỉ số cuối và độ trễ của kênh thứ i. Độ trễ của các kênh là nhỏ hơn 100.

// Output
// Ghi ra -1 nếu không tìm được đường truyền tin thỏa mãn yêu cầu đặt ra, trái lại cần ghi thông tin về đường truyền tin tìm được bao gồm:

// Dòng đầu ghi số nguyên dương  s là số lượng nút trên đường truyền tìm được;
// Dòng thứ hai ghi s số lần lượt là chỉ số của các nút theo thứ tự mà đường truyền tìm được đi qua, bắt đầu từ nút 1 kết thúc ở nút t.

use std::io::{BufRead, BufReader};

// Hằng số
// Giới hạn đếm số đường đi tối đa để tránh tràn số
const MAX_COUNT_CAP: usize = 1_000_000_000 + 5;
// Số nút tối đa trong đồ thị
const MAX_NODES: usize = 1000;
// Giới hạn "extra_delay" tối đa
const MAX_EXTRA_DELAY: usize = 105;
// Giá trị vô cùng để khởi tạo khoảng cách
const INFINITY_VALUE: usize = 0x3f3f3f3f;

// Cấu trúc lưu trữ cạnh: nút đích và độ trễ
#[derive(Clone)]
struct Edge {
    destination: usize,
    delay: usize,
}

// Biến toàn cục
// Bảng QHĐ: path_count[node][extra_delay] = số đường đi từ node đến đích
static mut PATH_COUNT: [[i32; MAX_EXTRA_DELAY]; MAX_NODES + 1] = [[-1; MAX_EXTRA_DELAY]; MAX_NODES + 1];
// Khoảng cách ngắn nhất từ mỗi nút đến đích
static mut MIN_DISTANCE: [usize; MAX_NODES + 1] = [INFINITY_VALUE; MAX_NODES + 1];

// Đồ thị theo hướng thuận và ngược
static mut FORWARD_GRAPH: Vec<Vec<Edge>> = Vec::new(); // G[u] -> danh sách cạnh ra từ u
static mut REVERSE_GRAPH: Vec<Vec<Edge>> = Vec::new(); // R[v] -> danh sách cạnh vào v

// Các biến config
static mut NODE_COUNT: usize = 0;
static mut EDGE_COUNT: usize = 0;
static mut TARGET_NODE: usize = 0;
static mut KTH_PATH: usize = 0;
static mut MAX_ALLOWED_DELAY: usize = INFINITY_VALUE;

/// Hàm đếm số đường đi từ current_node đến TARGET_NODE với "độ trễ dư" extra_delay
/// Sử dụng quy hoạch động có nhớ
unsafe fn count_paths(current_node: usize, extra_delay: i32) -> i32 {
    // Điều kiện cơ sở 1: Nếu tổng độ trễ vượt quá giới hạn QoS hoặc extra_delay âm
    if extra_delay < 0 {
        return 0;
    }
    let total_delay = unsafe { (extra_delay as usize) + MIN_DISTANCE[current_node] };
    if unsafe { total_delay > MAX_ALLOWED_DELAY } {
        return 0;
    }

    // Điều kiện cơ sở 2: Đã đến đích và extra_delay = 0 (đi theo đường ngắn nhất)
    if unsafe { current_node == TARGET_NODE } && extra_delay == 0 {
        return 1;
    }

    // Memoization: Kiểm tra nếu đã tính trạng thái này rồi
    if unsafe { PATH_COUNT[current_node][extra_delay as usize] != -1 } {
        return unsafe { PATH_COUNT[current_node][extra_delay as usize] };
    }

    // Khởi tạo số đường đi cho trạng thái hiện tại
    unsafe { PATH_COUNT[current_node][extra_delay as usize] = 0 };

    // Duyệt tất cả các cạnh đi ra từ nút hiện tại
    for edge in unsafe { &FORWARD_GRAPH[current_node] } {
        let next_node = edge.destination;
        let edge_delay = edge.delay;

        // Nếu nút kế tiếp có đường đi đến đích
        if unsafe { MIN_DISTANCE[next_node] != INFINITY_VALUE } {
            // Tính "độ trễ dư" mới khi đi qua cạnh này
            // Giải thích:
            // - MIN_DISTANCE[current_node] = Độ trễ tối thiểu từ current_node đến đích
            // - MIN_DISTANCE[next_node] = Độ trễ tối thiểu từ next_node đến đích
            // - edge_delay = Độ trễ từ current_node đến next_node
            // => Chênh lệch = MIN_DISTANCE[current_node] - MIN_DISTANCE[next_node] - edge_delay
            //    Nếu dương: đường dài hơn đường ngắn nhất
            //    Nếu âm: đường ngắn hơn (không thể xảy ra vì MIN_DISTANCE đã tối ưu)
            let next_extra_delay = extra_delay as i64 + unsafe { MIN_DISTANCE[current_node] } as i64
                - unsafe { MIN_DISTANCE[next_node] } as i64
                - edge_delay as i64;

            // Chỉ tiếp tục nếu next_extra_delay không âm
            if next_extra_delay >= 0 {
                // Cộng dồn số đường đi từ next_node đến đích
                unsafe {
                    PATH_COUNT[current_node][extra_delay as usize] += count_paths(next_node, next_extra_delay as i32);

                    // Giới hạn đếm để tránh tràn số
                    if PATH_COUNT[current_node][extra_delay as usize] >= MAX_COUNT_CAP as i32 {
                        PATH_COUNT[current_node][extra_delay as usize] = MAX_COUNT_CAP as i32;
                        break;
                    }
                }
            }
        }
    }

    unsafe { PATH_COUNT[current_node][extra_delay as usize] }
}

fn main() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    // Đọc input: số nút, số cạnh, nút đích, chỉ số K
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let arr: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

    unsafe {
        NODE_COUNT = arr[0];
        EDGE_COUNT = arr[1];
        TARGET_NODE = arr[2];
        KTH_PATH = arr[3];

        // Khởi tạo đồ thị
        FORWARD_GRAPH = vec![vec![]; NODE_COUNT + 1];
        REVERSE_GRAPH = vec![vec![]; NODE_COUNT + 1];
    }

    // Tìm độ trễ nhỏ nhất của cạnh (c_min)
    let mut min_edge_delay = INFINITY_VALUE;

    // Đọc dữ liệu các cạnh
    for _ in 0..unsafe { EDGE_COUNT } {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let arr: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let (from_node, to_node, delay) = (arr[0], arr[1], arr[2]);

        unsafe {
            // Thêm cạnh vào đồ thị thuận và ngược
            FORWARD_GRAPH[from_node].push(Edge {
                destination: to_node,
                delay,
            });
            REVERSE_GRAPH[to_node].push(Edge {
                destination: from_node,
                delay,
            });
        }

        // Cập nhật độ trễ nhỏ nhất
        if delay < min_edge_delay {
            min_edge_delay = delay;
        }
    }

    unsafe {
        // Sắp xếp danh sách kề theo chỉ số nút đích để đảm bảo thứ tự từ điển
        for u in 1..=NODE_COUNT {
            FORWARD_GRAPH[u].sort_by_key(|e| e.destination);
        }

        // Khởi tạo khoảng cách cho thuật toán Bellman-Ford
        for u in 1..=NODE_COUNT {
            MIN_DISTANCE[u] = INFINITY_VALUE;
            for p in 0..MAX_EXTRA_DELAY {
                PATH_COUNT[u][p] = -1;
            }
        }

        // Khoảng cách từ đích đến chính nó = 0
        MIN_DISTANCE[TARGET_NODE] = 0;

        // Bellman-Ford trên đồ thị ngược để tính khoảng cách ngắn nhất từ mỗi nút đến đích
        // Cần vòng lặp ngoài cùng (N-1) lần vì trong trường hợp xấu nhất,
        // đường đi ngắn nhất giữa hai nút có thể đi qua (N-1) cạnh
        for _ in 1..NODE_COUNT {
            for u in 1..=NODE_COUNT {
                // Bỏ qua các nút chưa có đường đi đến đích
                if MIN_DISTANCE[u] == INFINITY_VALUE {
                    continue;
                }

                // Xét tất cả các cạnh đi vào u
                for edge in &REVERSE_GRAPH[u] {
                    let (prev, w) = (edge.destination, edge.delay);

                    // Cập nhật khoảng cách nếu tìm thấy đường đi ngắn hơn
                    let new_distance = MIN_DISTANCE[u].saturating_add(w);
                    if MIN_DISTANCE[prev] > new_distance {
                        MIN_DISTANCE[prev] = new_distance;
                    }
                }
            }
        }

        // Kiểm tra nếu không có đường đi từ nguồn đến đích
        if MIN_DISTANCE[1] == INFINITY_VALUE {
            println!("-1");
            return;
        }

        // Tính giới hạn QoS: t_min + c_min
        MAX_ALLOWED_DELAY = MIN_DISTANCE[1] + min_edge_delay;

        // Đếm tổng số đường đi thỏa điều kiện QoS từ nguồn (nút 1) đến đích
        let mut total_paths = 0i32;
        let max_extra = MAX_ALLOWED_DELAY.saturating_sub(MIN_DISTANCE[1]);

        for extra in 0..=max_extra {
            total_paths += count_paths(1, extra as i32);
            if total_paths >= MAX_COUNT_CAP as i32 {
                total_paths = MAX_COUNT_CAP as i32;
                break;
            }
        }

        // Kiểm tra nếu không có đủ đường đi thứ K
        if (total_paths as usize) < KTH_PATH {
            println!("-1");
            return;
        }

        // Khôi phục đường đi thứ K bằng cách xây dựng từng bước
        let mut path = vec![1]; // Bắt đầu từ nút nguồn
        let mut current_node = 1;
        let mut remaining_k = KTH_PATH;
        let mut remaining_delay = MAX_ALLOWED_DELAY;

        while remaining_k > 0 {
            // Điều kiện dừng: đã tìm được đường đi thứ K và đã đến đích
            if remaining_k == 1 && current_node == TARGET_NODE {
                break;
            }

            let mut picked_edge = false;

            // Thử từng cạnh đi ra từ nút hiện tại (theo thứ tự từ điển)
            for edge in &FORWARD_GRAPH[current_node] {
                let next_node = edge.destination;
                let edge_delay = edge.delay;

                // Bỏ qua nếu nút tiếp không có đường đi đến đích hoặc độ trễ vượt quá giới hạn
                if MIN_DISTANCE[next_node] == INFINITY_VALUE || edge_delay > remaining_delay {
                    continue;
                }

                // Đếm số đường đi từ next_node đến đích với độ trễ còn lại
                let max_extra = match remaining_delay.checked_sub(edge_delay + MIN_DISTANCE[next_node]) {
                    Some(x) => x,
                    None => continue,
                };

                let mut paths_through_edge = 0i32;
                for extra in 0..=max_extra {
                    paths_through_edge += count_paths(next_node, extra as i32);
                    if paths_through_edge >= MAX_COUNT_CAP as i32 {
                        paths_through_edge = MAX_COUNT_CAP as i32;
                        break;
                    }
                }

                // Nếu đủ đường đi để bao gồm đường thứ K, đi theo cạnh này
                if (paths_through_edge as usize) >= remaining_k {
                    current_node = next_node;
                    remaining_delay = remaining_delay.saturating_sub(edge_delay);
                    picked_edge = true;
                    break;
                } else {
                    // Bỏ qua paths_through_edge đường đi và tiếp tục tìm
                    remaining_k -= paths_through_edge as usize;
                }
            }

            // Thêm nút mới vào đường đi
            path.push(current_node);

            // Nếu không tìm được cạnh nào phù hợp
            if !picked_edge {
                break;
            }
        }

        // In kết quả
        println!("{}", path.len());
        for node in path {
            print!("{} ", node);
        }
        println!();
    }
}
