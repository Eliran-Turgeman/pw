use rand::prelude::*;
use rand::seq::SliceRandom;

pub(crate) fn generate_strong_password(length: usize) -> String {
    let lowercase_letters = "abcdefghijklmnopqrstuvwxyz";
    let uppercase_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digits = "0123456789";
    let special_chars = "!@#$%^&*()_-+=<>?";
    let all_chars = format!("{}{}{}{}", lowercase_letters, uppercase_letters, digits, special_chars);

    let mut rng = thread_rng();

    let mut password: Vec<char> = vec![
        lowercase_letters.chars().choose(&mut rng).unwrap(),
        uppercase_letters.chars().choose(&mut rng).unwrap(),
        digits.chars().choose(&mut rng).unwrap(),
        special_chars.chars().choose(&mut rng).unwrap(),
    ];

    password.extend((password.len()..length).map(|_| {
        *all_chars.chars().collect::<Vec<char>>().choose(&mut rng).unwrap()
    }));

    password.shuffle(&mut rng);

    password.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_length() {
        let length = 12;
        let password = generate_strong_password(length);
        assert_eq!(password.len(), length);
    }

    #[test]
    fn password_contains_lowercase() {
        let password = generate_strong_password(12);
        assert!(password.chars().any(|c| c.is_lowercase()));
    }

    #[test]
    fn password_contains_uppercase() {
        let password = generate_strong_password(12);
        assert!(password.chars().any(|c| c.is_uppercase()));
    }

    #[test]
    fn password_contains_digit() {
        let password = generate_strong_password(12);
        assert!(password.chars().any(|c| c.is_digit(10)));
    }

    #[test]
    fn password_contains_special_character() {
        let password = generate_strong_password(12);
        let special_chars = "!@#$%^&*()_-+=<>?";
        assert!(password.chars().any(|c| special_chars.contains(c)));
    }

    #[test]
    fn passwords_are_random() {
        let password1 = generate_strong_password(12);
        let password2 = generate_strong_password(12);
        assert_ne!(password1, password2);
    }
}
