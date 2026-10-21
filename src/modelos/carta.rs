#[derive(Clone, Debug, PartialEq)]
pub struct Carta {
    palo: Palo,
    numero: i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Palo {
    Espada,
    Basto,
    Oro,
    Copa
}

pub const NUMEROS: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 10, 11, 12];

impl Carta {
    /// Constructor de Carta
    pub fn new(palo: Palo, numero: i32) -> Carta {
        if !NUMEROS.contains(&numero) {
            panic!("El número de carta es inválido")
        }

        Carta {
            palo,
            numero
        }
    }

    /// Devuelve la fuerza de la carta según las reglas del truco.
    ///
    /// Los valores más altos representan cartas más fuertes.
    ///
    /// # Panics
    /// Ocurre un panic si la combinación de número y palo no es válida
    pub fn get_fuerza(&self) -> i32 {
        match (self.numero, self.palo) {
            (1, Palo::Espada) => 14,
            (1, Palo::Basto) => 13,
            (7, Palo::Espada) => 12,
            (7, Palo::Oro) => 11,
            (3, _) => 10,
            (2, _) => 9,
            (1, Palo::Oro) => 8,
            (1, Palo::Copa) => 8,
            (12, _) => 7,
            (11, _) => 6,
            (10, _) => 5,
            (7, Palo::Copa) => 4,
            (7, Palo::Basto) => 4,
            (6, _) => 3,
            (5, _) => 2,
            (4, _) => 1,
            _ => panic!("Combinación prohibida")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ResultadoComparacion {
    GanaPrimera,
    GanaSegunda,
    Parda
}

/// Compara dos cartas según su fuerza
///
/// # Arguments
///
/// * `primera` - La primera carta de la comparación.
/// * `segunda` - La segunda carta de la comparación.
///
/// # Returns
///
/// Devuelve `GanaPrimera` si la primera carta es más fuerte,
/// `GanaSegunda` si la segunda es más fuerte o `Parda` si
/// ambas tienen la misma fuerza.
pub fn comparar_cartas(
    primera: Carta,
    segunda: Carta
) -> ResultadoComparacion {
    let fuerza_primera = primera.get_fuerza();
    let fuerza_segunda = segunda.get_fuerza();

    if fuerza_primera > fuerza_segunda { return ResultadoComparacion::GanaPrimera };
    if fuerza_primera < fuerza_segunda { return ResultadoComparacion::GanaSegunda };

    ResultadoComparacion::Parda
}

#[cfg(test)]
mod tests {
    mod cartas {
        use crate::modelos::carta::{Carta, Palo};

        pub fn ancho_espada() -> Carta { Carta::new(Palo::Espada, 1) }
        pub fn ancho_basto() -> Carta { Carta::new(Palo::Basto, 1) }

        pub fn uno_oro() -> Carta { Carta::new(Palo::Oro, 1) }
        pub fn uno_copa() -> Carta { Carta::new(Palo::Copa, 1) }

        pub fn dos_espada() -> Carta { Carta::new(Palo::Espada, 2) }
        pub fn dos_basto() -> Carta { Carta::new(Palo::Basto, 2) }
        pub fn dos_oro() -> Carta { Carta::new(Palo::Oro, 2) }
        pub fn dos_copa() -> Carta { Carta::new(Palo::Copa, 2) }

        pub fn tres_espada() -> Carta { Carta::new(Palo::Espada, 3) }
        pub fn tres_basto() -> Carta { Carta::new(Palo::Basto, 3) }
        pub fn tres_oro() -> Carta { Carta::new(Palo::Oro, 3) }
        pub fn tres_copa() -> Carta { Carta::new(Palo::Copa, 3) }

        pub fn cuatro_espada() -> Carta { Carta::new(Palo::Espada, 4) }
        pub fn cuatro_basto() -> Carta { Carta::new(Palo::Basto, 4) }
        pub fn cuatro_oro() -> Carta { Carta::new(Palo::Oro, 4) }
        pub fn cuatro_copa() -> Carta { Carta::new(Palo::Copa, 4) }

        pub fn cinco_espada() -> Carta { Carta::new(Palo::Espada, 5) }
        pub fn cinco_basto() -> Carta { Carta::new(Palo::Basto, 5) }
        pub fn cinco_oro() -> Carta { Carta::new(Palo::Oro, 5) }
        pub fn cinco_copa() -> Carta { Carta::new(Palo::Copa, 5) }

        pub fn seis_espada() -> Carta { Carta::new(Palo::Espada, 6) }
        pub fn seis_basto() -> Carta { Carta::new(Palo::Basto, 6) }
        pub fn seis_oro() -> Carta { Carta::new(Palo::Oro, 6) }
        pub fn seis_copa() -> Carta { Carta::new(Palo::Copa, 6) }

        pub fn siete_espada() -> Carta { Carta::new(Palo::Espada, 7) }
        pub fn siete_basto() -> Carta { Carta::new(Palo::Basto, 7) }
        pub fn siete_oro() -> Carta { Carta::new(Palo::Oro, 7) }
        pub fn siete_copa() -> Carta { Carta::new(Palo::Copa, 7) }

        pub fn sota_espada() -> Carta { Carta::new(Palo::Espada, 10) }
        pub fn sota_basto() -> Carta { Carta::new(Palo::Basto, 10) }
        pub fn sota_oro() -> Carta { Carta::new(Palo::Oro, 10) }
        pub fn sota_copa() -> Carta { Carta::new(Palo::Copa, 10) }

        pub fn caballo_espada() -> Carta { Carta::new(Palo::Espada, 11) }
        pub fn caballo_basto() -> Carta { Carta::new(Palo::Basto, 11) }
        pub fn caballo_oro() -> Carta { Carta::new(Palo::Oro, 11) }
        pub fn caballo_copa() -> Carta { Carta::new(Palo::Copa, 11) }

        pub fn rey_espada() -> Carta { Carta::new(Palo::Espada, 12) }
        pub fn rey_basto() -> Carta { Carta::new(Palo::Basto, 12) }
        pub fn rey_oro() -> Carta { Carta::new(Palo::Oro, 12) }
        pub fn rey_copa() -> Carta { Carta::new(Palo::Copa, 12) }
    }
    use crate::modelos::carta::{Carta, Palo, ResultadoComparacion, comparar_cartas};
    use cartas::*;

    #[test]
    #[should_panic(expected="El número de carta es inválido")]
    fn crear_una_carta_con_numero_invalido_provoca_panic() {
        Carta::new(Palo::Espada, 8);
    }

    #[test]
    fn get_fuerzas_de_las_cartas_son_las_esperadas() {
        assert_eq!(ancho_espada().get_fuerza(), 14);
        assert_eq!(ancho_basto().get_fuerza(), 13);
        assert_eq!(siete_espada().get_fuerza(), 12);
        assert_eq!(siete_oro().get_fuerza(), 11);
        assert_eq!(tres_espada().get_fuerza(), 10);
        assert_eq!(tres_basto().get_fuerza(), 10);
        assert_eq!(tres_oro().get_fuerza(), 10);
        assert_eq!(tres_copa().get_fuerza(), 10);
        assert_eq!(dos_espada().get_fuerza(), 9);
        assert_eq!(dos_basto().get_fuerza(), 9);
        assert_eq!(dos_oro().get_fuerza(), 9);
        assert_eq!(dos_copa().get_fuerza(), 9);
        assert_eq!(uno_oro().get_fuerza(), 8);
        assert_eq!(uno_copa().get_fuerza(), 8);
        assert_eq!(rey_espada().get_fuerza(), 7);
        assert_eq!(rey_basto().get_fuerza(), 7);
        assert_eq!(rey_oro().get_fuerza(), 7);
        assert_eq!(rey_copa().get_fuerza(), 7);
        assert_eq!(caballo_espada().get_fuerza(), 6);
        assert_eq!(caballo_basto().get_fuerza(), 6);
        assert_eq!(caballo_oro().get_fuerza(), 6);
        assert_eq!(caballo_copa().get_fuerza(), 6);
        assert_eq!(sota_espada().get_fuerza(), 5);
        assert_eq!(sota_basto().get_fuerza(), 5);
        assert_eq!(sota_oro().get_fuerza(), 5);
        assert_eq!(sota_copa().get_fuerza(), 5);
        assert_eq!(siete_basto().get_fuerza(), 4);
        assert_eq!(siete_copa().get_fuerza(), 4);
        assert_eq!(seis_espada().get_fuerza(), 3);
        assert_eq!(seis_basto().get_fuerza(), 3);
        assert_eq!(seis_oro().get_fuerza(), 3);
        assert_eq!(seis_copa().get_fuerza(), 3);
        assert_eq!(cinco_espada().get_fuerza(), 2);
        assert_eq!(cinco_basto().get_fuerza(), 2);
        assert_eq!(cinco_oro().get_fuerza(), 2);
        assert_eq!(cinco_copa().get_fuerza(), 2);
        assert_eq!(cuatro_espada().get_fuerza(), 1);
        assert_eq!(cuatro_basto().get_fuerza(), 1);
        assert_eq!(cuatro_oro().get_fuerza(), 1);
        assert_eq!(cuatro_copa().get_fuerza(), 1);
    }

    #[test]
    #[should_panic(expected="Combinación prohibida")]
    fn get_fuerza_de_una_carta_con_numero_8_provoca_panic() {
        let ocho_espada = Carta { palo: Palo::Espada, numero: 8 };
        ocho_espada.get_fuerza();
    }

    #[test]
    #[should_panic(expected="Combinación prohibida")]
    fn get_fuerza_de_una_carta_con_numero_9_provoca_panic() {
        let nueve_espada = Carta { palo: Palo::Espada, numero: 9 };
        nueve_espada.get_fuerza();
    }

    #[test]
    fn ancho_de_espada_vence_a_ancho_de_basto() {
        assert_eq!(comparar_cartas(ancho_espada(), ancho_basto()), ResultadoComparacion::GanaPrimera);
    }
    #[test]
    fn ancho_de_basto_vence_a_siete_de_espada() {
        assert_eq!(comparar_cartas(ancho_basto(), siete_espada()), ResultadoComparacion::GanaPrimera);
    }
    #[test]
    fn siete_de_espada_vence_a_siete_de_oro() {
        assert_eq!(comparar_cartas(siete_espada(), siete_oro()), ResultadoComparacion::GanaPrimera);
    }
    #[test]
    fn tres_vence_a_dos() {
        assert_eq!(comparar_cartas(tres_espada(), dos_espada()), ResultadoComparacion::GanaPrimera);
    }

    #[test]
    fn test_casos_de_parda() {
        assert_eq!(comparar_cartas(tres_espada(), tres_oro()), ResultadoComparacion::Parda);
        assert_eq!(comparar_cartas(dos_espada(), dos_oro()), ResultadoComparacion::Parda);
        assert_eq!(comparar_cartas(uno_oro(), uno_copa()), ResultadoComparacion::Parda);
        assert_eq!(comparar_cartas(rey_espada(), rey_basto()), ResultadoComparacion::Parda);
        assert_eq!(comparar_cartas(caballo_espada(), caballo_basto()), ResultadoComparacion::Parda);
        assert_eq!(comparar_cartas(sota_espada(), sota_basto()), ResultadoComparacion::Parda);
        assert_eq!(comparar_cartas(siete_copa(), siete_basto()), ResultadoComparacion::Parda);
        assert_eq!(comparar_cartas(seis_espada(), seis_basto()), ResultadoComparacion::Parda);
        assert_eq!(comparar_cartas(cinco_espada(), cinco_basto()), ResultadoComparacion::Parda);
        assert_eq!(comparar_cartas(cuatro_espada(), cuatro_basto()), ResultadoComparacion::Parda);
    }
}
