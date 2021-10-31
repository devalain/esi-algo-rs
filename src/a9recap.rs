#[derive(Debug, PartialEq)]
pub struct CaractèreInattendu(char);
pub fn valeur_lettre(c: char) -> Result<usize, CaractèreInattendu> {
    match c {
        'A'|'E'|'I'|'L'|'N'|'O'|'R'|'S'|'T'|'U' => Ok(1),
        'D'|'G'|'M' => Ok(2),
        'B'|'C'|'P' => Ok(3),
        'F'|'H'|'V' => Ok(4),
        'J'|'Q' => Ok(8),
        'K'|'W'|'X'|'Y'|'Z'=> Ok(10),
        other => Err(CaractèreInattendu(other))
    }
}

pub fn score_mot(mot: &str) -> Result<usize, CaractèreInattendu> {
    let valeurs = mot.chars().map(valeur_lettre).collect::<Result<Vec<usize>, _>>()?;
    Ok(valeurs.iter().sum())
}

pub fn mot_possible(chevalet: &[char], mot: &str) -> bool {
    let mut copie_chevalet: Vec<Option<char>> = chevalet.iter().cloned().map(Some).collect();
    for lettre in mot.chars() {
        if let Some(found) = copie_chevalet.iter_mut().find(|c| **c == Some(lettre)) {
            *found = None;
        } else {
            return false;
        }
    }

    true
}

pub fn meilleur_mot<'a>(chevalet: &[char], dico: &[&'a str]) -> Result<Option<(&'a str, usize)>, CaractèreInattendu> {
    let scores = dico
        .iter()
        .filter(|mot| mot_possible(chevalet, mot))
        .map(|mot| score_mot(*mot).map(|s| (*mot, s)))
        .collect::<Result<Vec<(&str, usize)>, _>>()?;
    Ok(scores.iter().copied().max_by(|x, y| x.1.cmp(&y.1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meilleur_mot_works() {
        const DICO: &[&str] = &["JAVA", "RUST", "C", "CPP"];
        
        let chevalet = &['J', 'R', 'S', 'U', 'T', 'A', 'A', 'V'];
        assert_eq!(meilleur_mot(chevalet, DICO), Ok(Some(("JAVA", 14))));
    }

    #[test]
    fn score_mot_works() {
        assert_eq!(score_mot("JAVA"), Ok(14));
        assert_eq!(score_mot("java"), Err(CaractèreInattendu('j')));
    }

    #[test]
    fn valeur_lettre_works() {
        assert_eq!(valeur_lettre('J'), Ok(8));
        assert_eq!(valeur_lettre('x'), Err(CaractèreInattendu('x')));
    }

    #[test]
    fn mot_possible_works() {
        let chevalet = &['A', 'O', 'V', 'G', 'G', 'J', 'L'];
        
        let mot = "ALGO";
        assert!(mot_possible(chevalet, mot));
        
        let mot = "JAVA";
        assert!(!mot_possible(chevalet, mot));
        
        let mot = "JOGGER";
        assert!(!mot_possible(chevalet, mot));
    }
}
