use rand::Rng;

/// Функція для генерації вектора випадкових чисел у діапазоні [10..99]
pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

/// Функція для знаходження індексів мінімальної пари суміжних елементів
pub fn min_adjacent_sum(data: &[i32]) -> (usize, usize) {
    data.windows(2)
        .enumerate()
        .min_by_key(|&(_, pair)| pair[0] + pair[1])
        .map(|(i, _)| (i, i + 1))
        .unwrap()
}

/// Функція для красивого виводу вектора з підсвіченням мінімальної пари
pub fn print_vector_with_min_sum(data: &[i32]) {
    let (i, j) = min_adjacent_sum(data);

    // Вивід індексів
    println!("indexes: {}", (0..data.len()).map(|x| format!("{:2}.", x)).collect::<Vec<_>>().join(" "));

    // Вивід значень
    println!("data:    [{}]", data.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(", "));

    // Вивід підкреслення для мінімальної пари
    println!("indexes: {}\\__ __/", " ".repeat(4 * i));

    // Вивід інформації про мінімальну пару
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[i], data[j], data[i] + data[j], i, j
    );
}

fn main() {
    let data = gen_random_vector(20);
    print_vector_with_min_sum(&data);
}
