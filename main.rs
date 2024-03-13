use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;
use std::num::ParseFloatError;
use std::str::FromStr;
use std::time::Instant;

type Point = (f64, f64);

fn distance(p1: Point, p2: Point) -> f64 {
    let (dx, dy) = (p2.0 - p1.0, p2.1 - p1.1);
    f64::sqrt(dx * dx + dy * dy)
}

fn nearest_neighbor(points: &[Point]) -> (Vec<Point>, f64) {
    let n = points.len();
    let mut visited = vec![false; n];
    let mut path = Vec::with_capacity(n);
    let mut total_distance = 0.0;
    
    let mut current_point_idx = 0;
    path.push(points[current_point_idx]);
    visited[current_point_idx] = true;

    for _ in 1..n {
        let mut min_dist = f64::INFINITY;
        let mut nearest_idx = 0;
        for (idx, point) in points.iter().enumerate() {
            if !visited[idx] {
                let dist = distance(path.last().unwrap().clone(), *point);
                if dist < min_dist {
                    min_dist = dist;
                    nearest_idx = idx;
                }
            }
        }
        path.push(points[nearest_idx]);
        visited[nearest_idx] = true;
        current_point_idx = nearest_idx; // Güncellenen satır
        total_distance += min_dist;
    }

    total_distance += distance(path[n - 1], points[0]);
    path.push(points[0]);
    (path, total_distance)
}

#[derive(Debug)]
enum PointParseError {
    InvalidFormat(String),
    ParseError(ParseFloatError),
    IoError(io::Error),
}

impl Error for PointParseError {}

impl fmt::Display for PointParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PointParseError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            PointParseError::ParseError(err) => write!(f, "Error parsing coordinate: {}", err),
            PointParseError::IoError(err) => write!(f, "I/O Error: {}", err),
        }
    }
}

impl From<io::Error> for PointParseError {
    fn from(err: io::Error) -> Self {
        PointParseError::IoError(err)
    }
}

fn parse_point(line: &str) -> Result<Point, PointParseError> {
    let coords: Vec<f64> = line
        .split_whitespace()
        .map(|s| f64::from_str(s).map_err(|e| PointParseError::ParseError(e)))
        .collect::<Result<_, _>>()?;
    if coords.len() != 2 {
        return Err(PointParseError::InvalidFormat(line.to_string()));
    }
    Ok((coords[0], coords[1]))
}

fn read_points_from_file(filename: &str) -> Result<Vec<Point>, PointParseError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut points = Vec::new();
    for line in reader.lines().filter_map(Result::ok) {
        let point = parse_point(&line)?;
        points.push(point);
    }
    Ok(points)
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let points = read_points_from_file("data5.txt")?;
    let (path, total_distance) = nearest_neighbor(&points);
    let elapsed_time = start_time.elapsed().as_secs_f64();
    println!("Path: {:?}", path);
    println!("Total Distance: {}", total_distance);
    println!("Elapsed Time: {:.6} seconds", elapsed_time);
    Ok(())
}
