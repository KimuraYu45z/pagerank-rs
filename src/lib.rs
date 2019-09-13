use std::collections::HashMap;

pub type Matrix = HashMap<String, Vector>;
pub type Vector = HashMap<String, f64>;

pub fn get_stochastic_matrix(link_matrix: &Matrix) -> Matrix {
  let mut stochastic_matrix = Matrix::new();

  for (src, vector) in link_matrix {
    let mut sum = 0f64;
    for (_, value) in vector {
      sum += *value;
    }

    let mut v = Vector::new();
    
    for (dst, value) in vector {
      v.insert(dst.to_string(), *value / sum);
    }
    stochastic_matrix.insert(src.to_string(), v);
  }

  return stochastic_matrix;
}

pub fn transition_score(stochastic_matrix: &Matrix, current_score_vector: &Vector) -> Vector {
  let mut score = Vector::new();

  for (src, vector) in stochastic_matrix {
    for (dst, value) in vector {
      let dst_current_score = score.get(dst).unwrap_or(&0f64);
      let src_current_score = current_score_vector.get(src).unwrap_or(&0f64);
      &score.insert(dst.to_string(), *dst_current_score + *src_current_score * *value);
    }
  }

  return score;
}

#[test]
fn test_pagerank() {
  let mut link_matrix = Matrix::new();
  const SIZE: i32 = 10;
  for i in 0..SIZE {
    let mut vector = Vector::new();

    for j in 0..SIZE {
      let surplus3 = (i + j + 1) as f64 % 3f64;
      if surplus3 == 0f64 {
        continue;
      }
      let surplus5 = (i + j + 1) as f64 % 5f64;

      vector.insert(j.to_string(), (surplus3 + 1f64) / (surplus5 + 1f64));
    }
    link_matrix.insert(i.to_string(), vector);
  }

  let stochastic_matrix = get_stochastic_matrix(&link_matrix);

  let mut score_vector = Vector::new();
  const DEFAULT: f64 = 1f64 / SIZE as f64;
  for i in 0..SIZE {
    score_vector.insert(i.to_string(), DEFAULT);
  }

  const PRECISION: i32 = 6;
  for _ in 0..PRECISION {
    score_vector = transition_score(&stochastic_matrix, &score_vector);
  }

  let mut sum = 0f64;
  for i in 0..SIZE {
    println!("{}", score_vector[&i.to_string()]);
    sum += score_vector[&i.to_string()];
  }
  println!("{}", sum);
}
