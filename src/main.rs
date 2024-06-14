use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    id: u32,
    name: String,
    language: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Showtime {
    movie_id: u32,
    cinema_id: u32,
    time: u32,
    available_seats: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    movies: Vec<Movie>,
    showtimes: Vec<Showtime>,
}

#[derive(Debug)]
struct FilterCriteria {
    language: Option<String>,
    available_seats: Option<u32>,
    time: Option<u32>,
}

fn filter_movies(api_response: &ApiResponse, criteria: &FilterCriteria) -> Vec<(u32, u32)> {
    let mut movie_ids = HashSet::new();
    if let Some(ref language) = criteria.language {
        for movie in &api_response.movies {
            if movie.language.contains(language) {
                movie_ids.insert(movie.id);
            }
        }
    } else {
        for movie in &api_response.movies {
            movie_ids.insert(movie.id);
        }
    }

    let mut result = Vec::new();
    for showtime in &api_response.showtimes {
        if movie_ids.contains(&showtime.movie_id) {
            if criteria
                .available_seats
                .map_or(true, |s| showtime.available_seats > s)
                && criteria.time.map_or(true, |t| showtime.time == t)
            {
                result.push((showtime.movie_id, showtime.cinema_id));
            }
        }
    }

    result
}

fn main() {
    let data = r#"
    {
        "movies": [
            {
                "id": 123,
                "name": "XYZ",
                "language": ["Hindi", "English"]
            },
            {
                "id": 456,
                "name": "ABC",
                "language": ["German", "English"]
            },
            {
                "id": 546,
                "name": "DEF",
                "language": ["English"]
            }
        ],
        "showtimes": [
            {
                "movieId": 123,
                "cinemaId": 345,
                "time": 1700,
                "availableSeats": 10
            },
            {
                "movieId": 456,
                "cinemaId": 678,
                "time": 1800,
                "availableSeats": 50
            },
            {
                "movieId": 546,
                "cinemaId": 911,
                "time": 1100,
                "availableSeats": 100
            }
        ]
    }
    "#;

    let api_response: ApiResponse = serde_json::from_str(data).unwrap();

    let criteria1 = FilterCriteria {
        language: Some("English".to_string()),
        available_seats: None,
        time: None,
    };

    let criteria2 = FilterCriteria {
        language: Some("English".to_string()),
        available_seats: Some(20),
        time: None,
    };

    let result1 = filter_movies(&api_response, &criteria1);
    println!("Result 1: {:?}", result1); // Expected: [(123, 345), (456, 678), (546, 911)]

    let result2 = filter_movies(&api_response, &criteria2);
    println!("Result 2: {:?}", result2); // Expected: [(456, 678), (546, 911)]
}
