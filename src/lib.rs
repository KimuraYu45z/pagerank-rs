use std::collections::HashMap;

pub type Matrix = HashMap<String, Vector>;
pub type Vector = HashMap<String, f64>;

pub fn get_stochastic_matrix(link_matrix: &Matrix) -> Matrix {
  let mut stochastic_matrix = Matrix::new();

  for (src, column) in link_matrix {
    let mut sum = 0f64;
    for (_, value) in column {
      sum += value;
    }

    stochastic_matrix.insert(src.to_string(), Vector::new());
    for (dst, value) in column {
      let v = stochastic_matrix.get_mut(src).unwrap();
      v.insert(dst.to_string(), *value / sum);
    }
  }

  return stochastic_matrix;
}

pub fn transition_score(stochastic_matrix: &Matrix, current_score_vector: &Vector) -> Vector {
  let mut score = Vector::new();

  for (src, column) in stochastic_matrix {
    for (dst, value) in column {
      score.entry(dst.to_string()).or_insert(0f64);
      let current_score = current_score_vector.get(src).unwrap_or(&0f64);
      let v = score.get_mut(dst).unwrap();
      *v += *current_score * *value;
    }
  }

  return score;
}