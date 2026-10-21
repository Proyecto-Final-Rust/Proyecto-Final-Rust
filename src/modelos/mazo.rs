use rand::seq::SliceRandom;

use crate::modelos::carta::{Carta, NUMEROS, Palo};

pub struct Mazo {
    cartas: Vec<Carta>
}

impl Mazo { 
    /// Genera un mazo español de 40 cartas y reordena las cartas
    /// de forma aleatoria haciendo uso del crate rand.
    pub fn new() -> Mazo {
        let mut cartas: Vec<Carta> = Vec::new();

        let palos = [
            Palo::Espada,
            Palo::Basto,
            Palo::Oro,
            Palo::Copa
        ];

        for palo in palos {
            for numero in NUMEROS {
                cartas.push(Carta::new(palo, numero));
            }
        }

        cartas.shuffle(&mut rand::rng());

        Mazo { cartas }
    }

    /// Toma la carta superior del mazo y la retorna,
    /// quitándola del mazo
    pub fn tomar_carta(&mut self) -> Carta {
        self.cartas.remove(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::modelos::mazo::Mazo;


    #[test]
    fn new_crea_40_cartas() {
        let mazo = Mazo::new();
        assert_eq!(mazo.cartas.len(), 40);
    }

    #[test]
    fn new_crea_cartas_distintas() {
        let mazo = Mazo::new();

        for i in 0..mazo.cartas.len() {
            for j in (i + 1)..mazo.cartas.len() {
                assert_ne!(mazo.cartas[i], mazo.cartas[j]);
            }
        }
    }

    #[test]
    fn tomar_carta_retorna_la_carta_superior() {
        let mut mazo = Mazo::new();
        let carta_superior = mazo.cartas[0].clone();

        assert_eq!(mazo.tomar_carta(), carta_superior);
    }

    #[test]
    fn tomar_carta_la_elimina_del_mazo() {
        let mut mazo = Mazo::new();
        let carta_superior = mazo.cartas[0].clone();

        mazo.tomar_carta();

        assert_eq!(mazo.cartas.len(), 39);
        assert_eq!(mazo.cartas.contains(&carta_superior), false);
    }
}
