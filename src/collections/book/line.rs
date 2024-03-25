use std::{
    cell::Cell, io::Read, ops::{Deref, DerefMut}, ptr::NonNull
};

use super::{BookError, BookResult};

/// Une entrée de la page (contient un objet, et de quoi vérifier les règles d'emprunt)
pub(super) struct LineInner<Item> {
    /// Nombre d'emprunts en lecture seule
    read: Cell<usize>,
    
    /// Nombre d'emprunts en écriture (en théorie 0 ou 1)
    write: Cell<usize>,
    
    /// Nombre de références fortes (= read + write + page)
    strong: Cell<usize>,
    
    /// Nombre de références faibles
    weak: Cell<usize>,
    
    /// The data
    pub(self) item: Item,
}

impl<Item> Default for LineInner<Item> {
    fn default() -> Self {
        unsafe{Self::new_unitialised()}
    }
}

impl<Item> LineInner<Item> {
    /// Crée une nouvelle entrée stockant un objet.
    pub fn new(item: Item) -> Self {
        Self {
            read: Cell::new(0),
            write: Cell::new(0),
            strong: Cell::new(1),
            weak: Cell::new(1),
            item,
        }
    }

    /// Crée une nouvelle entrée sans initialiser l'objet.
    pub unsafe fn new_unitialised() -> Self {
        Self {
            read: Cell::new(0),
            write: Cell::new(0),
            strong: Cell::new(0),
            weak: Cell::new(0),
            item: std::mem::zeroed()      
        }
    }
}

/// Opérations sur les différents compteurs (Ecriture, Lecture, Forte, Faible)
impl<Item> LineInner<Item> {
    #[inline(always)]
    pub fn strong_ref(&self) -> &Cell<usize> {
        &self.strong
    }

    #[inline(always)]
    pub fn strong(&self) -> usize {
        self.strong_ref().get()
    }

    #[inline(always)]
    pub fn inc_strong(&self) {
        self.strong_ref().set(self.strong() + 1)
    }

    #[inline(always)]
    pub fn dec_strong(&self) {
        self.strong_ref().set(self.strong() - 1)
    }

    #[inline(always)]
    pub fn weak_ref(&self) -> &Cell<usize> {
        &self.weak
    }

    #[inline(always)]
    pub fn weak(&self) -> usize {
        self.weak_ref().get()
    }

    #[inline(always)]
    pub fn inc_weak(&self) {
        self.weak_ref().set(self.weak() + 1)
    }

    #[inline(always)]
    pub fn dec_weak(&self) {
        self.weak_ref().set(self.weak() - 1)
    }

    #[inline(always)]
    pub fn read_ref(&self) -> &Cell<usize> {
        &self.read
    }

    #[inline(always)]
    pub fn read(&self) -> usize {
        self.read_ref().get()
    }

    #[inline(always)]
    pub fn inc_read(&self) {
        self.read_ref().set(self.read() + 1)
    }

    #[inline(always)]
    pub fn dec_read(&self) {
        self.read_ref().set(self.read() - 1)
    }

    #[inline(always)]
    pub fn write_ref(&self) -> &Cell<usize> {
        &self.write
    }

    #[inline(always)]
    pub fn write(&self) -> usize {
        self.write_ref().get()
    }

    #[inline(always)]
    pub fn inc_write(&self) {
        self.write_ref().set(self.write() + 1)
    }

    #[inline(always)]
    pub fn dec_write(&self) {
        self.write_ref().set(self.write() - 1)
    }
}

impl<Item> Drop for LineInner<Item> {
    fn drop(&mut self) {
        // Le compteur n'est jamais à zéro, 
        // sauf si toutes les références fortes ont été détruites.
        if self.weak() == 0 {
            return;
        }

        if self.strong() == 0 {
            drop(self.item);
            self.read.set(0);
            self.write.set(0);
            self.dec_weak();
        }
    }
}

/// Détruit une référence forte.
/// Si le compteur de référence forte est à zéro, détruit l'objet, 
/// et décroît le compteur de référence faible.
/// 
/// # Safety
/// La fonction doit être uniquement appelée lorsque :
/// - Line est détruit.
/// - ReadLine est détruit.
/// - Page est détruit.
pub(super) unsafe fn drop_strong_ref<Item>(this: &mut LineInner<Item>) {
    if this.strong() == 0 {
        return
    }
    
    this.dec_strong();

    if this.strong() == 0 {
        drop(this);
    }
}

/// Détruit une référence faible.
/// Si le compteur de référence faible est à zéro, alors
/// la ligne est considérée comme libre, et peut être réallouée
/// pour un autre usage.
pub(super) unsafe fn drop_weak_ref<Item>(ptr: *mut LineInner<Item>) {
    ptr.as_ref().unwrap().dec_weak();
}

/// Une référence forte et mutable sur une ligne d'une page.
pub struct Line<Item> {
    pub(super) ptr: NonNull<LineInner<Item>>
}

