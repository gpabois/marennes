use std::{alloc::Layout, cell::Cell, ptr::{self, NonNull}};

use super::{entry::BookEntry, line::{drop_strong_ref, Line, LineInner}};


pub(super) struct Page<const N: usize, Item>{
    ptr: NonNull<PageInner<N, Item>>
}

impl<const N: usize, Item> Page<N, Item> {
    /// Crée une nouvelle page de taille N.
    pub fn new() -> Self {
        Self::from_inner(
            Box::leak(
                Box::new(PageInner::new())
            ).into()
        )
    }
}

/// Implémente la copie de référence forte sur une page de données.
impl<const N: usize, Item> Clone for Page<N, Item> {
    fn clone(&self) -> Self {
        self.inner_ref().inc_strong();
        Self { ptr: self.ptr.clone() }
    }
}

/// Quelques fonctions internes pour manipuler le type interne de la page.
impl<const N: usize, Item> Page<N, Item> {
    #[inline(always)]
    fn inner_ref(&self) -> &PageInner<N, Item> {
        unsafe { self.ptr.as_ref() }
    }

    #[inline(always)]
    fn inner_mut(&self) -> &mut PageInner<N, Item> {
        unsafe { self.ptr.as_mut() }
    }

    fn from_inner(ptr: NonNull<PageInner<N, Item>>) -> Self {
        Self {
            ptr
        }
    }
}

impl<const N: usize, Item> Drop for Page<N, Item> {
    fn drop(&mut self) {
        unsafe {
            self.inner_ref().dec_strong();

            if self.inner_ref().strong() == 0 {
                ptr::drop_in_place(self.inner_mut());
            }

            self.inner_ref().dec_weak();

            if self.inner_ref().weak() == 0 {
                std::alloc::dealloc(
                    self.ptr.as_ptr() as *mut u8, 
                    Layout::for_value(self.ptr.as_ref())
                );
            }
        }
    }
}

impl<const N: usize, Item> Page<N, Item> {
    /// Crée une référence faible sur la page.
    pub(super) fn downgrade(this: &Self) -> WeakPage<N, Item> {
        this.inner_ref().inc_weak();
        WeakPage { ptr: this.ptr }
    }

    /// Retourne vrai si la page ne dispose plus de lignes libres.
    pub fn is_full(&self) -> bool {
        self.inner_ref().cursor() >= N
    }

    /// Ecris une nouvelle entrée, si il reste de la place.
    pub fn write(&mut self, item: Item) -> Option<BookEntry<N,Item>> {
        self.inner_mut().alloc(item).map(|ptr| {
            let line = Line {
                ptr: ptr.into()
            };

            BookEntry {
                page: self.clone(),
                line
            }
        })
    }
}


/// Référence faible sur une page.
pub struct WeakPage<const N: usize, Item> {
    ptr: NonNull<PageInner<N, Item>>
}

impl<const N: usize, Item> Clone for WeakPage<N, Item> {
    fn clone(&self) -> Self {
        self.inner().inc_weak();
        Self { ptr: self.ptr.clone() }
    }
}

impl<const N: usize, Item> WeakPage<N, Item> {
    #[inline(always)]
    fn inner(&self) -> &PageInner<N, Item> {
        unsafe {
            self.ptr.as_ref()
        }
    }
}

impl<const N: usize, Item> WeakPage<N, Item> {
    pub(super) fn try_upgrade(&self) -> Option<Page<N, Item>> {
        unsafe {
            if self.inner().strong() == 0 {
                return None
            }

            self.inner().inc_strong();
            Some(Page {ptr: self.ptr})
        }
    }
}

impl<const N: usize, Item> Drop for WeakPage<N, Item> {
    fn drop(&mut self) {
        unsafe {
            self.inner().dec_weak();

            // Plus rien nous retient,
            // on peut déallouer la page.
            if self.inner().weak() == 0 {
                std::alloc::dealloc(
                    self.ptr.as_ptr() as *mut u8, 
                    Layout::for_value(self.ptr.as_ref())
                );
            }
        }

    }
}

/// Type interne contenant les données d'une page.
pub(super) struct PageInner<const N: usize, Item> {
    cursor: usize,
    strong: Cell<usize>,
    weak:   Cell<usize>,
    lines:  [LineInner<Item>; N]
}

impl<const N: usize, Item> PageInner<N, Item> {
    pub fn new() -> Self {
        Self {
            cursor: 0,
            strong: Cell::new(0),
            weak: Cell::new(0),
            lines: unsafe{std::mem::zeroed()}
        }
    }
}

impl<const N: usize, Item> PageInner<N, Item> {
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Alloue une nouvelle ligne
    /// TODO: Implémente la recherche des lignes libérées.
    pub(self) fn alloc(&mut self, item: Item) -> Option<NonNull<LineInner<Item>>> {
        // Plus de place...
        if self.cursor() >= N {
            return None
        }

        self.lines[self.cursor] = LineInner::new(item);
        let ptr = ptr::addr_of_mut!(self.lines[self.cursor]);  
        self.cursor += 1;
        Some(NonNull::new(ptr).unwrap())
    }
}

/// On doit libérer les acquisitions sur les lignes en cours
/// Si une référence forte est maintenue quelque part, 
/// la ligne ne sera pas libérée
impl<const N: usize, Item> Drop for PageInner<N, Item> {
    fn drop(&mut self) {
        unsafe {
            // On récupère uniquement 
            // les lignes qui ont été allouées (déjà initialisées) au moins une fois.
            self.lines
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| *i < self.cursor)
            .map(|(_, line)| line)
            .for_each(|line| unsafe {
                drop_strong_ref(line)
            })
        }
    }
}

/// Implémente les fonctions liées aux compteurs de références.
impl<const N: usize, Item> PageInner<N, Item> {
        /// Acquiert une référence forte sur la page.
    ///
    /// Cela empêche une déallocation de la page tant 
    /// que le compteur de référence est supérieur à
    /// zéro.
    ///
    /// Tout acquisition doit faire l'objet d'une libération.
    #[inline(always)]
    pub fn inc_strong(&self) {
       self.strong_ref().set(self.strong() + 1);
    }

    /// Libère une référence forte sur la page
    ///
    /// Si le compteur est à zéro, la page est déallouée.
    /// Toute libération doit être associée à une acquisition.
    #[inline(always)]
    pub fn dec_strong(&self) {
       self.strong_ref().set(self.strong() - 1);
    }

    #[inline(always)]
    fn strong_ref(&self) -> &Cell<usize> {
        &self.strong
    }

    /// Récupère la valeur du compteur de références fortes.
    #[inline(always)]
    fn strong(&self) -> usize {
        self.strong.get()
    }

    #[inline(always)]
    pub fn inc_weak(&self) {
        self.weak_ref().set(self.weak() + 1)
    }

    #[inline(always)]
    fn weak_ref(&self) -> &Cell<usize> {
        &self.weak
    }

    /// Récupère la valeur du compteur de références faibles.
    #[inline(always)]
    fn weak(&self) -> usize {
        self.weak.get()
    }

    #[inline(always)]
    pub fn dec_weak(&self) {
       self.weak_ref().set(self.weak() - 1);
    }
}