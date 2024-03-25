use self::page::Page;

pub mod line;
pub mod page;
pub mod entry;

pub use entry::{BookReadEntry, BookEntry, BookWeakEntry};
pub enum BookError {
    AlwaysBorrowed,
    AlwaysMutBorrowed
}

pub type BookResult<D> = Result<D, BookError>;

/// Un livre est une collection d'objets répartis entre pages,
/// des blocs de mémoires contiguës contenant jusqu'à N éléments.
/// 
/// Le paramètre N permet de limiter les allocations répétées des entrées,
/// en contrepartie, l'empreinte mémoire est plus grande car une référence
/// forte sur une ligne, empêche la libération de la page entière.
/// 
/// Une entrée est indépendante des autres,
/// de sorte qu'il peut être écrit de nouvelles entrées,
/// tout en conservant une référence mutable à une
/// ou plusieurs entrées.
/// 
/// Chaque entrée peut :
/// - être empruntée en lecture plusieurs fois,
/// mais uniquement une fois en écriture, tant qu'il ne subsiste
/// pas d'emprunts en lecture seule.
/// - être dégradée en référence faible ;
/// - ne sera pas libérée tant qu'il subsiste au moins une référence forte.
/// 
/// Une page n'est pas libérée tant qu'il subsiste une référence forte
/// à une de ses lignes (entrées).
/// 
/// Un livre peut être fusionné avec un autre, sans surcoût, 
/// tout en s'assurant que les références existante ne sont pas invalidées.
pub struct Book<const N: usize, Item> {
    /// Garde toutes les pages du livre
    all: Vec<Page<N, Item>>,
    /// Garde les pages non remplies
    rems: Vec<Page<N, Item>>
}

impl<const N: usize, Item> Default for Book<N, Item> {
    fn default() -> Self {
        Self { all: Default::default(), rems: Default::default() }
    }
}

impl<const N: usize, Item> Book<N, Item> {
    /// Ecris une nouvelle entrée dans le livre
    pub fn write(&mut self, item: Item) -> BookEntry<N, Item> {
        if self.rems.is_empty() {
            let page = Page::<N, Item>::new();
            self.all.push(page.clone());
            self.rems.push(page);
        }

        let page = self.rems.last_mut().unwrap();
        let entry = page.write(item).unwrap();

        // Page pleine, on la retire de notre stack.
        if page.is_full() {
            self.rems.pop();
        }

        entry
    }

    /// Fusionne le livre de droite avec le livre à gauche.
    pub fn merge(&mut self, mut other: Self) {
        self.all.append(&mut other.all);
        self.rems.append(&mut other.rems);
    }
}

impl<const N: usize, Item> std::ops::AddAssign for Book<N, Item> {
    fn add_assign(&mut self, rhs: Self) {
        self.merge(rhs)
    }
}