impl<Item> Line<Item> {
    /// Dégrade vers une référence faible sur la ligne.
    pub fn weak_downgrade(this: &Self) -> WeakLine<Item> {
        this.inner().inc_weak();
        WeakLine{ptr: this.ptr}
    }

    /// Transforme la référence mutable, en référence immutable.
    pub fn read_downgrade(this: Self) -> ReadLine<Item> {
        this.inner().dec_write();
        this.inner().inc_read();
        // On évite un appel à Drop sur l'objet.
        let ptr = this.ptr;
        std::mem::forget(this);
        ReadLine { ptr }
    }
}

impl<Item> Deref for Line<Item> {
    type Target = Item;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &mut self.ptr.as_mut().item
        }
    }
}

impl<Item> DerefMut for Line<Item> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut self.ptr.as_mut().item
        }
    }
}

impl<Item> Line<Item> {
    #[inline(always)]
    fn inner(&self) -> &LineInner<Item> {
        unsafe {self.ptr.as_ref()}
    }
}

impl<Item> Drop for Line<Item> {
    fn drop(&mut self) {
        self.inner().dec_write();
        unsafe{drop_strong_ref(self.ptr.as_mut());}
    }
}

/// Une référence forte et immutable sur une ligne d'une page.
pub struct ReadLine<Item> {
    ptr: NonNull<LineInner<Item>>
}

impl<Item> ReadLine<Item> {
    #[inline(always)]
    fn inner(&self) -> &LineInner<Item> {
        unsafe {self.ptr.as_ref()}
    }
}


impl<Item> ReadLine<Item> {
    /// Dégrade vers une référence faible sur la ligne.
    pub fn weak_downgrade(this: &Self) -> WeakLine<Item> {
        this.inner().inc_weak();
        WeakLine{ptr: this.ptr}
    }

    /// Essaye de consumer la référence immutable, pour
    /// produire une référence mutable, si et seulement si
    /// aucun autre emprunt existe.
    pub fn try_write_upgrade(this: Self) -> BookResult<Line<Item>> {
        // On a plus d'une référence immutable, on ne peut rien faire.
        if this.inner().read() > 1 {
            return Err(BookError::AlwaysBorrowed);
        }

        this.inner().dec_read();
        this.inner().inc_write();
        let ptr = this.ptr;
        std::mem::forget(this);
        Ok(Line{ptr})
    }
}


impl<Item> Deref for ReadLine<Item> {
    type Target = Item;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &self.ptr.as_ref().item
        }
    }
}

impl<Item> Drop for ReadLine<Item> {
    fn drop(&mut self) {
        self.inner().dec_read();
        unsafe{drop_strong_ref(self.ptr.as_mut());}
    }
}

/// Référence faible vers une ligne d'une page.
pub struct WeakLine<Item> {
    ptr: NonNull<LineInner<Item>>
}

impl<Item> Clone for WeakLine<Item> {
    fn clone(&self) -> Self {
        self.inner().inc_weak();
        Self { ptr: self.ptr.clone() }
    }
}

impl<Item> WeakLine<Item> {
    /// Essaye d'obtenir une référence mutable vers l'entrée.
    /// 
    /// Retourne:
    /// - None si l'objet a été détruit.
    /// - Err(BookError::AlwaysBorrowed), si une référence immutable existe.
    /// - Err(BookError::AlwaysMutBorrowed), si une référence mutable existe.
    /// - la référence mutable sinon.
    pub fn try_write_upgrade(&self) -> Option<BookResult<Line<Item>>> {
        if self.inner().strong() == 0 {
            return None;
        }

        if self.inner().read() > 0 {
            return Some(Err(BookError::AlwaysBorrowed));
        }

        if self.inner().write() > 0 {
            return Some(Err(BookError::AlwaysMutBorrowed));
        }

        self.inner().inc_write();
        Some(Ok(Line{ptr: self.ptr}))
    }

    /// Essaye d'obtenir une référence immutable vers l'entrée.
    /// 
    /// Retourne:
    /// - None si l'objet a été détruit.
    /// - Err(BookError::AlwaysMutBorrowed), si une référence mutable existe.
    /// - la référence immutable sinon.
    pub fn try_read_upgrade(&self) -> Option<BookResult<ReadLine<Item>>> {
        if self.inner().strong() == 0 {
            return None;
        }

        if self.inner().write() > 0 {
            return Some(Err(BookError::AlwaysMutBorrowed));
        }

        self.inner().inc_write();
        Some(Ok(ReadLine{ptr: self.ptr}))
    }
}


impl<Item> WeakLine<Item> {
    #[inline(always)]
    fn inner(&self) -> &LineInner<Item> {
        unsafe{self.ptr.as_ref()}
    }
}


impl<Item> Drop for WeakLine<Item> {
    fn drop(&mut self) {
        unsafe{drop_weak_ref(self.ptr.as_ptr());}
    }
}